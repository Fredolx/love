#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use arp::scan;
// use arp::kill;
use models::{Interface, LanClient};
use pnet_datalink::NetworkInterface;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
};
mod arp;
mod models;
mod vendor;

static VENDORS: OnceLock<HashMap<String, String>> = OnceLock::new();
#[derive(Default)]
struct State {
    lan_clients: Arc<Mutex<Vec<LanClient>>>,
}
fn main() {
    netdev::get_interfaces()
        .into_iter()
        .for_each(|f| println!("{}", f.name));
    VENDORS.set(vendor::get_vendors().unwrap()).unwrap();
    tauri::Builder::default()
        .manage(State {
            ..Default::default()
        })
        .invoke_handler(tauri::generate_handler![get_interfaces, get_lan, kill_device])
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
fn kill_device(client: LanClient, inter: String) {
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
    )
}
