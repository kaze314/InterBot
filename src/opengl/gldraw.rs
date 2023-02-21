use crate::opengl::glbindings::*;
use crate::windowsapi::{GetModuleHandleA, HDC, HGLRC};

const MOD_NAME: *const i8 = "opengl32.dll\0".as_ptr() as *const i8;

pub unsafe fn setup_ortho(hdc: HDC) -> HGLRC {
	let mod_addr = GetModuleHandleA(MOD_NAME);
	let originalContext: HGLRC = wglGetCurrentContext(mod_addr);

	let myRenderingContext = wglCreateContext(hdc, mod_addr);
	wglMakeCurrent(hdc, myRenderingContext, mod_addr);

	let mut viewport: [GLint; 4] = [0; 4];
	glGetIntegerv(GL_VIEWPORT, viewport.as_mut_ptr(), mod_addr);
	glViewport(0, 0, viewport[2], viewport[3], mod_addr);

	glMatrixMode(GL_PROJECTION, mod_addr);
	glLoadIdentity(mod_addr);
	glOrtho(0.0, viewport[2] as f64, viewport[3] as f64, 0.0, -1.0, 1.0, mod_addr);
	glMatrixMode(GL_MODELVIEW, mod_addr);
	glLoadIdentity(mod_addr);
	glDisable(GL_DEPTH_TEST, mod_addr);

	return originalContext;
}

pub unsafe fn restore_gl(hdc: HDC, originalContext: HGLRC) {
	let mod_addr = GetModuleHandleA(MOD_NAME);
	wglMakeCurrent(hdc, originalContext, mod_addr);
}

pub unsafe fn draw_filled_rect(x: GLfloat, y: GLfloat, width: GLfloat, height: GLfloat, color: [GLubyte; 3] ) {
	let mod_addr = GetModuleHandleA(MOD_NAME);
	glColor3ub(color[0], color[1], color[2], mod_addr);
	glBegin(GL_QUADS, mod_addr);
	glVertex2f(x,y, mod_addr);
	glVertex2f(x+width, y, mod_addr);
	glVertex2f(x+height, y, mod_addr);
	glVertex2f(x+width, y+height, mod_addr);
	glVertex2f(x, y+height, mod_addr);
	glEnd(mod_addr);
}