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

#[cfg(target_os = "android")]
use bevy::android::ANDROID_APP;
#[cfg(target_os = "android")]
use jni::objects::{JObject, JValue};
#[cfg(target_os = "android")]
use jni::{Env, JavaVM, errors::Result, jni_sig, jni_str};
#[cfg(target_os = "android")]
pub fn get_android_screen_size() -> Result<(i32, i32)> {
    let android_app = ANDROID_APP
        .get()
        .expect("ANDROID_APP is not set. Did you forget to use the #[bevy_main] macro?");
    let vm = unsafe { jni::JavaVM::from_raw(android_app.vm_as_ptr() as *mut _) };

    vm.attach_current_thread(|env| {
        let activity = unsafe { JObject::from_raw(env, android_app.activity_as_ptr() as *mut _) };
        let window_manager = env
            .call_method(
                &activity,
                jni_str!("getWindowManager"),
                jni_sig!("()Landroid/view/WindowManager;"),
                &[],
            )
            .expect("Failed to call getWindowManager")
            .l()
            .expect("WindowManager is null");

        let window_metrics = env
            .call_method(
                &window_manager,
                jni_str!("getCurrentWindowMetrics"),
                jni_sig!("()Landroid/view/WindowMetrics;"),
                &[],
            )
            .expect("Failed to call getCurrentWindowMetrics")
            .l()
            .expect("WindowMetrics is null");
        let bounds = env
            .call_method(
                &window_metrics,
                jni_str!("getBounds"),
                jni_sig!("()Landroid/graphics/Rect;"),
                &[],
            )
            .expect("Failed to call getBounds")
            .l()
            .expect("Bounds Rect is null");

        let width = env
            .call_method(&bounds, jni_str!("width"), jni_sig!("()I"), &[])
            .expect("Failed to call Rect.width()")
            .i()
            .expect("Width is not an integer");

        let height = env
            .call_method(&bounds, jni_str!("height"), jni_sig!("()I"), &[])
            .expect("Failed to call Rect.height()")
            .i()
            .expect("Height is not an integer");

        Ok((width, height))
    })
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

    // set full screen on android
    #[cfg(target_os = "android")]
    {
        unsafe {
            let (width, height) = get_android_screen_size().unwrap();
            let size_string = format!("{}x{}", (height) as u32, (width) as u32);
            println!("size {}", size_string);
            std::env::set_var("WOW_WIN", size_string);
        }
    }
}
