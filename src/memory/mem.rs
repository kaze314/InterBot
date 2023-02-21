#![allow(dead_code)]

use crate::windowsapi::{c_void, VirtualProtect,
                        DWORD, PAGE_EXECUTE_READWRITE, 
                    };


pub unsafe fn memcpy(dst: u64, src: u64, len: u32) {
    /* for every memory address, copy the bytes from src to dst */
    for i in 0..len {
        *((dst + i as u64) as *mut u8) = *((src + i as u64) as *mut u8);
    }
}
pub unsafe fn bytecpy(dst: u64, src: &[u8], len: u64){

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
