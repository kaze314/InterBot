
fn main() {
    // Needed for winapi::GetKeyState
    println!("cargo:rustc-link-lib=dylib=user32");

    println!("cargo:rustc-link-lib=dylib=Gdi32");

    println!("cargo:rustc-link-lib=dylib=opengl32");
}