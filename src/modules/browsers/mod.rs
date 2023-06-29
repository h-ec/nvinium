use std::fs::File;
use std::env;
use std::io::Read;

fn replace_character(string: &str, old_character: char, new_character: char) -> String {
    let mut new_string = String::new();
    for character in string.chars() {
        if character == old_character {
            new_string.push(new_character);
        } else {
            new_string.push(character);
        }
    }
  
    return new_string;
}

pub fn get() -> Vec<&'static str> {
    let mut current_available_browsers = Vec::new();

    // Get the APPDATA environment variable.
    let appdata      = env::var("APPDATA").unwrap();

    // The %LOCALAPPDATA%\ path is `%APPDATA%\Local`.
    let localappdata = format!("{}/Local", appdata);
    
    let chrome_version_file_path: &str = &format!("{}/{}/{}/{}/{}", replace_character(&localappdata.replace("Roaming", ""), '\\', '/'), "Google", "Chrome", "User Data", "Last Version");
    let msedge_version_file_path: &str = &format!("{}/{}/{}/{}/{}", replace_character(&localappdata.replace("Roaming", ""), '\\', '/'), "Microsoft", "Edge", "User Data", "Last Version");

    let mse_file     = File::open(format!(r"{}", msedge_version_file_path));
    let mut mse_data = String::new();
    if mse_file.is_ok() {
        mse_file.unwrap().read_to_string(&mut mse_data).expect("INVALID_MSEDGE_PATH");
        current_available_browsers.push("MICROSOFT_EDGE");
    }
    
    let chv_file     = File::open(format!(r"{}", chrome_version_file_path));
    let mut chv_data = String::new();
    if chv_file.is_ok() {
        chv_file.unwrap().read_to_string(&mut chv_data).expect("INVALID_CHROME_PATH");
        current_available_browsers.push("GOOGLE_CHROME");
    }

    return current_available_browsers;
}