#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use arp::scan;
// use arp::kill;
use models::{Interface, LanClient};
use pnet_datalink::NetworkInterface;

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, OnceLock,
    },
    time::Duration,
};
mod arp;
mod models;
mod vendor;

static VENDORS: OnceLock<HashMap<String, String>> = OnceLock::new();

pub struct State {
    threads: Arc<Mutex<HashMap<String, LoveThreads>>>,
}

pub struct LoveThreads {
    timed_out: Arc<AtomicBool>,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let vendors_path = app.path_resolver().resolve_resource("vendors.txt").unwrap();
            VENDORS
                .set(vendor::get_vendors(vendors_path).unwrap())
                .unwrap();
            Ok(())
        })
        .manage(State {
            threads: Arc::new(Mutex::new(HashMap::new())),
        })
        .invoke_handler(tauri::generate_handler![
            get_interfaces,
            get_lan,
            kill_device,
            stop_kill_device
        ])
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
            ip: f
                .ips
                .iter()
                .find(|f| f.is_ipv4())
                .unwrap_or(f.ips.first().unwrap())
                .to_string(),
            mac: f.mac.unwrap_or_default().to_string(),
        })
        .collect();
}

fn find_interface_by_name(inter_str: String) -> NetworkInterface {
    return arp::get_interfaces()
        .into_iter()
        .find(|f| f.name == inter_str)
        .unwrap();
}

#[tauri::command(async)]
fn get_lan(inter: String) -> Vec<LanClient> {
    let interface = find_interface_by_name(inter);
    let lan_clients: Vec<LanClient> = scan(&interface)
        .into_iter()
        .map(|f| LanClient {
            ip: f.ipv4.to_string(),
            hostname: f.hostname,
            vendor: f.vendor,
            mac: f.mac.to_string(),
        })
        .collect();
    return lan_clients;
}

#[tauri::command(async)]
fn kill_device(client: LanClient, inter: String, delay: u64, state: tauri::State<State>) {
    let interface = find_interface_by_name(inter);
    let gateway = arp::find_gateway(&interface);
    arp::kill(
        arp::TargetDetails {
            ipv4: client.ip.parse().expect("Can't parse client ip"),
            mac: client.mac.parse().expect("Can't parse client mac"),
            hostname: client.hostname,
            vendor: client.vendor,
        },
        gateway,
        &interface,
        Duration::from_millis(delay),
        state,
    );
}

#[tauri::command(async)]
fn stop_kill_device(client: LanClient, state: tauri::State<State>) -> Result<(), String> {
    println!("stopping murder of {}", client.ip);
    state
        .threads
        .lock()
        .unwrap()
        .get(&client.ip)
        .ok_or(format!("Failed to get {} loveThread from state hashmap", client.ip))?
        .timed_out
        .store(true, Ordering::Relaxed);
    Ok(())
}
