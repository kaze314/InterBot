
pub fn toggle(isOn: bool) -> bool {
	if isOn {
		return false;
	} else {
		return true;
	}
}

pub fn display_banner() {
    println!("**Kaze Client**\n Version 1.5\n");
    println!("[NOTE]: Hold Right Mouse Button to enable aimbot");
    println!("[NOTE]: Press DELETE to stop aimbot\n");
}