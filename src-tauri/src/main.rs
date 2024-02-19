// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod systeminfo;
mod timer;
mod db;
use timer::counter;
use systeminfo::get_cpu_usage;
use db::insert_db;
use db::retrieve_logs;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      counter, get_cpu_usage, insert_db, retrieve_logs
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
