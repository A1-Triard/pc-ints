#![feature(start)]

#![deny(warnings)]

#![no_std]

#[cfg(windows)]
#[link(name="msvcrt")]
extern { }

#[cfg(windows)]
fn exit(code: u8) -> ! {
    unsafe { winapi::um::processthreadsapi::ExitProcess(
        code as winapi::shared::minwindef::UINT
    ); }
    #[allow(clippy::empty_loop)]
    loop { }
}

#[cfg(not(windows))]
fn exit(code: u8) -> ! {
    unsafe { libc::exit(code as u16 as i16 as libc::c_int) }
}

#[panic_handler]
extern fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit(99)
}

use pc_ints::DOS_ERR_FILE_NOT_FOUND;

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    assert_eq!(DOS_ERR_FILE_NOT_FOUND, 2);
    0
}
