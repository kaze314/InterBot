#![cfg(target_pointer_width = "64")]
#![allow(unused_variables)]
#![allow(non_snake_case)]

use crate::sdk::entity::get_player;
use crate::opengl::gldraw::*;
use crate::opengl::glbindings::Red;
use crate::sdk::offsets::{get_entity_list_ptr, get_players_index};
use crate::memory::hook::{start_hook};
use crate::windowsapi::{VK_RBUTTON, VK_DELETE, DWORD, HINSTANCE,
                        GetAsyncKeyState, CloseHandle, CreateThread, c_void, FreeConsole,
                        AllocConsole, GetModuleHandleA, SetConsoleTitleA, FreeLibraryAndExitThread
                    };

mod windowsapi;
mod memory;
mod opengl;
mod sdk;
mod util;

#[no_mangle]
#[allow(unreachable_patterns)]
unsafe extern "C" fn DllMain(hinst_dll: HINSTANCE, fdw_reason: DWORD, _: usize) -> u8{
    static mut THREAD_CREATED: bool = false;

    windowsapi::DisableThreadLibraryCalls(hinst_dll);

    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            if !THREAD_CREATED {
                CloseHandle(CreateThread(std::ptr::null_mut(), 0, Some(main_thread), hinst_dll as *mut c_void, 0, std::ptr::null_mut()));
                THREAD_CREATED = true;
            }
        },
        _ => (),
    };

    return true as u8;
}


static mut OGL_SWAP_BUFFERS: unsafe extern "stdcall" fn (hdc: windowsapi::HDC) -> u8 = our_funct;

unsafe extern "stdcall" fn our_funct(hdc: windowsapi::HDC) -> u8 {
    println!("HOOKED7");

    let originalContext = setup_ortho(hdc);
    draw_filled_rect(300.0, 300.0, 200.0, 200.0, Red);
    restore_gl(hdc, originalContext);
    
    return OGL_SWAP_BUFFERS(hdc);
}

/* Its better to have a smaller Y_SMOOTHING value. */
const X_SMOOTHING: f32 = 17500.0;
const Y_SMOOTHING: f32 = 10000.0;

unsafe extern "C" fn main_thread(lp_thread_parameter: *mut c_void) -> u32 {
    AllocConsole();
    SetConsoleTitleA("Kaze Client 1.0\0".as_ptr() as *const i8);

    println!("**Kaze Client**\n Version 1.0\n");
    println!("[NOTE]: Hold Right Mouse Button to enable aimbot");
    println!("[NOTE]: Press DELETE to stop aimbot\n");

    println!("[NOTE]: X Smoothing - {}", X_SMOOTHING);
    println!("[NOTE]: Y Smoothing - {}", Y_SMOOTHING);

    /* get base address */
    let base_addr = GetModuleHandleA("sauerbraten.exe\0".as_ptr() as *const i8) as isize;

    /* get wglswapbuffers address and hook */
    let mut hook = start_hook("wglSwapBuffers\0".as_ptr() as *const i8, "opengl32.dll\0".as_ptr() as *const i8, our_funct as *const () as u64, 15);

    OGL_SWAP_BUFFERS = std::mem::transmute(hook.enable());

    /* cheat loop */
    loop{
        let players_amount = get_players_index(base_addr);
        let entity_list_ptr = get_entity_list_ptr(base_addr);

        if GetAsyncKeyState(VK_RBUTTON) as u32 & 0x8000 != 0 {
            if players_amount > 1 {
                let local_player = get_player(entity_list_ptr, 0);
                let closest_player = local_player.get_closest_player(entity_list_ptr, players_amount);

                local_player.aim_at(closest_player.pos.to_Vector3(), X_SMOOTHING, Y_SMOOTHING);
            }
        }

        if GetAsyncKeyState(VK_DELETE) as u32 & 0x8000 != 0 {
            hook.toggle();
            break;
        }
    }

    FreeConsole();
    //FreeLibraryAndExitThread(lp_thread_parameter as *mut HINSTANCE, 0);
    return 0;
}
