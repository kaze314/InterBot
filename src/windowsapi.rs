#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub enum HINSTANCE__ {}
pub type HINSTANCE = *mut HINSTANCE__;
pub type c_char = i8;
pub type c_int  = i32;
pub type CHAR   = c_char;
pub type DWORD  = u32;
pub type BOOL   = i32;
pub type LPCSTR = *const CHAR;
pub type HANDLE = *mut c_void;
pub type HGDIOBJ= *mut c_void;
pub type HDC    = *mut HDC__;
pub type HGLRC  = *mut HGLRC__;
pub type HFONT  = *mut HFONT__;
pub enum c_void  {}
pub enum HDC__   {}
pub enum HGLRC__ {}
pub enum HFONT__ {}


pub const PAGE_EXECUTE_READWRITE: DWORD = 0x40;
pub const MEM_COMMIT:             DWORD = 0x1000;
pub const MEM_RESERVE:            DWORD = 0x2000;

pub const VK_LBUTTON: i32 = 0x01;
pub const VK_RBUTTON: i32 = 0x02;
pub const VK_DELETE:  i32 = 0x2E;
pub const VK_F6:      i32 = 0x75;
pub const VK_NUMLOCK: i32 = 0x90;
pub const VK_HOME:    i32 = 0x24;
pub const VK_INSERT:  i32 = 0x2D;

pub const FW_MEDIUM:      c_int = 500;
pub const ANSI_CHARSET:   DWORD = 0;
pub const OUT_TT_PRECIS:  DWORD = 4;
pub const PROOF_QUALITY:  DWORD = 2;
pub const FF_DONTCARE:    DWORD = 0 << 4;
pub const DEFAULT_PITCH:  DWORD = 0;
pub const CLIP_DEFAULT_PRECIS: DWORD = 0;


#[repr(C)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: DWORD,
    pub lpSecurityDescriptor: *mut c_void,
    pub bInheritHandle: BOOL,
}

type LPSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;

extern "system" {
    pub fn GetModuleHandleA(lpModuleName: LPCSTR) -> HINSTANCE;
    pub fn VirtualProtect(lpaddress: *const c_void, dwsize: usize, flnewprotect: DWORD, lpfloldprotect: &DWORD) -> BOOL;
    pub fn GetAsyncKeyState(vKey: i32) -> i16;
    pub fn GetProcAddress(hModule: HINSTANCE, lpProcName: LPCSTR) -> u64;
    pub fn VirtualAlloc(lpAddress: *mut c_void, dwSize: usize, flAllocationType: DWORD, flProtect: DWORD) -> u64;
    pub fn DisableThreadLibraryCalls(hLibModule: HINSTANCE);
    pub fn FreeLibraryAndExitThread(hLibModule: *mut HINSTANCE, dwExitCode: DWORD);
    pub fn CreateThread(lpThreadAttributes: LPSECURITY_ATTRIBUTES, dwStackSize: usize, lpStartAddress: Option<unsafe extern "C" fn(lpThreadParameter: *mut c_void) -> DWORD>, lpParameter: *mut c_void, dwCreationFlags: DWORD, lpThreadId: *mut DWORD) -> HANDLE;
    pub fn CloseHandle(hObject: HANDLE) -> BOOL;
    pub fn FreeConsole() -> BOOL;
    pub fn AllocConsole() -> BOOL;
    pub fn SetConsoleTitleA(lpConsoleTitle: LPCSTR) -> BOOL;
    pub fn CreateFontA(cHeight: c_int, cWidth: c_int, cEscapement: c_int, cOrientation: c_int, cWeight: c_int, bItalic: DWORD, bUnderline: DWORD, bStrikeOut: DWORD, iCharSet: DWORD, iOutPrecision: DWORD, iClipPrecision: DWORD, iQuality: DWORD, iPitchAndFamily: DWORD, pszFaceName: LPCSTR) -> HFONT;
    pub fn SelectObject(hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
    pub fn DeleteObject(ho: HGDIOBJ) -> BOOL;
}
