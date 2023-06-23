
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use chat_tauri::list::studnet_list;
use rand::prelude::*;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn get_random_num() -> i32{
    let len_usize = studnet_list().len();
    let len = len_usize as i32;
    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(0..len);
    random_num
}
#[tauri::command]
fn get_list(num:i32) -> String{
    let mut students = studnet_list();
    students.sort();
    let top_five = &students[(students.len() - 5)..];
    let n = num as usize;
    format!("Student Name,{}",top_five[5-n])
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,get_random_num,get_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
