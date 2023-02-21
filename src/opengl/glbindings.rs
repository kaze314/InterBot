use std::mem::transmute;
use crate::windowsapi::{GetProcAddress, HINSTANCE, BOOL, HGLRC, HDC};

pub const GL_VIEWPORT:   GLenum = 0x0BA2;
pub const GL_PROJECTION: GLenum = 0x1701;
pub const GL_MODELVIEW:  GLenum = 0x1700;
pub const GL_DEPTH_TEST: GLenum = 0x0B71;
pub const GL_QUADS:      u32    = 0x0007;
pub const GL_LINE_LOOP:  u32    = 0x0002;

pub type GLbitfield = u32;
pub const GL_ALL_ATTRIB_BITS: GLbitfield = 0x000fffff;

pub type GLint    = i32;
pub type GLsizei  = i32;
pub type GLdouble = f64;
pub type GLfloat  = f32;
pub type GLubyte  = u8;
pub type GLenum   = u32;

pub const Red:   [GLubyte; 3]   = [255,0,0];
pub const Green: [GLubyte; 3]   = [0,255,0];
pub const Grey:  [GLubyte; 3]   = [192,192,192];
pub const Black: [GLubyte; 3]   = [0,0,0];

pub unsafe fn glPushAttrib(mask: GLbitfield, mod_addr: HINSTANCE) {
    let func_addr = GetProcAddress(mod_addr, "glPushAttrib\0".as_ptr() as *const i8);

	let oglPushAttrib: unsafe extern "stdcall" fn (mask: GLbitfield) = transmute(func_addr);
	oglPushAttrib(mask);
}

pub unsafe fn glPushMatrix(mod_addr: HINSTANCE) {
    let func_addr = GetProcAddress(mod_addr, "glPushMatrix\0".as_ptr() as *const i8);

	let oglPushMatrix: unsafe extern "stdcall" fn () = transmute(func_addr);
	oglPushMatrix();
}

pub unsafe fn glGetIntegerv(pname: GLenum, params: *mut GLint, mod_addr: HINSTANCE) {
    let func_addr = GetProcAddress(mod_addr, "glGetIntegerv\0".as_ptr() as *const i8);

	let oglGetIntegerv: unsafe extern "stdcall" fn (pname: GLenum, params: *mut GLint) = transmute(func_addr);
	oglGetIntegerv(pname, params);
}

pub unsafe fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei, mod_addr: HINSTANCE) {
    let func_addr = GetProcAddress(mod_addr, "glViewport\0".as_ptr() as *const i8);

	let oglViewport: unsafe extern "stdcall" fn (x: GLint, y: GLint, width: GLsizei, height: GLsizei) = transmute(func_addr);
	oglViewport(x, y, width, height);
}

pub unsafe fn glMatrixMode(mode: GLenum, mod_addr: HINSTANCE) {
    let func_addr = GetProcAddress(mod_addr, "glMatrixMode\0".as_ptr() as *const i8);

	let oglMatrixMode: unsafe extern "stdcall" fn (mode: GLenum) = transmute(func_addr);
	oglMatrixMode(mode);
}

pub unsafe fn glLoadIdentity(mod_addr: HINSTANCE) {
    let func_addr = GetProcAddress(mod_addr, "glLoadIdentity\0".as_ptr() as *const i8);

	let oglLoadIdentity: unsafe extern "stdcall" fn () = transmute(func_addr);
	oglLoadIdentity();
}

pub unsafe fn glOrtho(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble, mod_addr: HINSTANCE) {
    let func_addr = GetProcAddress(mod_addr, "glOrtho\0".as_ptr() as *const i8);

	let oglOrtho: unsafe extern "stdcall" fn (left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble) = transmute(func_addr);
	oglOrtho(left, right, bottom, top, zNear, zFar);
}

pub unsafe fn glDisable(cap: GLenum, mod_addr: HINSTANCE) {
    let func_addr = GetProcAddress(mod_addr, "glDisable\0".as_ptr() as *const i8);

	let oglDisable: unsafe extern "stdcall" fn (cap: GLenum) = transmute(func_addr);
	oglDisable(cap);
}

pub unsafe fn glPopMatrix(mod_addr: HINSTANCE) {
    let func_addr = GetProcAddress(mod_addr, "glPopMatrix\0".as_ptr() as *const i8);

	let oglPopMatrix: unsafe extern "stdcall" fn () = transmute(func_addr);
	oglPopMatrix();
}

pub unsafe fn glPopAttrib(mod_addr: HINSTANCE) {
    let func_addr = GetProcAddress(mod_addr, "glPopAttrib\0".as_ptr() as *const i8);

	let oglPopAttrib: unsafe extern "stdcall" fn () = transmute(func_addr);
	oglPopAttrib();
}

pub unsafe fn glColor3ub(red: GLubyte, green: GLubyte, blue: GLubyte, mod_addr: HINSTANCE) {
    let func_addr = GetProcAddress(mod_addr, "glColor3ub\0".as_ptr() as *const i8);

	let oglColor3ub: unsafe extern "stdcall" fn (red: GLubyte, green: GLubyte, blue: GLubyte) = transmute(func_addr);
	oglColor3ub(red, green, blue);
}

pub unsafe fn glBegin(mode: GLenum, mod_addr: HINSTANCE) {
    let func_addr = GetProcAddress(mod_addr, "glBegin\0".as_ptr() as *const i8);

	let oglBegin: unsafe extern "stdcall" fn (mode: GLenum) = transmute(func_addr);
	oglBegin(mode);
}

pub unsafe fn glEnd(mod_addr: HINSTANCE) {
    let func_addr = GetProcAddress(mod_addr, "glEnd\0".as_ptr() as *const i8);

	let oglEnd: unsafe extern "stdcall" fn () = transmute(func_addr);
	oglEnd();
}

pub unsafe fn glVertex2f(x: GLfloat, y: GLfloat, mod_addr: HINSTANCE) {
    let func_addr = GetProcAddress(mod_addr, "glVertex2f\0".as_ptr() as *const i8);

	let oglVertex2f: unsafe extern "stdcall" fn (x: GLfloat, y: GLfloat) = transmute(func_addr);
	oglVertex2f(x, y);
}

pub unsafe fn wglCreateContext(a1: HDC, mod_addr: HINSTANCE) -> HGLRC {
    let func_addr = GetProcAddress(mod_addr, "wglCreateContext\0".as_ptr() as *const i8);

	let owglCreateContext: unsafe extern "stdcall" fn (a1: HDC) -> HGLRC = transmute(func_addr);
	return owglCreateContext(a1);
}

pub unsafe fn wglMakeCurrent(a1: HDC, a2: HGLRC, mod_addr: HINSTANCE) -> BOOL {
    let func_addr = GetProcAddress(mod_addr, "wglMakeCurrent\0".as_ptr() as *const i8);

	let owglMakeCurrent: unsafe extern "stdcall" fn (a1: HDC, a2: HGLRC) -> BOOL = transmute(func_addr);
	return owglMakeCurrent(a1, a2);
}

pub unsafe fn wglGetCurrentContext(mod_addr: HINSTANCE) -> HGLRC {
    let func_addr = GetProcAddress(mod_addr, "wglGetCurrentContext\0".as_ptr() as *const i8);

	let owglGetCurrentContext: unsafe extern "stdcall" fn () -> HGLRC = transmute(func_addr);
	return owglGetCurrentContext();
}