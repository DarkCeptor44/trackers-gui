// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate reqwest;

const URL_BEST: &str =
    "https://raw.githubusercontent.com/ngosang/trackerslist/master/trackers_best.txt";
const URL_ALL: &str =
    "https://raw.githubusercontent.com/ngosang/trackerslist/master/trackers_all.txt";
const URL_BEST_IP: &str =
    "https://raw.githubusercontent.com/ngosang/trackerslist/master/trackers_best_ip.txt";
const URL_ALL_IP: &str =
    "https://raw.githubusercontent.com/ngosang/trackerslist/master/trackers_all_ip.txt";

#[tauri::command]
fn get_trackers(list: &str) -> String {
    let url = match list {
        "best" => URL_BEST,
        "all" => URL_ALL,
        "best_ip" => URL_BEST_IP,
        "all_ip" => URL_ALL_IP,
        _ => URL_BEST,
    };

    match _get_trackers(url) {
        Ok(resp) => resp,
        Err(_) => "".to_string(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_trackers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn _get_trackers(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let resp = client.get(url).send()?.text()?;
    Ok(resp)
}
