use crate::constants::constants::{STRIKER_VERSION, STRIKER_WHO_AM_I};
use lazy_static::lazy_static;
use std::fmt;
use std::net::UdpSocket;
use std::process;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{net::ToSocketAddrs, sync::Once};

pub const SYSLOG_ADDRESS: &str = "192.168.0.27:10514";
pub const SYSLOG_MSG_MAX: usize = 1024;
pub const SYSLOG_FACILITY: u8 = 1 << 3;

#[allow(dead_code)]
pub const SYSLOG_CRIT: u8 = 2;
#[allow(dead_code)]
pub const SYSLOG_ERR: u8 = 3;
#[allow(dead_code)]
pub const SYSLOG_INFO: u8 = 6;
#[allow(dead_code)]
pub const SYSLOG_DEBUG: u8 = 7;

lazy_static! {
    static ref SYSLOG: Mutex<Option<UdpSocket>> = Mutex::new(None);
    static ref SYSLOG_ADDR: Mutex<Option<std::net::SocketAddr>> = Mutex::new(None);
}

static INIT: Once = Once::new();

pub fn init_xlog<A: ToSocketAddrs>(addr: A) {
    INIT.call_once(|| match UdpSocket::bind("0.0.0.0:0") {
        Ok(sock) => {
            let addr = addr.to_socket_addrs().unwrap().next().unwrap();
            *SYSLOG.lock().unwrap() = Some(sock);
            *SYSLOG_ADDR.lock().unwrap() = Some(addr);
        }
        Err(e) => {
            eprintln!("Failed to bind UDP socket: {}", e);
        }
    });
}

fn xlog_syslog(severity: u8, args: fmt::Arguments) {
    let priority = SYSLOG_FACILITY + severity;

    let message = format!("{}", args);
    let pid = process::id();
    let packet = format!("<{}>{}: [version={}] [PID={}] | {}", priority, STRIKER_WHO_AM_I, STRIKER_VERSION, pid, message);

    if packet.len() > SYSLOG_MSG_MAX {
        eprintln!("Syslog packet too long, truncating");
    }

    if let (Some(sock), Some(addr)) = (&*SYSLOG.lock().unwrap(), &*SYSLOG_ADDR.lock().unwrap()) {
        let _ = sock.send_to(packet.as_bytes(), addr);
    }
}

pub fn xlog_start(args: fmt::Arguments) -> usize {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
    xlog_syslog(SYSLOG_INFO, args);
    start as usize
}

#[allow(dead_code)]
pub fn xlog_debug(args: fmt::Arguments) {
    xlog_syslog(SYSLOG_DEBUG, args);
}

#[allow(dead_code)]
pub fn xlog_info(args: fmt::Arguments) {
    xlog_syslog(SYSLOG_INFO, args);
}

#[allow(dead_code)]
pub fn xlog_error(args: fmt::Arguments) {
    xlog_syslog(SYSLOG_ERR, args);
}

#[allow(dead_code)]
pub fn xlog_fatal(args: fmt::Arguments) {
    xlog_syslog(SYSLOG_CRIT, args);
}

#[allow(dead_code)]
pub fn xlog_panic(args: fmt::Arguments) {
    xlog_syslog(SYSLOG_CRIT, args);
    panic!("{}", args);
}

#[allow(dead_code)]
pub fn xlog_stop(start_time: usize, args: fmt::Arguments) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();

    let elapsed = now.saturating_sub(start_time as u64);
    let message = format!("{} | total time = {}s", args, elapsed);

    xlog_syslog(SYSLOG_INFO, format_args!("{}", message));
}
