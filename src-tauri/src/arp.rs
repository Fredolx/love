use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{process, thread};

use dns_lookup::lookup_addr;
use pnet::ipnetwork::IpNetwork;
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, ArpPacket, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::{MutablePacket, Packet};
use pnet_datalink::{DataLinkReceiver, DataLinkSender, MacAddr, NetworkInterface};
use std::io::ErrorKind::TimedOut;

const DATALINK_RCV_TIMEOUT: u64 = 500;
const ARP_PACKET_SIZE: usize = 28;
const ETHERNET_STD_PACKET_SIZE: usize = 42;
const INTERVAL: u64 = 10;
const TIMEOUT: u64 = 2000;

pub struct TargetDetails {
    pub ipv4: Ipv4Addr,
    pub mac: MacAddr,
    pub hostname: Option<String>,
    pub vendor: Option<String>,
}

pub fn get_interfaces() -> Vec<NetworkInterface> {
   return pnet_datalink::interfaces();
} 

pub fn scan(interface: &NetworkInterface) {
    let channel_config = pnet_datalink::Config {
        read_timeout: Some(Duration::from_millis(DATALINK_RCV_TIMEOUT)),
        ..pnet_datalink::Config::default()
    };

    let (mut tx, mut rx) = match pnet_datalink::channel(&interface, channel_config) {
        Ok(pnet_datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => {
            eprintln!("Expected an Ethernet datalink channel");
            process::exit(1);
        }
        Err(error) => {
            eprintln!("Datalink channel creation failed ({})", error);
            process::exit(1);
        }
    };
    let timed_out = Arc::new(AtomicBool::new(false));
    let cloned_timed_out = Arc::clone(&timed_out);
    let arp_responses = thread::spawn(move || receive_arp_responses(&mut rx, cloned_timed_out));

    let source_ip = find_source_ip(&interface);

    let cidr = get_cidr(interface);

    let ip_addresses = get_ip_addresses(cidr);

    for ip_address in ip_addresses {
        send_arp_request(&mut tx, &interface, source_ip, ip_address);
        thread::sleep(Duration::from_millis(INTERVAL));
    }

    let mut sleep_ms_mount: u64 = 0;
    while sleep_ms_mount < TIMEOUT {
        thread::sleep(Duration::from_millis(100));
        sleep_ms_mount += 100;
    }
    timed_out.store(true, Ordering::Relaxed);

    let target_details = arp_responses.join().unwrap_or_else(|error| {
        eprintln!("Failed to close receive thread ({:?})", error);
        process::exit(1);
    });
    println!("{:?}", target_details.into_iter().map(|f| format!("{} {}", f.ipv4.to_string(), f.hostname.unwrap_or_default().to_string()) ).collect::<Vec<String>>());
}

fn get_ip_addresses(cidr_str: String) -> Vec<Ipv4Addr> {
    let ip_network: IpNetwork = cidr_str.parse().expect("invalid cidr");
    let addresses: Vec<_> = if let IpNetwork::V4(network) = ip_network {
        network.iter().collect()
    } else {
        Vec::new()
    };
    return addresses;
}

fn find_source_ip(network_interface: &NetworkInterface) -> Ipv4Addr {
    let potential_network = network_interface
        .ips
        .iter()
        .find(|network| network.is_ipv4());
    match potential_network.map(|network| network.ip()) {
        Some(IpAddr::V4(ipv4_addr)) => ipv4_addr,
        _ => {
            eprintln!("Expected IPv4 address on network interface");
            process::exit(1);
        }
    }
}

fn get_cidr(interface: &NetworkInterface) -> String {
    return format!(
        "{}/{}",
        interface.ips.first().unwrap().ip().to_string(),
        interface.ips.first().unwrap().prefix().to_string()
    );
}

fn send_arp_request(
    tx: &mut Box<dyn DataLinkSender>,
    interface: &NetworkInterface,
    source_ip: Ipv4Addr,
    target_ip: Ipv4Addr,
) {
    let mut ethernet_buffer = vec![0u8; ETHERNET_STD_PACKET_SIZE];
    let mut ethernet_packet =
        MutableEthernetPacket::new(&mut ethernet_buffer).unwrap_or_else(|| {
            eprintln!("Could not build Ethernet packet");
            process::exit(1);
        });

    let target_mac = MacAddr::broadcast();
    let source_mac = interface.mac.unwrap_or_else(|| {
        eprintln!("Interface should have a MAC address");
        process::exit(1);
    });

    ethernet_packet.set_destination(target_mac);
    ethernet_packet.set_source(source_mac);

    let selected_ethertype = EtherTypes::Arp;
    ethernet_packet.set_ethertype(selected_ethertype);

    let mut arp_buffer = [0u8; ARP_PACKET_SIZE];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap_or_else(|| {
        eprintln!("Could not build ARP packet");
        process::exit(1);
    });

    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(ArpOperations::Request);
    arp_packet.set_sender_hw_addr(source_mac);
    arp_packet.set_sender_proto_addr(source_ip);
    arp_packet.set_target_hw_addr(target_mac);
    arp_packet.set_target_proto_addr(target_ip);

    ethernet_packet.set_payload(arp_packet.packet_mut());

    tx.send_to(
        ethernet_packet.to_immutable().packet(),
        Some(interface.clone()),
    );
}

fn receive_arp_responses(
    rx: &mut Box<dyn DataLinkReceiver>,
    timed_out: Arc<AtomicBool>,
) -> Vec<TargetDetails> {
    let mut discover_map: HashMap<Ipv4Addr, TargetDetails> = HashMap::new();

    loop {
        if timed_out.load(Ordering::Relaxed) {
            break;
        }

        let arp_buffer = match rx.next() {
            Ok(buffer) => buffer,
            Err(error) => {
                match error.kind() {
                    // The 'next' call will only block the thread for a given
                    // amount of microseconds. The goal is to avoid long blocks
                    // due to the lack of packets received.
                    TimedOut => continue,
                    _ => {
                        eprintln!("Failed to receive ARP requests ({})", error);
                        process::exit(1);
                    }
                };
            }
        };

        let ethernet_packet = match EthernetPacket::new(arp_buffer) {
            Some(packet) => packet,
            None => continue,
        };

        let is_arp_type = matches!(ethernet_packet.get_ethertype(), EtherTypes::Arp);
        if !is_arp_type {
            continue;
        }

        let arp_packet =
            ArpPacket::new(&arp_buffer[MutableEthernetPacket::minimum_packet_size()..]);

        // If we found an ARP packet, extract the details and add the essential
        // fields in the discover map. Please note that results are grouped by
        // IPv4 address - which means that a MAC change will appear as two
        // separete records in the result table.
        if let Some(arp) = arp_packet {
            let sender_ipv4 = arp.get_sender_proto_addr();
            let sender_mac = arp.get_sender_hw_addr();

            discover_map.insert(
                sender_ipv4,
                TargetDetails {
                    ipv4: sender_ipv4,
                    mac: sender_mac,
                    hostname: None,
                    vendor: None,
                },
            );
        }
    }

    // For each target found, enhance each item with additional results
    // results such as the hostname & MAC vendor.
    let target_details = discover_map
        .into_values()
        .map(|mut target_detail| {
            target_detail.hostname = find_hostname(target_detail.ipv4);
            //target_detail.vendor = vendor_list.search_by_mac(&target_detail.mac);
            target_detail
        })
        .collect();

    // The response summary can be used to display analytics related to the
    // performed ARP scans (packet counts, timings, ...)
    target_details
}

fn find_hostname(ipv4: Ipv4Addr) -> Option<String> {
    let ip: IpAddr = ipv4.into();
    match lookup_addr(&ip) {
        Ok(hostname) => {
            // The 'lookup_addr' function returns an IP address if no hostname
            // was found. If this is the case, we prefer switching to None.
            if hostname.parse::<IpAddr>().is_ok() {
                return None;
            }

            Some(hostname)
        }
        Err(_) => None,
    }
}


