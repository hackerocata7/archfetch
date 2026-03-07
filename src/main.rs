extern crate os_release;

use sysinfo::System;
use bytesize::ByteSize;
use get_shell::get_shell_name;
use os_release::OsRelease;
use std::ffi::OsString;
use std::fs;
use std::env;

fn compiletext(acii: Vec<String>, details: Vec<String>) -> String {
    String::new()
}

fn get_kern() -> String {
    let f = fs::read_to_string("/proc/version").expect("Couldnt get kernel version");
    let i = f.split(" ").collect::<Vec<&str>>();
    format!("{} {} {}", i[0], i[1], i[2])
}

fn get_info() -> Vec<String> {
    let mut retv = Vec::new();
    let OSR = OsRelease::new().expect("Cant parse os release");

    let release = OSR.pretty_name;
    let version = OSR.version;
    let os = format!("{release} {version}");
    retv.push(os);

    let hostname = hostname::get().expect("Failed to parse hostname.").into_string().expect("Error parseing to string hostname.");
    retv.push(hostname);

    let host = fs::read_to_string("/sys/devices/virtual/dmi/id/product_family").expect("Could not read host").trim().to_string();
    retv.push(host);

    let kernel = get_kern();
    retv.push(kernel);

    let shell = get_shell_name().expect("Could not parse shell name.");
    retv.push(shell);

    let wm = env::var("XDG_CURRENT_DESKTOP").expect("Couldnt retrive WM");
    retv.push(wm);

    let mut sys = System::new_all();
    sys.refresh_all();
    let ramgb = ByteSize::b(sys.used_memory()).as_gib().to_string()[..4].to_string();
    let ramgbtot = ByteSize::b(sys.total_memory()).as_gib().to_string()[..4].to_string();
    retv.push(format!("{ramgb} GB / {ramgbtot} GB"));

    retv



}

fn main() {
    let v = get_info();
    println!("{}", v[0]);
    println!("{}", v[1]);
    println!("{}", v[2]);
    println!("{}", v[3]);
    println!("{}", v[4]);
    println!("{}", v[5]);
    println!("{}", v[6]);
}
