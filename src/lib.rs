#![deny(warnings)]
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(dead_code))))]
#![doc(test(attr(allow(unused_variables))))]

#![no_std]

#[cfg(target_os="dos")]
use core::arch::asm;
use core::mem::MaybeUninit;
#[cfg(target_os="dos")]
use core::mem::size_of;
use core::num::NonZeroU8;

pub const DOS_ERR_FUNC_NUM_INVALID: u8 = 1;
pub const DOS_ERR_FILE_NOT_FOUND: u8 = 2;
pub const DOS_ERR_PATH_NOT_FOUND: u8 = 3;
pub const DOS_ERR_TOO_MANY_OPEN_FILES: u8 = 4;
pub const DOS_ERR_ACCESS_DENIED: u8 = 5;
pub const DOS_ERR_INVALID_HANDLE: u8 = 6;
pub const DOS_ERR_MCB_DESTROYED: u8 = 7;
pub const DOS_ERR_INSUFFICIENT_MEMORY: u8 = 8;
pub const DOS_ERR_MBA_INVALID: u8 = 9;
pub const DOS_ERR_ENVIRONMENT_INVALID: u8 = 10;
pub const DOS_ERR_FORMAT_INVALID: u8 = 11;
pub const DOS_ERR_ACCESS_CODE_INVALID: u8 = 12;
pub const DOS_ERR_DATA_INVALID: u8 = 13;
pub const DOS_ERR_INVALID_DRIVE: u8 = 15;
pub const DOS_ERR_ATTEMPT_RM_CUR_DIR: u8 = 16;
pub const DOS_ERR_NOT_SAME_DEV: u8 = 17;
pub const DOS_ERR_NO_MORE_TILES: u8 = 18;
pub const DOS_ERR_DISK_WRITE_PROTECTED: u8 = 19;
pub const DOS_ERR_UNKNOWN_UNIT: u8 = 20;
pub const DOS_ERR_DRIVE_NOT_READY: u8 = 21;
pub const DOS_ERR_UNKNOWN_CMD: u8 = 22;
pub const DOS_ERR_DATA_ERROR: u8 = 23;
pub const DOS_ERR_BAD_STRUCT_LEN: u8 = 24;
pub const DOS_ERR_SEEK_ERROR: u8 = 25;
pub const DOS_ERR_UNKNOWN_MEDIA_TYPE: u8 = 26;
pub const DOS_ERR_SECTOR_NOT_FOUND: u8 = 27;
pub const DOS_ERR_PRINTER_NO_PAPER: u8 = 28;
pub const DOS_ERR_WRITE_FAULT: u8 = 29;
pub const DOS_ERR_READ_FAULT: u8 = 30;
pub const DOS_ERR_GENERAL_FAILURE: u8 = 31;
pub const DOS_ERR_SHARING_VIOLATION: u8 = 32;
pub const DOS_ERR_LOCK_VIOLATION: u8 = 33;
pub const DOS_ERR_DISK_CHANGE_INVALID: u8 = 34;
pub const DOS_ERR_FCB_UNAVAILABLE: u8 = 35;
pub const DOS_ERR_SHARING_BUFFER_INVALID: u8 = 36;
pub const DOS_ERR_CODE_PAGE_MISMATCH: u8 = 37;
pub const DOS_ERR_OUT_OF_INPUT: u8 = 38;
pub const DOS_ERR_INSUFFICIENT_DISK_SPACE: u8 = 39;
pub const DOS_ERR_NET_REQUEST_NOT_SUPPORTED: u8 = 50;
pub const DOS_ERR_NET_COMPUTER_NOT_LISTENING: u8 = 51;
pub const DOS_ERR_NET_DUPLICATE_NAME: u8 = 52;
pub const DOS_ERR_NET_NAME_NOT_FOUND_53: u8 = 53;
pub const DOS_ERR_NET_BUSY: u8 = 54;
pub const DOS_ERR_NET_DEV_NO_LONGER_EXISTS: u8 = 55;
pub const DOS_ERR_NET_BIOS_CMD_LIMIT_EXCEEDED: u8 = 56;
pub const DOS_ERR_NET_ADAPTER_HARDWARE_ERROR: u8 = 57;
pub const DOS_ERR_NET_INCORRECT_RESPONSE: u8 = 58;
pub const DOS_ERR_NET_UNEXPECTED_ERROR: u8 = 59;
pub const DOS_ERR_NET_INCOMPATIBLE_ADAPTER: u8 = 60;
pub const DOS_ERR_PRINT_QUEUE_FULL: u8 = 61;
pub const DOS_ERR_QUEUE_NOT_FULL: u8 = 62;
pub const DOS_ERR_NO_SPACE_TO_PRINT_FILE: u8 = 63;
pub const DOS_ERR_NET_NAME_WAS_DELETED: u8 = 64;
pub const DOS_ERR_NET_ACCESS_DENIED: u8 = 65;
pub const DOS_ERR_NET_DEV_TYPE_INCORRECT: u8 = 66;
pub const DOS_ERR_NET_NAME_NOT_FOUND_67: u8 = 67;
pub const DOS_ERR_NET_NAME_LIMIT_EXCEEDED: u8 = 68;
pub const DOS_ERR_NET_BIOS_SESSION_LIMIT_EXCEEDED: u8 = 69;
pub const DOS_ERR_TEMPORARILY_PAUSED: u8 = 70;
pub const DOS_ERR_NET_REQUEST_NOT_ACCEPTED: u8 = 71;
pub const DOS_ERR_NET_REDIRECTION_PAUSED: u8 = 72;
pub const DOS_ERR_NET_INVALID_VERSION: u8 = 73;
pub const DOS_ERR_ACCOUNT_EXPIRED: u8 = 74;
pub const DOS_ERR_PASSWORD_EXPIRED: u8 = 75;
pub const DOS_ERR_LOGIN_ATTEMPTED_INVALID: u8 = 76;
pub const DOS_ERR_NET_DISK_LIMIT_EXCEED: u8 = 77;
pub const DOS_ERR_NET_NOT_LOGGED: u8 = 78;
pub const DOS_ERR_FILE_EXISTS: u8 = 80;
pub const DOS_ERR_CANNOT_MAKE_DIR: u8 = 82;
pub const DOS_ERR_FAIL_ON_INT_24H: u8 = 83;
pub const DOS_ERR_TOO_MANY_REDIRECTIONS: u8 = 84;
pub const DOS_ERR_DUPLICATE_REDIRECTION: u8 = 85;
pub const DOS_ERR_INVALID_PASSWORD: u8 = 86;
pub const DOS_ERR_INVALID_PARAMETER: u8 = 87;
pub const DOS_ERR_NET_WRITE_FAULT: u8 = 88;
pub const DOS_ERR_NET_FUNC_NOT_SUPPORTED: u8 = 89;
pub const DOS_ERR_SYS_COMPONENT_NOT_INSTALLED: u8 = 90;

