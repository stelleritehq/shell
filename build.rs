//! build.rs
//! Required for Slint to build
//! No Args added yet 
// TODO: add md3 and live reload args
// TODO: setup wgpu w/ opengl

fn main() {
    slint_build::compile("ui/app.slint").unwrap();
}
