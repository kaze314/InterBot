
const PLAYER_INDEX: isize = 0x346C90+0xC as isize;
const ENTITY_LIST: isize = 0x346C90 as isize;

pub unsafe fn get_players_index(base_addr: isize) -> u32 {
	*((base_addr+PLAYER_INDEX) as *mut u32)
}
pub unsafe fn get_entity_list_ptr(base_addr: isize) -> *mut u64 {
	(base_addr + ENTITY_LIST) as *mut u64
}
