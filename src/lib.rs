#![cfg(target_pointer_width = "64")]
#![allow(unused_variables)]
#![allow(non_snake_case)]

use crate::opengl::{gldraw::*,
                   esp::draw_esp,
                   gltext::*,
                   glbindings::{wglMakeCurrent, wglGetCurrentContext, GREEN},
               };

use crate::sdk::{entity::get_player,
                 offsets::{get_entity_list_ptr, get_players_index, get_spectator_amount}
             };

use crate::memory::hook::{start_hook};
use crate::windowsapi::{VK_RBUTTON, VK_DELETE, VK_INSERT, VK_HOME, DWORD, HINSTANCE, HGLRC,
                        GetAsyncKeyState, CloseHandle, CreateThread, c_void, FreeConsole,
                        AllocConsole, GetModuleHandleA, SetConsoleTitleA
                    };

use std::time::Instant;
use crate::sdk::entity::Entity;
use crate::util::util::{toggle, display_banner};

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
static mut NEW_CONTEXT: Option<HGLRC> = None;
static mut ESP_ON: bool = true;

unsafe extern "stdcall" fn our_funct(hdc: windowsapi::HDC) -> u8 {
    /* save the original context */
    let original_context = wglGetCurrentContext();

    /* if we have not created a context, create one. */
    if NEW_CONTEXT.is_none() {
        NEW_CONTEXT = Some(setup_ortho(hdc));
    }
    
    wglMakeCurrent(hdc, NEW_CONTEXT.unwrap());

    if ESP_ON {
        /* esp code */
        let base_addr = GetModuleHandleA("sauerbraten.exe\0".as_ptr() as *const i8) as isize;
        let matrix: [f32; 16] = *((base_addr as u64 + 0x32D040) as *const [f32; 16]);

        let mut font = build_font(hdc, 20);

        draw_esp(matrix, base_addr, font, 20);

        /* print amount of spectators */
        let mut message: Vec<u8> = "Spectator amount: ".as_bytes().to_vec();
        let spectator_amount = get_spectator_amount(base_addr).to_string();
        let spectator_amount_bytes = spectator_amount.as_bytes();

        for i in spectator_amount_bytes {
            message.push(*i);
        }

        font = build_font(hdc, 35);
        print_gl(50.0, 600.0, GREEN, font, &message);
    }
    

    /* revert to original context and call orginal wglswapbuffers */
    wglMakeCurrent(hdc, original_context);

    return OGL_SWAP_BUFFERS(hdc);
}

/* Its better to have a smaller Y_SMOOTHING value. */
const X_SMOOTHING: f32 = 170000.0;
const Y_SMOOTHING: f32 = 110000.0;

unsafe extern "C" fn main_thread(lp_thread_parameter: *mut c_void) -> u32 {
    AllocConsole();
    SetConsoleTitleA("Kaze Client 1.0\0".as_ptr() as *const i8);
    display_banner();

    println!("[NOTE]: X Smoothing - {}", X_SMOOTHING);
    println!("[NOTE]: Y Smoothing - {}", Y_SMOOTHING);

    /* get base address */
    let base_addr = GetModuleHandleA("sauerbraten.exe\0".as_ptr() as *const i8) as isize;

    /* get wglswapbuffers address and hook */
    let mut hook = start_hook("wglSwapBuffers\0".as_ptr() as *const i8, "opengl32.dll\0".as_ptr() as *const i8, our_funct as *const () as u64, 15);

    OGL_SWAP_BUFFERS = std::mem::transmute(hook.enable());

    let mut now = Instant::now();
    let mut toggle_aimbot: bool = true;
    let mut insert_was_pressed: bool = false;
    let mut home_was_pressed: bool = false;
    let mut current_enemy: Entity;

    /* cheat loop */
    loop{
        let players_amount = get_players_index(base_addr);
        let entity_list_ptr = get_entity_list_ptr(base_addr);

        if GetAsyncKeyState(VK_RBUTTON) as u32 & 0x8000 != 0  && toggle_aimbot {
            if players_amount > 1 {
                let local_player = get_player(entity_list_ptr, 0);
                let closest_player = local_player.get_closest_player(entity_list_ptr, players_amount);

                
                local_player.aim_at(closest_player.pos.to_Vector3(), X_SMOOTHING, Y_SMOOTHING, now.elapsed().as_millis() as u32);
                
            }
        } else {
            now = Instant::now();
        }

        /* toggle aimbot */
        if GetAsyncKeyState(VK_INSERT) as u32 & 0x8000 != 0 {
            if !insert_was_pressed {
                toggle_aimbot = toggle(toggle_aimbot);
            }
            insert_was_pressed = true;
        } else {
            insert_was_pressed = false;
        }

        /* toggle esp */
        if GetAsyncKeyState(VK_HOME) as u32 & 0x8000 != 0 {
            if !home_was_pressed {
                ESP_ON = toggle(ESP_ON);
            }
            home_was_pressed = true;
        } else {
            home_was_pressed = false;
        }

        if GetAsyncKeyState(VK_DELETE) as u32 & 0x8000 != 0 {
            hook.toggle();
            break;
        }
    }

    FreeConsole();
    return 0;
}

