
const PLAYER_INDEX: isize = 0x346C90+0xC as isize;
const ENTITY_LIST: isize = 0x346C90 as isize;
const SPECTATOR_AMOUNT: isize = 0x346FDC;

pub unsafe fn get_players_index(base_addr: isize) -> u32 {
	*((base_addr+PLAYER_INDEX) as *mut u32)
}
pub unsafe fn get_entity_list_ptr(base_addr: isize) -> *mut u64 {
	(base_addr + ENTITY_LIST) as *mut u64
}
pub unsafe fn get_spectator_amount(base_addr: isize) -> u8 {
	*((base_addr + SPECTATOR_AMOUNT) as *mut u8)
}
