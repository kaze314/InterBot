
use crate::util::math::{PVector3, Vector3, PI};

pub struct Entity {
	pub pos: PVector3,
	pub angle: PVector3,
	pub team: [u8; 4],
	pub name: [u8; 16],
	pub name_len: usize,
	pub is_dead: u8,
	pub health: u32,
}

impl Entity {
	pub unsafe fn aim_at(&self, pos: Vector3, x_smoothing: f32, y_smoothing: f32, time: u32) {
		let local_player_pos: Vector3 = self.pos.to_Vector3();
		let deltaVec = Vector3{x: (pos.x - local_player_pos.x), y: (pos.y - local_player_pos.y), z: (pos.z - local_player_pos.z)};
		let deltaVecLength = ((deltaVec.x*deltaVec.x) + (deltaVec.y*deltaVec.y) + (deltaVec.z*deltaVec.z)).sqrt();

		let pitch: f32 = (deltaVec.z / deltaVecLength).asin() * (180.0/PI);
		let mut yaw: f32 =  -(deltaVec.x).atan2(deltaVec.y) * (180.0/PI);

    	if yaw < 0.0 {
    		yaw = yaw+360.0;
    	}

    	let mut time = time;
    	if time > 5000 {
    		time = 5000;
    	}

    	println!("angle.x {:?}", *(self.angle.x));
    	println!("pitch {:?}", pitch);

    	//TODO: add noise
		*(self.angle.y) = *(self.angle.y) + (pitch - *(self.angle.y))/ y_smoothing* ( 1.0 + (time as f32/30.0));
		*(self.angle.x) = *(self.angle.x) + (yaw - *(self.angle.x))/ x_smoothing* ( 1.0 + (time as f32/35.0));
	}
	pub unsafe fn aim_at_distance(&self, pos: Vector3) -> f32 {
		let local_player_pos: Vector3 = self.pos.to_Vector3();
		let local_player_angle: Vector3 = self.angle.to_Vector3();

		let deltaVec = Vector3{x: (pos.x - local_player_pos.x), y: (pos.y - local_player_pos.y), z: (pos.z - local_player_pos.z)};
		let deltaVecLength = ((deltaVec.x*deltaVec.x) + (deltaVec.y*deltaVec.y) + (deltaVec.z*deltaVec.z)).sqrt();

		let pitch: f32 = (deltaVec.z / deltaVecLength).asin() * (180.0/PI);
		let mut yaw: f32 =  -(deltaVec.x).atan2(deltaVec.y) * (180.0/PI);

    	if yaw < 0.0 {
    		yaw = yaw+360.0;
    	}

    	let delta_yaw = yaw - local_player_angle.x;
    	let delta_pitch = pitch - local_player_angle.y;

    	if yaw 
    	(delta_yaw*delta_yaw + delta_pitch*delta_pitch).sqrt()
	}
	pub unsafe fn get_closest_player(&self, entity_list_ptr: *mut u64, players_amount: u32) -> Entity {
		let mut closest_player: Entity = get_player(entity_list_ptr, 0);
	    let mut closest_distance: f32 = 10000.0;

	    for i in 1..players_amount {
	        let player = get_player(entity_list_ptr, i);
	        let distance = self.aim_at_distance(player.pos.to_Vector3());

	        if distance < closest_distance && player.is_dead == 0 {
	            closest_player = player;
	            closest_distance = distance;
	        }
	    }
	    closest_player
	}
}

/* Constucts and returns the Entity struct */
pub unsafe fn get_player(entity_list_ptr: *mut u64, index: u32) -> Entity{
	let player = *((*entity_list_ptr + (index as u64 * 0x08)) as *mut u64);

	let player_postition = PVector3 {
									x: (player + 0x30) as *mut f32,
									y: (player + 0x34) as *mut f32,
									z: (player + 0x38) as *mut f32,
								};
	let player_angle = PVector3 {
								 x: (player + 0x3C) as *mut f32,
								 y: (player + 0x40) as *mut f32,
								 z: (player + 0x44) as *mut f32,
	                        };
	
	let player_team = *((player + 0x378) as *mut [u8; 4]);
	let player_name = *((player + 0x274) as *mut [u8; 16]);
	
	/* Gets the length of the name */
	let mut name_len: usize = 0;
	for i in 0..16 {
		if player_name[i] == 0 { break }
		name_len += 1;
	}

	let is_dead: u8 = *((player + 0x77) as *mut u8);
	let health: u32 = *((player + 0x178) as *mut u32);

	let player_entity = Entity{pos: player_postition, angle: player_angle, team: player_team, name: player_name, name_len: name_len, is_dead: is_dead, health: health};
	player_entity
}
