#[cfg(target_os = "android")]
use bevy::android::ANDROID_APP;
#[cfg(target_os = "android")]
use jni::objects::{JObject, JString, JValue};
#[cfg(target_os = "android")]
use jni::{Env, JavaVM, errors::Result, jni_sig, jni_str};
use serde_json;
use serde_json::{Value, json};
use std::fs;
use std::path::PathBuf;

pub fn prepare_fs() {
    #[cfg(target_os = "ios")]
    {
        prepare_fs_ios();
    }

    let path = transform_path("Data");
    println!("path {}", path.to_str().unwrap());

    let files = list_files_in_documents(transform_path(""));
    println!("Contents list");
    for file in files {
        println!("- {}", file);
    }

    write_initial_data();
    read_config();
}

pub fn write_initial_data() {
    let file_path = transform_path("config.json");
    if !file_path.exists() {
        let default_config = json!({
            "host": "127.0.0.1:3724",
            "user": "changeme",
            "pass": "changeme"
        });

        let json_string = serde_json::to_string_pretty(&default_config).unwrap();
        fs::write(&file_path, json_string).unwrap();
    }
}

#[cfg(target_os = "ios")]
pub fn transform_path_ios(relative_path: &str) -> PathBuf {
    use objc2_foundation::{NSFileManager, NSSearchPathDirectory, NSSearchPathDomainMask};

    unsafe {
        let file_manager = NSFileManager::defaultManager();

        let urls = file_manager.URLsForDirectory_inDomains(
            NSSearchPathDirectory::DocumentDirectory,
            NSSearchPathDomainMask::UserDomainMask,
        );

        if let Some(doc_url) = urls.firstObject() {
            if let Some(path_nsstring) = doc_url.path() {
                let doc_path_string = path_nsstring.to_string();
                let mut base_path = PathBuf::from(doc_path_string);

                if !relative_path.is_empty() {
                    base_path.push(relative_path);
                }
                return base_path;
            }
        }
    }

    PathBuf::from("").join(relative_path)
}

#[cfg(target_os = "ios")]
pub fn prepare_fs_ios() -> Result<(), Box<dyn std::error::Error>> {
    use objc2_foundation::{NSFileManager, NSSearchPathDirectory, NSSearchPathDomainMask};
    use std::fs;

    unsafe {
        let file_manager = NSFileManager::defaultManager();
        let urls = file_manager.URLsForDirectory_inDomains(
            NSSearchPathDirectory::DocumentDirectory,
            NSSearchPathDomainMask::UserDomainMask,
        );

        if let Some(doc_url) = urls.firstObject() {
            if let Some(path_nsstring) = doc_url.path() {
                let path_string = path_nsstring.to_string();
                let mut file_path = std::path::PathBuf::from(path_string);
                file_path.push("Readme.txt");
                if !file_path.exists() {
                    fs::write(
                        &file_path,
                        "Put your Data directory here, change config.json, play :)",
                    )?;
                }
            }
        }
    }
    Ok(())
}

pub fn list_files_in_documents(target_path: PathBuf) -> Vec<String> {
    let mut file_list = Vec::new();

    for entry in fs::read_dir(target_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if let Some(file_name) = path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                file_list.push(name_str.to_string());
            }
        }
    }

    file_list
}

#[cfg(target_os = "android")]
pub fn get_android_media_path() -> PathBuf {
    let android_app = ANDROID_APP
        .get()
        .expect("ANDROID_APP is not set. Did you forget to use the #[bevy_main] macro?");

    let vm = unsafe { jni::JavaVM::from_raw(android_app.vm_as_ptr() as *mut _) };

    let path_buf_result: Result<PathBuf> = vm.attach_current_thread(|env| {
        let activity = unsafe { JObject::from_raw(env, android_app.activity_as_ptr() as *mut _) };
        let directory_documents = env
            .new_string("Documents")
            .expect("Failed to create Java string");

        let files_dir = env
            .call_method(
                &activity,
                jni_str!("getExternalFilesDir"),
                jni_sig!("(Ljava/lang/String;)Ljava/io/File;"),
                &[JValue::from(&directory_documents)],
            )
            .expect("Failed to call getExternalFilesDir")
            .l()
            .expect("getExternalFilesDir returned null");

        let path_object = env
            .call_method(
                &files_dir,
                jni_str!("getAbsolutePath"),
                jni_sig!("()Ljava/lang/String;"),
                &[],
            )
            .expect("Failed to call getAbsolutePath")
            .l()
            .expect("getAbsolutePath returned null");

        let path_jstring: JString =
            JString::cast_local(env, path_object).expect("Failed to cast path object to JString");

        let java_path: String = env
            .get_string(&path_jstring)
            .expect("Failed to get Java string")
            .into();

        let android_media_path = java_path.replace("Android/data", "Android/media");
        Ok(PathBuf::from(android_media_path))
    });

    let android_media_path: PathBuf = path_buf_result.unwrap();
    android_media_path
}

#[cfg(target_os = "android")]
pub fn transform_path_android(relative_path: &str) -> PathBuf {
    get_android_media_path().join("WoW/").join(relative_path)
}

pub fn transform_path(relative_path: &str) -> PathBuf {
    #[cfg(target_os = "ios")]
    return transform_path_ios(relative_path);

    #[cfg(target_os = "android")]
    return transform_path_android(relative_path);

    transform_path_pkg(relative_path)
}

pub fn transform_path_pkg(relative_path: &str) -> PathBuf {
    if let Ok(mut exe_path) = std::env::current_exe() {
        if exe_path.pop() {
            return exe_path.join(relative_path);
        }
    }
    PathBuf::from("").join(relative_path)
}

pub fn read_config() {
    let json_str = fs::read_to_string(transform_path("config.json")).unwrap();
    let json: Value = serde_json::from_str(&json_str).unwrap();

    let host = extract_string(&json, "host");
    let user = extract_string(&json, "user");
    let pass = extract_string(&json, "pass");

    set_env_if_not_empty("WOW_HOST", &host);
    set_env_if_not_empty("WOW_USER", &user);
    set_env_if_not_empty("WOW_PASS", &pass);
}

pub fn extract_string(json: &Value, key: &str) -> Option<String> {
    json.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

pub fn set_env_if_not_empty(key: &str, value: &Option<String>) {
    if let Some(val) = value {
        if !val.is_empty() {
            unsafe {
                std::env::set_var(key, val);
            }
        }
    }
}
