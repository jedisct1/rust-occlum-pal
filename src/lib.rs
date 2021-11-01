//! Occlum SGX enclaves management.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(dead_code)]

mod sys;
use sys::*;

use std::ffi::CString;
use std::fmt;
use std::os::raw::{c_char, c_int};
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::pin::Pin;
use std::ptr;
use std::sync::{Arc, Mutex};

/// Occlum-PAL error codes.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    VersionError,
    InitError,
    CreateError,
    ExecError,
    SignalError,
    ArgumentsError,
    CStringError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::VersionError => write!(f, "PAL API version mismatch"),
            Error::InitError => write!(f, "Initialization error"),
            Error::CreateError => write!(f, "Process creation error"),
            Error::ExecError => write!(f, "Process execution error"),
            Error::SignalError => write!(f, "Process signaling error"),
            Error::ArgumentsError => write!(f, "Arguments list error"),
            Error::CStringError => write!(f, "String contains a bare \\0 character"),
        }
    }
}

impl std::error::Error for Error {}

/// Log level.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Trace,
}

impl LogLevel {
    fn as_bytes(&self) -> &'static [u8] {
        match *self {
            LogLevel::Off => b"off",
            LogLevel::Error => b"error",
            LogLevel::Warn => b"warn",
            LogLevel::Info => b"info",
            LogLevel::Trace => b"trace",
        }
    }
}

/// Enclave configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    instance_dir: CString,
    log_level: LogLevel,
}

