#![allow(dead_code)]

use crate::windowsapi::{c_void, VirtualProtect, VirtualAlloc,
                        DWORD, PAGE_EXECUTE_READWRITE, MEM_COMMIT, MEM_RESERVE
                    };



unsafe fn memcpy(dst: u64, src: u64, len: u32) {
    //for every memory address, copy the bytes from src to dst
    for i in 0..len {
        *((dst + i as u64) as *mut u8) = *((src + i as u64) as *mut u8);
    }
}
unsafe fn bytecpy(dst: u64, src: &[u8], len: u64){

    for (i, byte) in src.into_iter().enumerate() {
        *((dst + i as u64) as *mut u8) = *byte;
    }
}

pub fn find_dynamic_addr(ptr: u32, offsets: Vec<u32>) -> u32 {
    let mut addr: u32 = ptr;
    unsafe{
        for i in offsets {
            addr = *(addr as *const u32);
            addr += i;
        }
    }
    addr
}

/*
    toHook: your function
    ourFunct: function you want to be hooked
    len: length of bytes to be replaced
*/
pub unsafe fn HOOK64(toHook: usize, ourFunct: *mut c_void, len: usize) -> bool {
    if len < 14 { return false; }

    //change protection
    let oldprotect: DWORD = 0; 
    VirtualProtect(toHook as *const c_void, len, PAGE_EXECUTE_READWRITE, &oldprotect);

    // nop original instructions
    for i in 0..len {
        *((toHook as u64 + i as u64) as *mut u8) = 0x90;
    }

    // get the address to jump to
    let relativeAddress: u64 = ourFunct as u64;

    //place jmp
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
pub unsafe fn TRAMP_HOOK64(dst: usize, src: u64, len: usize) -> u64 {
    if len < 14 { return 0; }

    //create gateway
    let gateway: u64 = VirtualAlloc(0 as *mut c_void, len, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);

    //write the stolen bytes
    memcpy(gateway, src, len as u32);

    //get the gateway to destination
    let GATEWAY_RELATIVE_ADDR: u64 = src + len as u64;

    //place jmp
    let JMP_BYTES: &[u8] = b"\xFF\x25\x00\x00\x00\x00";
    bytecpy(gateway + len as u64, JMP_BYTES, 6);

    //place address
    *((gateway + len as u64 + 6) as *mut u64) = GATEWAY_RELATIVE_ADDR;

    HOOK64(src as usize, dst as *mut c_void, len);

    return gateway;
}

pub fn patch(dst: usize, src: &[u8], size: usize){
    unsafe{
        let oldprotect: DWORD = 0;
        VirtualProtect(dst as *const c_void, 2, PAGE_EXECUTE_READWRITE, &oldprotect);

        let vector: Vec<u8> = src.to_vec().into();
        for (idx, byte) in vector.into_iter().enumerate() {
            *((dst + idx) as *mut u8) = byte;
        }

        VirtualProtect(dst as *const c_void, 2, oldprotect, &oldprotect);
        
    }
}   



