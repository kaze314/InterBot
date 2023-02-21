use crate::memory::mem::{bytecpy,memcpy,patch};
use crate::windowsapi::{c_void, VirtualProtect, VirtualAlloc, GetModuleHandleA, GetProcAddress,
                        DWORD, PAGE_EXECUTE_READWRITE, MEM_COMMIT, MEM_RESERVE, LPCSTR
                    };

/* 
    status: bool for if the hook is enabled

    src: address of orginal function
    dst: the function you you want called
    PtrToGateway: a pointer to the gateway
    len: length of bytes to be replaced

    originalBytes: the bytes that were replaced
*/

#[derive(Default)]
pub struct Hook {
    status: bool,

    src: u64,
    dst: u64,
    Len: usize,

    originalBytes: Vec<u8>
}

impl Hook {
    pub unsafe fn enable(&mut self) -> *const () {
        memcpy(self.originalBytes.as_mut_ptr() as u64, self.src, self.Len as u32);
        self.status = true;
        println!("{:?}", self.originalBytes);
        return TRAMP_HOOK64(self.dst as usize, self.src, self.Len);
    }

    pub unsafe fn disable(&mut self) {
        if self.status {
            patch(self.src as usize, self.originalBytes.as_slice(), self.Len);
            self.status = false;
        }
    }

    pub unsafe fn toggle(&mut self) {
        if !self.status { self.enable(); }
        else { self.disable(); }
    }
}

/*
    ExportName: the function name you want hooked
    ModName: the function's module name
    dst: the function you you want called
    Len: amount of bytes to be replaced
*/
pub unsafe fn start_hook(ExportName: LPCSTR, ModName: LPCSTR, dst: u64, Len: usize) -> Hook {
    let mod_addr = GetModuleHandleA(ModName);
    let func_addr = GetProcAddress(mod_addr, ExportName);


    return Hook {
        status: false,
        src: func_addr,
        dst: dst,
        Len: Len,

        originalBytes: vec![0; Len],
    }
}

/*
    toHook: your function
    ourFunct: function you want to be hooked
    len: length of bytes to be replaced
*/
pub unsafe fn HOOK64(toHook: usize, ourFunct: *mut c_void, len: usize) -> bool {
    if len < 14 { return false; }

    /* change protection */
    let oldprotect: DWORD = 0; 
    VirtualProtect(toHook as *const c_void, len, PAGE_EXECUTE_READWRITE, &oldprotect);

    /* nop original instructions */
    for i in 0..len { 
        *((toHook as u64 + i as u64) as *mut u8) = 0x90;
    }

    /* get the address to jump to */
    let relativeAddress: u64 = ourFunct as u64;
    println!("relativeAddress {:?}", relativeAddress);

    /* place jmp */
    let JMP_BYTES: &[u8] = b"\xFF\x25\x00\x00\x00\x00";
    bytecpy(toHook as u64, JMP_BYTES, 6);

    let bytes: &[u8] = &relativeAddress.to_ne_bytes();
    bytecpy((toHook + 6) as u64, bytes, bytes.len() as u64);

    VirtualProtect(toHook as *const c_void, len, oldprotect, &oldprotect);

    true
}

/*
    dst: your function
    src: function you want to be hooked
    len: length of bytes to be replaced
*/
pub unsafe fn TRAMP_HOOK64(dst: usize, src: u64, len: usize) -> *const () {
    if len < 14 { return 0 as *mut (); }

    /* create gateway */
    let gateway: u64 = VirtualAlloc(0 as *mut c_void, len, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
    println!("gateway address: {}", gateway);
    /* write the stolen bytes */
    memcpy(gateway, src, len as u32);

    /* get the gateway to destination */
    let GATEWAY_RELATIVE_ADDR: u64 = src + len as u64;

    /* place jmp */
    let JMP_BYTES: &[u8] = b"\xFF\x25\x00\x00\x00\x00";
    bytecpy(gateway + len as u64, JMP_BYTES, 6);

    /* place address */
    *((gateway + len as u64 + 6) as *mut u64) = GATEWAY_RELATIVE_ADDR;

    /* place ret */
    *((gateway + len as u64 + 6 + 8) as *mut u64) = 0xC3;

    HOOK64(src as usize, dst as *mut c_void, len);

    return gateway as *mut ();
}