impl Config {
    /// Create a new enclave configuration.
    /// `instance_dir` specifies the path of an Occlum instance directory, which is usually created with the
    /// `occlum new` command. The default value is "."; that is, the current working directory
    /// is the Occlum instance directory.
    /// `log_level` specifies the log level of the enclave. The default value is `LogLevel::Off`.
    pub fn new(instance_dir: impl ToString, log_level: Option<LogLevel>) -> Result<Self, Error> {
        let log_level = log_level.unwrap_or(LogLevel::Off);
        Ok(Self {
            instance_dir: CString::new(instance_dir.to_string().as_bytes())
                .map_err(|_| Error::CStringError)?,
            log_level,
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new(".", None).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stdio {
    stdin: c_int,
    stdout: c_int,
    stderr: c_int,
}

impl Default for Stdio {
    fn default() -> Self {
        Self {
            stdin: 0,
            stdout: 1,
            stderr: 2,
        }
    }
}

impl Stdio {
    /// Redirect the process standard descriptors to existing descriptors.
    pub fn new(stdin: impl AsRawFd, stdout: impl AsRawFd, stderr: impl AsRawFd) -> Self {
        Self {
            stdin: stdin.as_raw_fd() as _,
            stdout: stdout.as_raw_fd() as _,
            stderr: stderr.as_raw_fd() as _,
        }
    }

    fn to_api(&self) -> Pin<Box<occlum_stdio_fds>> {
        Box::pin(occlum_stdio_fds {
            stdin_fd: self.stdin,
            stdout_fd: self.stdout,
            stderr_fd: self.stderr,
        })
    }
}

#[derive(Debug)]
pub struct CStringsVec {
    cstrings: Vec<CString>,
    ptrs: Vec<*const c_char>,
}

impl CStringsVec {
    fn new(cstrings: Vec<CString>) -> Result<Self, Error> {
        let mut ptrs = Vec::with_capacity(cstrings.len() + 1);
        for cstring in &cstrings {
            ptrs.push(cstring.as_ptr());
        }
        ptrs.push(ptr::null());
        Ok(Self { cstrings, ptrs })
    }

    fn as_mut_ptr(&mut self) -> *mut *const c_char {
        self.ptrs.as_mut_ptr()
    }
}

#[derive(Debug)]
struct ProcessApiConcrete {
    path: CString,
    argv_cstrings_vec: CStringsVec,
    env_cstrings_vec: CStringsVec,
    stdio_api: Pin<Box<occlum_stdio_fds>>,
    pid: Pin<Box<c_int>>,
}

#[derive(Debug)]
struct ProcessApi {
    concrete: ProcessApiConcrete,
    ptrs: Pin<Box<occlum_pal_create_process_args>>,
}

/// A process to be run in an enclave.
#[derive(Debug)]
pub struct Process {
    enclave: Arc<Mutex<Enclave>>,
    process_api: ProcessApi,
}

impl Process {
    /// Return the process identifier.
    pub fn process_id(&self) -> ProcessId {
        ProcessId {
            pid: unsafe { *self.process_api.ptrs.pid },
        }
    }

    /// Execute the process inside the Occlum enclave
    pub fn exec(&self) -> Result<ExitCode, Error> {
        let mut exit_code: c_int = -1;
        let mut exec_args = Box::pin(occlum_pal_exec_args {
            pid: self.process_id().pid,
            exit_value: &mut exit_code,
        });
        let exec_result = unsafe { occlum_pal_exec(&mut *exec_args) };
        if exec_result == 0 {
            Ok(exit_code)
        } else {
            Err(Error::CreateError)
        }
    }
}

/// A process identifier, that can be used to send signals to the process.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProcessId {
    pid: c_int,
}

impl ProcessId {
    /// Kill the process.
    pub fn kill(self) -> Result<(), Error> {
        if unsafe { occlum_pal_kill(self.pid, 9) } == 0 {
            Ok(())
        } else {
            Err(Error::SignalError)
        }
    }

    /// Send SIGTERM to the process.
    pub fn terminate(self) -> Result<(), Error> {
        if unsafe { occlum_pal_kill(self.pid, 16) } == 0 {
            Ok(())
        } else {
            Err(Error::SignalError)
        }
    }
}

pub type ExitCode = c_int;

/// A process running in an enclave.
#[derive(Debug, PartialEq, Eq)]
pub struct ProcessBuilder {
    path: CString,
    argv: Vec<CString>,
    env: Vec<CString>,
    stdio: Option<Stdio>,
}

impl ProcessBuilder {
    /// Create a new process.
    /// `path` is the path of the executable to run.
    /// `argv` is the list of arguments to pass to the executable. If not `None`, the first argument must be the application name, as seen by the application itself.
    /// `env` is the list of environment variables to pass to the application.
    /// `stdio` is the standard I/O descriptors to use for the process.
    pub fn new(
        path: impl ToString,
        argv: Option<&[String]>,
        env: Option<&[String]>,
        stdio: Option<Stdio>,
    ) -> Result<Self, Error> where {
        let path = path.to_string();
        let path_cstring = CString::new(path.as_bytes()).map_err(|_| Error::CStringError)?;
        let argv = match argv {
            None => {
                let prog = Path::new(&path).file_name().unwrap_or_default();
                let prog_cstring = CString::new(prog.to_string_lossy().as_bytes())
                    .map_err(|_| Error::CStringError)?;
                vec![prog_cstring]
            }
            Some(argv) => {
                let mut y = vec![];
                for x in argv {
                    y.push(CString::new(x.to_string().as_bytes()).map_err(|_| Error::CStringError)?)
                }
                y
            }
        };
        if argv.is_empty() {
            return Err(Error::ArgumentsError);
        }

        let env = env.unwrap_or(&[]);
        let env = {
            let mut y = vec![];
            for x in env {
                y.push(CString::new(x.to_string().as_bytes()).map_err(|_| Error::CStringError)?)
            }
            y
        };

        Ok(Self {
            path: path_cstring,
            argv,
            env,
            stdio,
        })
    }

    /// Create a new process to be run in the enclave.
    pub fn build(self, enclave: &Arc<Mutex<Enclave>>) -> Result<Process, Error> {
        let stdio_api = match self.stdio {
            None => Stdio::default().to_api(),
            Some(stdio) => stdio.to_api(),
        };
        let mut process_api_concrete = ProcessApiConcrete {
            path: self.path,
            argv_cstrings_vec: CStringsVec::new(self.argv)?,
            env_cstrings_vec: CStringsVec::new(self.env)?,
            stdio_api,
            pid: Box::pin(-1),
        };
        let mut process_api_ptrs = Box::pin(occlum_pal_create_process_args {
            path: process_api_concrete.path.as_ptr(),
            argv: process_api_concrete.argv_cstrings_vec.as_mut_ptr(),
            env: process_api_concrete.env_cstrings_vec.as_mut_ptr(),
            stdio: &*process_api_concrete.stdio_api,
            pid: &mut *process_api_concrete.pid,
        });
        if unsafe { occlum_pal_create_process(&mut *process_api_ptrs) } != 0 {
            return Err(Error::CreateError);
        }
        Ok(Process {
            enclave: enclave.clone(),
            process_api: ProcessApi {
                concrete: process_api_concrete,
                ptrs: process_api_ptrs,
            },
        })
    }
}

#[derive(Debug)]
struct ConfigApiConcrete {
    instance_dir: CString,
    log_level: CString,
}

#[derive(Debug)]
struct ConfigApi {
    concrete: ConfigApiConcrete,
    ptrs: Pin<Box<occlum_pal_attr>>,
}

/// An Occlum SGX enclave.
#[derive(Debug)]
pub struct Enclave {
    config: Config,
    config_api: ConfigApi,
}

impl Enclave {
    /// Create a new SGX enclave with the given configuration.
    pub fn new(config: Config) -> Result<Arc<Mutex<Self>>, Error> {
        if (unsafe { occlum_pal_get_version() } <= 0) {
            return Err(Error::VersionError);
        }

        let config_api_concrete = ConfigApiConcrete {
            instance_dir: CString::new(config.instance_dir.as_bytes())
                .map_err(|_| Error::CStringError)?,
            log_level: CString::new(config.log_level.as_bytes())
                .map_err(|_| Error::CStringError)?,
        };
        let config_api_ptrs = Box::pin(occlum_pal_attr {
            instance_dir: config_api_concrete.instance_dir.as_ptr() as *const c_char,
            log_level: config_api_concrete.log_level.as_bytes().as_ptr() as *const c_char,
        });
        if unsafe { occlum_pal_init(&*config_api_ptrs) } != 0 {
            return Err(Error::InitError);
        }
        Ok(Arc::new(Mutex::new(Enclave {
            config,
            config_api: ConfigApi {
                concrete: config_api_concrete,
                ptrs: config_api_ptrs,
            },
        })))
    }

    fn destroy(&mut self) {
        unsafe {
            occlum_pal_destroy();
        }
    }
}

impl Drop for Enclave {
    fn drop(&mut self) {
        self.destroy();
    }
}