#[cfg(not(target_os="dos"))]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub fn int_21h_ah_4Ch_exit(al_exit_code: u8) {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[allow(non_snake_case)]
#[inline]
pub fn int_21h_ah_4Ch_exit(al_exit_code: u8) {
    unsafe {
        asm!(
            "int 0x21",
            in("ax") 0x4C00u16 | al_exit_code as u16,
        );
    }
}

#[derive(Debug, Clone)]
pub struct DosVer {
    pub ah_minor: u8,
    pub al_major: u8,
}

#[cfg(not(target_os="dos"))]
pub fn int_21h_ah_30h_dos_ver() -> DosVer {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[inline]
pub fn int_21h_ah_30h_dos_ver() -> DosVer {
    let mut ax: u16;
    unsafe {
        asm!(
            "int 0x21",
            in("ax") 0x3000u16,
            lateout("ax") ax,
            lateout("cx") _,
            lateout("bx") _,
        );
    }
    DosVer { ah_minor: (ax >> 8) as u8, al_major: ax as u8 }
}

#[derive(Debug, Clone)]
pub struct CodePage {
    pub bx_active: u16,
    pub dx_default: u16,
}

#[derive(Debug, Clone)]
pub struct AxErr {
    pub ax_err: u16,
}

#[cfg(target_os="dos")]
const CF: u8 = 0x01;

#[cfg(target_os="dos")]
const ZF: u8 = 0x40;

#[cfg(not(target_os="dos"))]
pub fn int_21h_ax_6601h_code_page() -> Result<CodePage, AxErr> {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[inline]
pub fn int_21h_ax_6601h_code_page() -> Result<CodePage, AxErr> {
    let mut bx_active: u16;
    let mut dx_default: u16;
    let mut flags: u16;
    let mut ax_err: u16;
    unsafe {
        asm!(
            "int 0x21",
            "mov {ax_err:x}, ax",
            "lahf",
            ax_err = lateout(reg) ax_err,
            in("ax") 0x6601u16,
            lateout("ax") flags,
            lateout("bx") bx_active,
            lateout("dx") dx_default,
        );
    }
    if ((flags >> 8) as u8) & CF == 0 {
        Ok(CodePage { bx_active, dx_default })
    } else {
        Err(AxErr { ax_err })
    }
}

#[cfg(target_os="dos")]
#[inline]
fn p32<T>(p: *const T) -> u32 {
    assert!(size_of::<*const T>() == size_of::<u32>());
    p as usize as u32
}

#[derive(Debug, Clone)]
pub struct AlLastCh {
    pub al_last_ch: u8,
}

#[cfg(not(target_os="dos"))]
#[allow(unused_variables)]
pub fn int_21h_ah_02h_out_ch(dl_ch: u8) -> AlLastCh {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[inline]
pub fn int_21h_ah_02h_out_ch(dl_ch: u8) -> AlLastCh {
    let mut ax: u16;
    unsafe {
        asm!(
            "int 0x21",
            in("ax") 0x0200u16,
            in("dx") dl_ch as u16,
            lateout("ax") ax,
        );
    }
    AlLastCh { al_last_ch: ax as u8 }
}

#[cfg(not(target_os="dos"))]
#[allow(unused_variables)]
pub fn int_21h_ah_09h_out_str(dx_str_24h: *const u8) {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[inline]
pub fn int_21h_ah_09h_out_str(dx_str_24h: *const u8) {
    unsafe {
        asm!(
            "int 0x21",
            in("ax") 0x0900u16,
            in("edx") p32(dx_str_24h),
            lateout("ax") _,
        );
    }
}

#[derive(Debug, Clone)]
pub struct AxHandle {
    pub ax_handle: u16,
}

#[cfg(not(target_os="dos"))]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub fn int_21h_ah_3Dh_open(dx_path_z: *const u8, al_mode: u8) -> Result<AxHandle, AxErr> {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[allow(non_snake_case)]
#[inline]
pub fn int_21h_ah_3Dh_open(dx_path_z: *const u8, al_mode: u8) -> Result<AxHandle, AxErr> {
    let mut ax: u16;
    let mut flags: u16;
    unsafe {
        asm!(
            "int 0x21",
            "mov {ax:x}, ax",
            "lahf",
            ax = lateout(reg) ax,
            in("ax") 0x3d00u16 | al_mode as u16,
            in("edx") p32(dx_path_z),
            lateout("ax") flags,
        );
    }
    if ((flags >> 8) as u8) & CF == 0 {
        Ok(AxHandle { ax_handle: ax })
    } else {
        Err(AxErr { ax_err: ax })
    }
}

#[derive(Debug, Clone)]
pub struct AxRead {
    pub ax_read: u16,
}

#[cfg(not(target_os="dos"))]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub fn int_21h_ah_3Fh_read(bx_handle: u16, dx_cx_buf: &mut [MaybeUninit<u8>]) -> Result<AxRead, AxErr> {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[allow(non_snake_case)]
#[inline]
pub fn int_21h_ah_3Fh_read(bx_handle: u16, dx_cx_buf: &mut [MaybeUninit<u8>]) -> Result<AxRead, AxErr> {
    let mut flags: u16;
    let mut ax: u16;
    unsafe {
        asm!(
            "int 0x21",
            "mov {ax:x}, ax",
            "lahf",
            ax = lateout(reg) ax,
            in("ax") 0x3F00u16,
            in("bx") bx_handle,
            in("ecx") u16::try_from(dx_cx_buf.len()).unwrap() as u32,
            in("edx") p32(dx_cx_buf.as_mut_ptr()),
            lateout("ax") flags
        );
    }
    if ((flags >> 8) as u8) & CF == 0 {
        Ok(AxRead { ax_read: ax })
    } else {
        Err(AxErr { ax_err: ax })
    }
}

#[derive(Debug, Clone)]
pub struct AxWritten {
    pub ax_written: u16,
}

#[cfg(not(target_os="dos"))]
#[allow(unused_variables)]
pub fn int_21h_ah_40h_write(bx_handle: u16, dx_cx_buf: &[u8]) -> Result<AxWritten, AxErr> {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[inline]
pub fn int_21h_ah_40h_write(bx_handle: u16, dx_cx_buf: &[u8]) -> Result<AxWritten, AxErr> {
    let mut flags: u16;
    let mut ax: u16;
    unsafe {
        asm!(
            "int 0x21",
            "mov {ax:x}, ax",
            "lahf",
            ax = lateout(reg) ax,
            in("ax") 0x4000u16,
            in("bx") bx_handle,
            in("ecx") u16::try_from(dx_cx_buf.len()).unwrap() as u32,
            in("edx") p32(dx_cx_buf.as_ptr()),
            lateout("ax") flags
        );
    }
    if ((flags >> 8) as u8) & CF == 0 {
        Ok(AxWritten { ax_written: ax })
    } else {
        Err(AxErr { ax_err: ax })
    }
}

#[derive(Debug, Clone)]
pub struct AxSegment {
    pub ax_segment: u16,
}

#[derive(Debug, Clone)]
pub struct AllocErr {
    pub ax_err: u16,
    pub bx_available_paragraphs: u16,
}

#[cfg(not(target_os="dos"))]
#[allow(unused_variables)]
pub fn int_21h_ah_48h_alloc(bx_paragraphs: u16) -> Result<AxSegment, AllocErr> {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[inline]
pub fn int_21h_ah_48h_alloc(bx_paragraphs: u16) -> Result<AxSegment, AllocErr> {
    let mut ebx_paragraphs = bx_paragraphs as u32;
    let mut ax: u16;
    let mut flags: u16;
    unsafe {
        asm!(
            "int 0x21",
            "mov {ax:x}, ax",
            "lahf",
            ax = lateout(reg) ax,
            in("ax") 0x4800u16,
            inlateout("ebx") ebx_paragraphs => ebx_paragraphs,
            lateout("ax") flags,
        );
    }
    if ((flags >> 8) as u8) & CF == 0 {
        Ok(AxSegment { ax_segment: ax })
    } else {
        Err(AllocErr { ax_err: ax, bx_available_paragraphs: ebx_paragraphs as u16 })
    }
}

#[derive(Debug, Clone)]
pub struct BxSegment {
    pub bx_segment: u16,
}

#[cfg(not(target_os="dos"))]
pub fn int_21h_ah_62h_psp_addr() -> BxSegment {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[inline]
pub fn int_21h_ah_62h_psp_addr() -> BxSegment {
    let mut bx_segment: u16;
    unsafe {
        asm!(
            "int 0x21",
            in("ax") 0x6200u16,
            lateout("bx") bx_segment,
        );
    }
    BxSegment { bx_segment }
}

#[derive(Debug, Clone)]
pub struct AlChar {
    pub al_char: u8,
}

#[cfg(not(target_os="dos"))]
#[allow(non_snake_case)]
pub fn int_21h_ah_06h_dl_FFh_inkey() -> Option<AlChar> {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[allow(non_snake_case)]
#[inline]
pub fn int_21h_ah_06h_dl_FFh_inkey() -> Option<AlChar> {
    let mut ax: u16;
    let mut flags: u16;
    unsafe {
        asm!(
            "int 0x21",
            "mov {ax:x}, ax",
            "lahf",
            ax = lateout(reg) ax,
            in("ax") 0x0600u16,
            in("dx") 0x00FFu16,
            lateout("ax") flags,
        );
    }
    if ((flags >> 8) as u8) & ZF == 0 {
        Some(AlChar { al_char: ax as u8 })
    } else {
        None
    }
}

#[derive(Debug, Clone)]
pub struct CxDxAddr {
    pub cx_dx_addr: u32,
}

#[cfg(not(target_os="dos"))]
#[allow(unused_variables)]
pub fn int_31h_ax_0006h_segment_addr(bx_selector: u16) -> Result<CxDxAddr, AxErr> {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[inline]
pub fn int_31h_ax_0006h_segment_addr(bx_selector: u16) -> Result<CxDxAddr, AxErr> {
    let mut flags: u16;
    let mut ax_err: u16;
    let mut cx: u16;
    let mut dx: u16;
    unsafe {
        asm!(
            "int 0x31",
            "mov {ax_err:x}, ax",
            "lahf",
            ax_err = lateout(reg) ax_err,
            in("ax") 0x0006u16,
            in("bx") bx_selector,
            lateout("ax") flags,
            lateout("cx") cx,
            lateout("dx") dx,
        );
    }
    if ((flags >> 8) as u8) & CF == 0 {
        Ok(CxDxAddr { cx_dx_addr: ((cx as u32) << 16) | (dx as u32) })
    } else {
        Err(AxErr { ax_err })
    }
}

#[derive(Debug, Clone)]
pub struct RmAlloc {
    pub ax_segment: u16,
    pub dx_selector: u16,
}

#[cfg(not(target_os="dos"))]
#[allow(unused_variables)]
pub fn int_31h_ax_0100h_rm_alloc(bx_paragraphs: u16) -> Result<RmAlloc, AllocErr> {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[inline]
pub fn int_31h_ax_0100h_rm_alloc(mut bx_paragraphs: u16) -> Result<RmAlloc, AllocErr> {
    let mut flags: u16;
    let mut ax: u16;
    let mut dx_selector: u16;
    unsafe {
        asm!(
            "int 0x31",
            "mov {ax:x}, ax",
            "lahf",
            ax = lateout(reg) ax,
            in("ax") 0x0100u16,
            inlateout("bx") bx_paragraphs => bx_paragraphs,
            lateout("ax") flags,
            lateout("dx") dx_selector,
        );
    }
    if ((flags >> 8) as u8) & CF == 0 {
        Ok(RmAlloc { ax_segment: ax, dx_selector })
    } else {
        Err(AllocErr { ax_err: ax, bx_available_paragraphs: bx_paragraphs })
    }
}

#[cfg(not(target_os="dos"))]
#[allow(unused_variables)]
pub fn int_31h_ax_0101h_rm_free(dx_selector: u16) -> Result<(), AxErr> {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[inline]
pub fn int_31h_ax_0101h_rm_free(dx_selector: u16) -> Result<(), AxErr> {
    let mut flags: u16;
    let mut ax_err: u16;
    unsafe {
        asm!(
            "int 0x31",
            "mov {ax_err:x}, ax",
            "lahf",
            ax_err = lateout(reg) ax_err,
            in("ax") 0x0101u16,
            in("dx") dx_selector,
            lateout("ax") flags,
        );
    }
    if ((flags >> 8) as u8) & CF == 0 {
        Ok(())
    } else {
        Err(AxErr { ax_err })
    }
}

#[derive(Debug, Clone)]
pub struct AlErr {
    pub al_err: NonZeroU8,
}

#[cfg(not(target_os="dos"))]
#[allow(unused_variables)]
pub fn int_10h_ah_00h_set_video_mode(al_mode: u8) -> Result<(), AlErr> {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[inline]
pub fn int_10h_ah_00h_set_video_mode(al_mode: u8) -> Result<(), AlErr> {
    let mut al_err: u16;
    unsafe {
        asm!(
            "int 0x10",
            in("ax") al_mode as u16,
            lateout("ax") al_err,
        );
    }
    NonZeroU8::new(al_err as u8).map(|al_err| AlErr { al_err }).map_or(Ok(()), Err)
}

#[derive(Debug, Clone)]
pub struct VideoMode {
    pub al_mode: u8,
    pub ah_cols: u8,
    pub bh_active_page: u8,
}

#[cfg(not(target_os="dos"))]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub fn int_10h_ah_0Fh_video_mode() -> VideoMode {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[allow(non_snake_case)]
#[inline]
pub fn int_10h_ah_0Fh_video_mode() -> VideoMode {
    let mut ax: u16;
    let mut bx_active_page: u16;
    unsafe {
        asm!(
            "int 0x10",
            in("ax") 0x0F00u16,
            lateout("ax") ax,
            lateout("bx") bx_active_page,
        );
    }
    VideoMode {
        al_mode: ax as u8,
        ah_cols: (ax >> 8) as u8,
        bh_active_page: (bx_active_page >> 8) as u8,
    }
}

pub struct IntHandler {
    pub ebx_int_handler: u32,
}

#[cfg(not(target_os="dos"))]
#[allow(unused_variables)]
pub fn int_21h_ah_35h_get_int(al_int_vec: u8) -> IntHandler {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[inline]
pub fn int_21h_ah_35h_get_int(al_int_vec: u8) -> IntHandler {
    let mut ebx_int_handler: u32;
    unsafe {
        asm!(
            "int 0x21",
            in("ax") 0x3500u16 | al_int_vec as u16,
            lateout("ebx") ebx_int_handler,
        );
    }
    IntHandler { ebx_int_handler }
}

#[cfg(not(target_os="dos"))]
#[allow(unused_variables)]
pub fn int_21h_ah_25h_set_int(al_int_vec: u8, edx_int_handler: *mut u8) {
    panic!("cfg(target_os=\"dos\")");
}

#[cfg(target_os="dos")]
#[inline]
pub fn int_21h_ah_25h_set_int(al_int_vec: u8, edx_int_handler: *mut u8) {
    unsafe {
        asm!(
            "int 0x21",
            in("ax") 0x2500u16 | al_int_vec as u16,
            in("edx") p32(edx_int_handler),
        );
    }
}
