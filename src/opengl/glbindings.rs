#![allow(dead_code)]

use crate::windowsapi::{c_void, BOOL, HGLRC, HDC, DWORD};

pub const GL_VIEWPORT:      GLenum = 0x0BA2;
pub const GL_PROJECTION:    GLenum = 0x1701;
pub const GL_MODELVIEW:     GLenum = 0x1700;
pub const GL_DEPTH_TEST:    GLenum = 0x0B71;
pub const GL_QUADS:         u32    = 0x0007;
pub const GL_LINE_STRIP:    u32    = 0x0003;
pub const GL_UNSIGNED_BYTE: u32    = 0x1401;

pub type GLbitfield = u32;
pub const GL_ALL_ATTRIB_BITS: GLbitfield = 0x000fffff;

pub type GLint    = i32;
pub type GLvoid   = c_void;
pub type GLuint   = u32;
pub type GLsizei  = i32;
pub type GLdouble = f64;
pub type GLfloat  = f32;
pub type GLubyte  = u8;
pub type GLenum   = u32;

pub const RED:      [GLubyte; 3]   = [255,0,0];
pub const GREEN:    [GLubyte; 3]   = [0,255,0];
pub const BLUE:     [GLubyte; 3]   = [0,0,255];
pub const TURQUOISE:[GLubyte; 3]   = [64,224,208];
pub const GREY:     [GLubyte; 3]   = [192,192,192];
pub const BLACK:    [GLubyte; 3]   = [0,0,0];

extern "system" {
	pub fn glGetIntegerv(pname: GLenum, params: *mut GLint);
	pub fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
	pub fn glMatrixMode(mode: GLenum);
	pub fn glLoadIdentity();
	pub fn glOrtho(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble);
	pub fn glDisable(cap: GLenum);
	pub fn glColor3ub(red: GLubyte, green: GLubyte, blue: GLubyte);
	pub fn glBegin(mode: GLenum);
	pub fn glEnd();
	pub fn glVertex2f(x: GLfloat, y: GLfloat);
	pub fn wglCreateContext(a1: HDC) -> HGLRC;
	pub fn wglMakeCurrent(a1: HDC, a2: HGLRC) -> BOOL;
	pub fn wglGetCurrentContext() -> HGLRC;
	pub fn glLineWidth(width: GLfloat);
	pub fn glGenLists(range: GLsizei) -> GLuint;
	pub fn wglUseFontBitmapsA(hdc: HDC, first: DWORD, count: DWORD, listBase: DWORD) -> BOOL;
	pub fn wglGetCurrentDC() -> HDC;
	pub fn glRasterPos2f(x: GLfloat, y: GLfloat);
	pub fn glCallLists(n: GLsizei, gltype: GLenum, lists: *const GLvoid);
	pub fn glListBase(base: GLuint);
}