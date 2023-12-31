use core::ffi::{c_char, c_int};

use crate::utils::CStrRef;

/// The entrypoint of the program.
/// This is called by a bit of assembly handling architecture-specific _start.
pub(crate) unsafe extern "C" fn start(rsp: u64) -> ! {
    let argc = (rsp as *const u64).read();
    let argv = (rsp + 8) as *const *const c_char;
    let envp = (8 + 8 * argc + rsp + 8) as *mut Option<CStrRef<'static>>;

    crate::env::init(envp);

    extern "C" {
        fn main(argc: c_int, argv: *const *const c_char) -> c_int;
    }

    // crate::env::debug_env();

    let result = main(argc as i32, argv);

    sys_exit(result as u64);
}

pub fn sys_exit(code: u64) -> ! {
    unsafe {
        crate::sys::syscall::syscall!(crate::sys::syscall::SYS_EXIT, code);
        crate::sys::helpers::trap!();
        core::hint::unreachable_unchecked()
    }
}

pub fn abort() -> ! {
    // FIXME: we actually need to do signal shenanigans
    unsafe {
        crate::sys::syscall::syscall!(crate::sys::syscall::SYS_EXIT, 1);
        crate::sys::helpers::trap!();
        core::hint::unreachable_unchecked()
    }
}
