use std::fs::File;
use std::env;
use std::io::Read;

fn replace_character(string: &str, old_character: char, new_character: char) -> String {
    let mut new_string: String = String::new();
    for character in string.chars() {
        if character == old_character {
            new_string.push(new_character);
        } else {
            new_string.push(character);
        }
    }
  
    return new_string;
}

fn get_string_before_character(string: &str, character: char) -> &str {
    let character_index: usize = string.find(character).unwrap();
    return &string[..character_index];
}

fn supported_web_ver(version_num: &str) -> bool
{
    let version: i32 = get_string_before_character(version_num, '.').to_string().parse().unwrap();
    if version >= 94 {
        return true;
    }
    return false;
}

pub fn get() -> Vec<&'static str> {
    let mut current_available_browsers: Vec<&str> = Vec::new();

    // Get the APPDATA environment variable.
    let appdata: String      = env::var("APPDATA").unwrap();

    // The %LOCALAPPDATA%\ path is `%APPDATA%\Local`.
    let localappdata: String = format!("{}/Local", appdata);
    
    let chrome_version_file_path: &str = &format!("{}/{}/{}/{}/{}", replace_character(&localappdata.replace("Roaming", ""), '\\', '/'), "Google", "Chrome", "User Data", "Last Version");
    let msedge_version_file_path: &str = &format!("{}/{}/{}/{}/{}", replace_character(&localappdata.replace("Roaming", ""), '\\', '/'), "Microsoft", "Edge", "User Data", "Last Version");

    let mse_file: Result<File, std::io::Error>     = File::open(format!(r"{}", msedge_version_file_path));
    let mut mse_data: String = String::new();
    if mse_file.is_ok() {
        mse_file.unwrap().read_to_string(&mut mse_data).expect("INVALID_MSEDGE_PATH");
        if supported_web_ver(&mse_data) {
            current_available_browsers.push("MICROSOFT_EDGE");
        }
    }
    
    let chv_file: Result<File, std::io::Error>     = File::open(format!(r"{}", chrome_version_file_path));
    let mut chv_data: String = String::new();
    if chv_file.is_ok() {
        chv_file.unwrap().read_to_string(&mut chv_data).expect("INVALID_CHROME_PATH");
        if supported_web_ver(&chv_data) {
            current_available_browsers.push("GOOGLE_CHROME");
        }
    }

    return current_available_browsers;
}
