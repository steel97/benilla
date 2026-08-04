//! A 3d Scene with a button and playing sound.

use std::path::PathBuf;

use benilla::main_shared;
use bevy::prelude::bevy_main;

pub fn transform_path_ios(relative_path: &str) -> PathBuf {
    if let Ok(mut exe_path) = std::env::current_exe() {
        if exe_path.pop() {
            return exe_path.join("wow").join(relative_path);
        }
    }
    PathBuf::from("wow").join(relative_path)
}

#[bevy_main]
pub fn main() {
    unsafe {
        std::env::set_var("WOW_DATA", transform_path_ios("Data").to_str().unwrap());
    }
    main_shared();
}
