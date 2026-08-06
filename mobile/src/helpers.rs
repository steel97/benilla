#[cfg(target_os = "ios")]
#[unsafe(no_mangle)]
pub extern "C" fn get_dim_ios() -> *mut std::os::raw::c_char {
    use objc2_foundation::MainThreadMarker;
    use objc2_ui_kit::UIScreen;
    use std::ffi::CString;

    if let Some(mtm) = MainThreadMarker::new() {
        let main_screen = UIScreen::mainScreen(mtm);

        /*let size_str = if let Some(current_mode) = main_screen.currentMode() {
            let size = current_mode.size();
            format!("{}x{}", size.height as u32, size.width as u32)
        } else {*/
        let bounds = main_screen.bounds();
        //let scale = main_screen.scale();
        let size_str = format!(
            "{}x{}",
            (bounds.size.height) as u32,
            (bounds.size.width) as u32
        );
        //};

        let c_str = CString::new(size_str).unwrap_or_else(|_| CString::new("0x0").unwrap());
        c_str.into_raw()
    } else {
        panic!("failed to take screen resolution");
    }
}

#[cfg(target_os = "ios")]
#[unsafe(no_mangle)]
pub extern "C" fn free_dim_ios(ptr: *mut std::os::raw::c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = std::ffi::CString::from_raw(ptr);
        }
    }
}

pub fn postprocess_env() {
    // set full screen resolution on ios
    #[cfg(target_os = "ios")]
    {
        use std::ffi::CStr;

        unsafe {
            let ptr = get_dim_ios();

            if !ptr.is_null() {
                let c_str = CStr::from_ptr(ptr);
                let size_string = c_str.to_string_lossy().into_owned();
                println!("size {}", size_string);
                std::env::set_var("WOW_WIN", size_string);
            }

            free_dim_ios(ptr);
        }
    }
}
