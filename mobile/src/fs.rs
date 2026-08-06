use serde_json;
use serde_json::{Value, json};
use std::fs;
use std::path::PathBuf;

pub fn prepare_fs() {
    #[cfg(target_os = "ios")]
    {
        prepare_fs_ios();
        let path = transform_path_ios("Data");
        println!("path {}", path.to_str().unwrap());

        match list_files_in_documents(transform_path_ios("")) {
            Ok(files) => {
                println!("Contents list");
                for file in files {
                    println!("- {}", file);
                }
            }
            Err(e) => println!("Failed to read directory: {}", e),
        }

        read_config();
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
                let mut file_path = std::path::PathBuf::from(path_string.clone());
                file_path.push("Readme.txt");
                if !file_path.exists() {
                    fs::write(
                        &file_path,
                        "Put your Data directory here, change config.json, play :)",
                    )?;
                }

                let mut file_path = std::path::PathBuf::from(path_string);
                file_path.push("config.json");
                if !file_path.exists() {
                    let default_config = json!({
                        "host": "127.0.0.1:3724",
                        "user": "changeme",
                        "pass": "changeme"
                    });

                    let json_string = serde_json::to_string_pretty(&default_config).unwrap();
                    fs::write(&file_path, json_string)?;
                }
            }
        }
    }
    Ok(())
}

pub fn list_files_in_documents(
    target_path: PathBuf,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut file_list = Vec::new();

    for entry in fs::read_dir(target_path)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(file_name) = path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                file_list.push(name_str.to_string());
            }
        }
    }

    Ok(file_list)
}

pub fn transform_path(relative_path: &str) -> PathBuf {
    #[cfg(target_os = "ios")]
    return transform_path_ios(relative_path);

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
