use crate::opengl::glbindings::*;
use crate::util::math::Vector3;
use crate::windowsapi::{CreateFontA, SelectObject, DeleteObject, c_void, HDC, HFONT, HGDIOBJ, 
						FW_MEDIUM, ANSI_CHARSET, OUT_TT_PRECIS, CLIP_DEFAULT_PRECIS, PROOF_QUALITY, FF_DONTCARE, DEFAULT_PITCH};


pub unsafe fn print_gl(x: f32, y: f32, color: [GLubyte; 3], base: u32, text: &[u8]) {
	glColor3ub(color[0], color[1], color[2]);
	glRasterPos2f(x, y);

	glListBase(base - 32);
	glCallLists(text.len() as i32, GL_UNSIGNED_BYTE, text.as_ptr() as *const c_void);
}

pub unsafe fn center_text(x: f32, y: f32, width: f32, height: f32, textWidth: f32, textHeight: f32) -> Vector3 {
	return Vector3 {x: x+(width-textWidth)/2.0, y: y+textHeight, z: 0.0};
}

pub unsafe fn build_font(hdc: HDC, height: i32) -> u32 {
	let base: u32 = glGenLists(96);
	let font: HFONT = CreateFontA(-height, 0, 0, 0, FW_MEDIUM, 0, 0, 0, ANSI_CHARSET, OUT_TT_PRECIS, CLIP_DEFAULT_PRECIS, PROOF_QUALITY, FF_DONTCARE | DEFAULT_PITCH, "Consolas\0".as_ptr() as *const i8);
	let old_font: HFONT = SelectObject(hdc, font as HGDIOBJ) as HFONT;
	wglUseFontBitmapsA(hdc, 32, 96, base);
	SelectObject(hdc, old_font as HGDIOBJ);
	DeleteObject(font as HGDIOBJ);

	return base;

}