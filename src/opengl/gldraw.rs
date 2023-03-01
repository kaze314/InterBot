#![allow(dead_code)]
use crate::opengl::glbindings::*;
use crate::windowsapi::{HDC, HGLRC};

/* create and return a context with the required settings for drawing 2D */
pub unsafe fn setup_ortho(hdc: HDC) -> HGLRC {

	let myRenderingContext = wglCreateContext(hdc);
	wglMakeCurrent(hdc, myRenderingContext);

	let mut viewport: [GLint; 4] = [0; 4];
	glGetIntegerv(GL_VIEWPORT, viewport.as_mut_ptr());
	glViewport(0, 0, viewport[2], viewport[3]);

	glMatrixMode(GL_PROJECTION);
	glLoadIdentity();
	glOrtho(0.0, viewport[2] as f64, viewport[3] as f64, 0.0, -1.0, 1.0);
	glMatrixMode(GL_MODELVIEW);
	glLoadIdentity();
	glDisable(GL_DEPTH_TEST);

	return myRenderingContext;
}

pub unsafe fn draw_filled_rect(x: GLfloat, y: GLfloat, width: GLfloat, height: GLfloat, color: [GLubyte; 3]) {
	glColor3ub(color[0], color[1], color[2]);
	glBegin(GL_QUADS);

	glVertex2f(x,y);
	glVertex2f(x+width, y);
	glVertex2f(x+width, y+height);
	glVertex2f(x, y+height);

	glEnd();
}

pub unsafe fn draw_outline(x: GLfloat, y: GLfloat, width: GLfloat, height: GLfloat, lineWidth: GLfloat, color: [GLubyte; 3]) {

	glLineWidth(lineWidth);
	glBegin(GL_LINE_STRIP);
	glColor3ub(color[0], color[1], color[2]);

	glVertex2f(x - 0.5, y - 0.5);
	glVertex2f(x + width + 0.5, y - 0.5);
	glVertex2f(x + width + 0.5, y + height + 0.5);
	glVertex2f(x - 0.5, y + height - 0.5);
	glVertex2f(x - 0.5, y - 0.5);

	glEnd();
}