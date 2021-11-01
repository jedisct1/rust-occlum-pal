extern "C" {
    pub fn occlum_pal_get_version() -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct occlum_pal_attr {
    pub instance_dir: *const ::std::os::raw::c_char,
    pub log_level: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_occlum_pal_attr() {
    assert_eq!(
        ::std::mem::size_of::<occlum_pal_attr>(),
        16usize,
        concat!("Size of: ", stringify!(occlum_pal_attr))
    );
    assert_eq!(
        ::std::mem::align_of::<occlum_pal_attr>(),
        8usize,
        concat!("Alignment of ", stringify!(occlum_pal_attr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<occlum_pal_attr>())).instance_dir as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(occlum_pal_attr),
            "::",
            stringify!(instance_dir)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<occlum_pal_attr>())).log_level as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(occlum_pal_attr),
            "::",
            stringify!(log_level)
        )
    );
}
impl Default for occlum_pal_attr {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type occlum_pal_attr_t = occlum_pal_attr;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct occlum_stdio_fds {
    pub stdin_fd: ::std::os::raw::c_int,
    pub stdout_fd: ::std::os::raw::c_int,
    pub stderr_fd: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_occlum_stdio_fds() {
    assert_eq!(
        ::std::mem::size_of::<occlum_stdio_fds>(),
        12usize,
        concat!("Size of: ", stringify!(occlum_stdio_fds))
    );
    assert_eq!(
        ::std::mem::align_of::<occlum_stdio_fds>(),
        4usize,
        concat!("Alignment of ", stringify!(occlum_stdio_fds))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<occlum_stdio_fds>())).stdin_fd as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(occlum_stdio_fds),
            "::",
            stringify!(stdin_fd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<occlum_stdio_fds>())).stdout_fd as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(occlum_stdio_fds),
            "::",
            stringify!(stdout_fd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<occlum_stdio_fds>())).stderr_fd as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(occlum_stdio_fds),
            "::",
            stringify!(stderr_fd)
        )
    );
}
pub type occlum_stdio_fds_t = occlum_stdio_fds;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct occlum_pal_create_process_args {
    pub path: *const ::std::os::raw::c_char,
    pub argv: *mut *const ::std::os::raw::c_char,
    pub env: *mut *const ::std::os::raw::c_char,
    pub stdio: *const occlum_stdio_fds,
    pub pid: *mut ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_occlum_pal_create_process_args() {
    assert_eq!(
        ::std::mem::size_of::<occlum_pal_create_process_args>(),
        40usize,
        concat!("Size of: ", stringify!(occlum_pal_create_process_args))
    );
    assert_eq!(
        ::std::mem::align_of::<occlum_pal_create_process_args>(),
        8usize,
        concat!("Alignment of ", stringify!(occlum_pal_create_process_args))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<occlum_pal_create_process_args>())).path as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(occlum_pal_create_process_args),
            "::",
            stringify!(path)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<occlum_pal_create_process_args>())).argv as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(occlum_pal_create_process_args),
            "::",
            stringify!(argv)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<occlum_pal_create_process_args>())).env as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(occlum_pal_create_process_args),
            "::",
            stringify!(env)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<occlum_pal_create_process_args>())).stdio as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(occlum_pal_create_process_args),
            "::",
            stringify!(stdio)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<occlum_pal_create_process_args>())).pid as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(occlum_pal_create_process_args),
            "::",
            stringify!(pid)
        )
    );
}
impl Default for occlum_pal_create_process_args {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct occlum_pal_exec_args {
    pub pid: ::std::os::raw::c_int,
    pub exit_value: *mut ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_occlum_pal_exec_args() {
    assert_eq!(
        ::std::mem::size_of::<occlum_pal_exec_args>(),
        16usize,
        concat!("Size of: ", stringify!(occlum_pal_exec_args))
    );
    assert_eq!(
        ::std::mem::align_of::<occlum_pal_exec_args>(),
        8usize,
        concat!("Alignment of ", stringify!(occlum_pal_exec_args))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<occlum_pal_exec_args>())).pid as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(occlum_pal_exec_args),
            "::",
            stringify!(pid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<occlum_pal_exec_args>())).exit_value as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(occlum_pal_exec_args),
            "::",
            stringify!(exit_value)
        )
    );
}
impl Default for occlum_pal_exec_args {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub fn occlum_pal_init(attr: *const occlum_pal_attr) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn occlum_pal_create_process(
        args: *mut occlum_pal_create_process_args,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn occlum_pal_exec(args: *mut occlum_pal_exec_args) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn occlum_pal_kill(
        pid: ::std::os::raw::c_int,
        sig: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn occlum_pal_destroy() -> ::std::os::raw::c_int;
}
