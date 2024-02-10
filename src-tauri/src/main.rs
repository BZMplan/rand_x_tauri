#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::fs::File;
use std::io::{BufReader, BufRead,Write};
use rand_x_tauri::get_info::get_info;
use rand_x_tauri::name::name;
use rand::prelude::*;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


#[tauri::command]
fn get_random_num() -> String {
    let len_usize = name().len();
    let name =name();
    let len = len_usize as i32;
    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(0..len) as usize;
    //let mut list = studnet_list();
    let read_data = read_data_from_file("data/data.randx");
    modify_line_in_file("data/data.randx", random_num.try_into().unwrap(),get_number(read_data,random_num)+1);
    let student = name[random_num];
    format!("恭喜{}同学",student.to_string())
    
}

fn modify_line_in_file(filename: &str, line_number: usize, new_data: i32) {
    let mut lines = read_data_from_file(filename);
    if line_number < lines.len() {
        lines[line_number] = new_data.to_string();
        write_data_to_file(filename, &lines);
    } else {
        println!("Line number out of range");
    }
}
fn string_to_i32(string: &String) -> i32 {
    string.parse().unwrap()
}

fn get_number(n:Vec<String>,m:usize) -> i32{
    string_to_i32(&n[m])
}

fn read_data_from_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.expect("Failed to read line")).collect()
}

fn write_data_to_file(filename: &str, data: &[String]) {
    let mut file = File::create(filename).expect("Failed to create file");
    for line in data {
        writeln!(file, "{}", line).expect("Failed to write to file");
    }
}



#[tauri::command]
//列表
// fn get_list(index: usize) -> String {
//     let name = name();
//     let (count,position) =get_info(index-1);
//     format!("{}同学已被抽中{}次",name[position],count)
// }

//条形统计图
fn get_list(index:usize) -> (String,i32){
    let name = name();
    let (count,position) = get_info(index-1);
    (name[position].to_string(),count)
}
#[tauri::command]
fn get_count(index:usize) -> i32{
    let (count,_) = get_info(index-1);
    count
}
#[tauri::command]
fn get_name(index:usize) -> String{
    let name = name();
    let (_,position) = get_info(index-1);
    name[position].to_string()
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_random_num, get_list,get_count,get_name])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
