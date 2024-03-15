#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use models::Interface;
mod arp;
mod models;
fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_interfaces])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command(async)]
fn get_interfaces() -> Vec<Interface> {
    return arp::get_interfaces().iter().filter(|f| f.ips.len() > 0).map(|f| Interface {
    name: f.name.clone(),
    ip: f.ips.first().unwrap().to_string(),
    mac: f.mac.unwrap_or_default().to_string()
   }).collect::<Vec<Interface>>();
}
