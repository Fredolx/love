#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, sync::OnceLock};
use arp::scan;
use models::{Interface, LanClient};
mod arp;
mod models;
mod vendor;

static VENDORS: OnceLock<HashMap<String, String>> = OnceLock::new();

fn main() {
    VENDORS.set(vendor::get_vendors().unwrap()).unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_interfaces, get_lan])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command(async)]
fn get_interfaces() -> Vec<Interface> {
    return arp::get_interfaces()
        .iter()
        .filter(|f| f.ips.len() > 0)
        .map(|f| Interface {
            name: f.name.clone(),
            ip: f.ips.first().unwrap().to_string(),
            mac: f.mac.unwrap_or_default().to_string(),
        })
        .collect();
}

#[tauri::command(async)]
fn get_lan(inter: String) -> Vec<LanClient> {
    let interface = arp::get_interfaces()
        .into_iter()
        .find(|f| f.name == inter)
        .unwrap();
    return scan(&interface)
        .into_iter()
        .map(|f| LanClient {
            ip: f.ipv4.to_string(),
            hostname: f.hostname,
            vendor: f.vendor,
            mac: f.mac.to_string()
        })
        .collect();
}
