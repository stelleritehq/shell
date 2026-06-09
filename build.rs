//! build.rs
//! Required for Slint to build
//! No Args added yet
// TODO: add live reload args
// TODO: setup wgpu w/ opengl

fn main() {
    let config = slint_build::CompilerConfiguration::new().with_library_paths(
        std::collections::HashMap::from([(
            "material".to_string(),
            std::path::Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap())
                .join("crates/md3/material.slint"),
        )]),
    );
    slint_build::compile_with_config("ui/demo.slint", config).unwrap();
}
