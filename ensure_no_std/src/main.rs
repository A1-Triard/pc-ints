#![feature(start)]

#![deny(warnings)]

#![no_std]

#[cfg(windows)]
#[link(name="msvcrt")]
extern { }

#[panic_handler]
extern fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit_no_std::exit(99)
}

use pc_ints::DOS_ERR_FILE_NOT_FOUND;

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    assert_eq!(DOS_ERR_FILE_NOT_FOUND, 2);
    0
}
