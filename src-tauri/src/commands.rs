// use a2_utils::file;
mod request;

// config =====
#[tauri::command]
pub fn print(s: &str)  {
    println!("{}", s);
}

#[tauri::command]
pub fn download_chromedriver_versions() -> Result<String, String> {
    request::download_chromedriver_versions().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn download_chromedriver_known_versions() -> Result<String, String> {
    request::download_chromedriver_known_versions().map_err(|e| e.to_string())
}
