use crate::util::math::{Vector3, Vector4};

use crate::opengl::glbindings::*;
use crate::opengl::gldraw::{draw_outline, draw_filled_rect};
use crate::opengl::gltext::*;

use crate::sdk::entity::get_player;
use crate::sdk::offsets::{get_players_index, get_entity_list_ptr};

const PLAYER_MAX_HEALTH: u32 = 100;
const PLAYER_HEIGHT: i32 = 14;
const PLAYER_WIDTH: i32 = 5;
const PLAYER_ASPECT_RATIO: f32 = (PLAYER_HEIGHT / PLAYER_WIDTH) as f32;

pub unsafe fn draw_esp(matrix: [f32; 16], base_addr: isize, font: u32, textHeight: i32) {
	let mut viewport: [GLint; 4] = [0; 4];
	glGetIntegerv(GL_VIEWPORT, viewport.as_mut_ptr());

	let players_amount = get_players_index(base_addr);
    let entity_list_ptr = get_entity_list_ptr(base_addr);

    if players_amount > 1 {

        for i in 1..players_amount {
        	let local_player = get_player(entity_list_ptr, 0);
        	let entity = get_player(entity_list_ptr, i);

        	let center = entity.pos.to_Vector3();
        	let mut foot = entity.pos.to_Vector3();
        	foot.z = foot.z - 14.0;

        	let mut head_coords: Vector3 = Vector3 {x: 0.0, y: 0.0, z:0.0};
        	let mut foot_coords: Vector3 = Vector3 {x: 0.0, y: 0.0, z:0.0};

        	/* If the entitys are valid, store the postion of the head on the screen and the postion of the feet on screen. */
        	if entity.is_dead == 0 && world_to_screen(center, &mut head_coords, matrix, viewport[2], viewport[3]) && world_to_screen(foot, &mut foot_coords, matrix, viewport[2], viewport[3]) {
				
				let height: f32 = head_coords.y - foot_coords.y;
				let width: f32 = height/PLAYER_ASPECT_RATIO;
				let x = head_coords.x - width/2.0;
				let y = head_coords.y-height;

				match std::str::from_utf8(&entity.team) {
					Ok(t) => {

						/* Set color depending on team */
						let mut color: [u8; 3] = RED;
						if t == std::str::from_utf8(&local_player.team).unwrap() {
							color = TURQUOISE;
						}

						/* player outline */
						draw_outline(x, y, width, height, 2.0, color);

						/* player health bar */
						draw_filled_rect(x + width, y, width/15.0, height, RED);
						let health_height: f32 = height * (entity.health as f32/PLAYER_MAX_HEALTH as f32) ;
						draw_filled_rect(x + width, y, width/15.0, health_height, GREEN);

						/* print team */
						let mut dest = center_text(x, y, width, height, (t.len() * (textHeight/2) as usize) as f32, textHeight as f32);
						print_gl(dest.x, dest.y, color, font, &entity.team);

						/* print name */
						dest = center_text(x, y, width, height, (entity.name_len * (textHeight/2) as usize) as f32, textHeight as f32);
						print_gl(dest.x, dest.y + textHeight as f32, color, font, &entity.name[..entity.name_len]);

					},
					Err(_) => draw_outline(x, y, width, height, 2.0, RED), /* If the name is invalid then just draw a box */
				}
        	}
        }
    }
}

pub unsafe fn world_to_screen(pos: Vector3, screen: *mut Vector3, matrix: [f32; 16], windowWidth: i32, windowHeight: i32) -> bool {
	/* Matrix-vector Product, multiplying world(eye) coordinates by projection matrix = clipCoords */
	let mut clipCoords: Vector4 = Vector4 {x: 0.0, y: 0.0, z: 0.0, w: 0.0};

	clipCoords.x = pos.x * matrix[0] + pos.y * matrix[4] + pos.z * matrix[8] + matrix[12];
	clipCoords.y = pos.x * matrix[1] + pos.y * matrix[5] + pos.z * matrix[9] + matrix[13];
	clipCoords.z = pos.x * matrix[2] + pos.y * matrix[6] + pos.z * matrix[10] + matrix[14];
	clipCoords.w = pos.x * matrix[3] + pos.y * matrix[7] + pos.z * matrix[11] + matrix[15];

	if clipCoords.w < 0.1 { return false; }

	/* perspective division, dividing by clip.W = Normalized Device Coordinates */
	let mut NDC: Vector3 = Vector3 {x: 0.0, y: 0.0, z: 0.0};
	NDC.x = clipCoords.x / clipCoords.w;
	NDC.y = clipCoords.y / clipCoords.w;
	NDC.z = clipCoords.z / clipCoords.w;

	/* Transform to window coordinates */
	(*(screen)).x = (windowWidth as f32 / 2.0 * NDC.x as f32) + (NDC.x as f32 + windowWidth as f32 / 2.0);
	(*(screen)).y = -(windowHeight as f32 / 2.0 * NDC.y as f32) + (NDC.y as f32 + windowHeight as f32 / 2.0);

	true
}
