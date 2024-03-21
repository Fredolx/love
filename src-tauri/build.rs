use std::env;

fn main() {
    if env::consts::OS == "windows" {
        println!("cargo:rustc-link-arg=delayimp.lib");
        let delay_load_dlls: [&str; 3] = ["wpcap.dll", "Packet.dll", "RemoteCapture.dll"];
        for dll in delay_load_dlls {
            println!("cargo:rustc-link-arg=/DELAYLOAD:{dll}");
        }
    }
    tauri_build::build()
}
