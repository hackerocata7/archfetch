extern crate os_release;

use os_release::OsRelease;
use std::ffi::OsString;
use std::fs;

fn compiletext(acii: Vec<String>, details: Vec<String>) -> String {
    String::new()
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

    let host = fs::read_to_string("/sys/devices/virtual/dmi/id/product_family").expect("Could not read host");
    retv.push(host);


    retv



}

fn main() {
    let v = get_info();
    println!("{}", v[0]);
    println!("{}", v[1]);
    println!("{}", v[2]);
}
