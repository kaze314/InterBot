#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub enum HINSTANCE__ {}
pub type HINSTANCE = *mut HINSTANCE__;
pub type c_char = i8;
pub type CHAR = c_char;
pub type DWORD = u32;
pub type BOOL = i32;
pub type LPCSTR = *const CHAR;
pub type HANDLE = *mut c_void;
pub enum c_void {}
pub enum HDC {}

pub const PAGE_EXECUTE_READWRITE: DWORD = 0x40;
pub const MEM_COMMIT: DWORD = 0x1000;
pub const MEM_RESERVE: DWORD = 0x2000;

pub const VK_LBUTTON: i32 = 0x01;
pub const VK_RBUTTON: i32 = 0x02;
pub const VK_DELETE: i32 = 0x2E;
pub const VK_F6: i32 = 0x75;
pub const VK_NUMLOCK: i32 = 0x90;

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
}
