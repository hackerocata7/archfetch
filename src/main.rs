extern crate os_release;

use sysinfo::{System, Disks};
use bytesize::ByteSize;
use get_shell::get_shell_name;
use os_release::OsRelease;
use std::{fs, env};

fn compiletext(acii: Vec<String>, details: Vec<String>) {
    let mut prtv:  Vec<String> = Vec::new();
    for i in 0..acii.len() {
        prtv.push(format!("{} {}", acii[i], if i % 2 == 0 {if i / 2 >= details.len() {"".to_string()} else {details[i / 2].clone()}} else {"".to_string()}));
    }

    for i in prtv {
        println!("{}", i);
    }
}

fn do_ascii() -> Vec<String> {
    let mut retv = Vec::new();
    let homedir = std::env::var("HOME").expect("Coudnt locate home directory.");
    let ascii = fs::read_to_string(format!("{homedir}/.config/archfetch/logo.txt")).expect("Couldnt locate ~/.config/archfetch/logo.txt");
    let spa = ascii.split("\n");
    for i in spa {
        retv.push(i.to_string());
    }
    retv
}

fn get_kern() -> String {
    let f = fs::read_to_string("/proc/version").expect("Couldnt get kernel version");
    let i = f.split(" ").collect::<Vec<&str>>();
    format!(" {} {} {}", i[0], i[1], i[2])
}

fn get_info() -> Vec<String> {
    let mut retv = Vec::new();
    let osr = OsRelease::new().expect("Cant parse os release");

    let release = osr.pretty_name;
    let version = osr.version;
    let os = format!("󰍹 {release} {version}");
    retv.push(os);

//    let hostname = hostname::get().expect("Failed to parse hostname.").into_string().expect("Error parseing to string hostname.");
//    retv.push(hostname);

    let host = fs::read_to_string("/sys/devices/virtual/dmi/id/product_family").expect("Could not read host").trim().to_string();
    retv.push(format!("󰌢 {host}"));

    let kernel = get_kern();
    retv.push(kernel);

    let shell = get_shell_name().expect("Could not parse shell name.");
    retv.push(format!(" {shell}"));

    let wm = env::var("XDG_CURRENT_DESKTOP").expect("Couldnt retrive WM");
    retv.push(format!(" {wm}"));

    let mut sys = System::new_all();
    sys.refresh_all();
    // Ignore the double .to_string(); Very hacked thing. LOL
    let ramgb = ByteSize::b(sys.used_memory()).as_gib().to_string()[..4].to_string();
    let ramgbtot = ByteSize::b(sys.total_memory()).as_gib().to_string()[..4].to_string();
    retv.push(format!(" {ramgb} GiB / {ramgbtot} GiB"));

        
    let disks = Disks::new_with_refreshed_list();
    let mut total_sp: u64 = 0;
    let mut used_sp: u64 = 0;
    for disk in &disks {
        total_sp += disk.total_space();
        used_sp += disk.total_space() - disk.available_space();
    }
    let totalg = ByteSize::b(total_sp).as_gib().to_string()[..5].to_string();
    let usedg = ByteSize::b(used_sp).as_gib().to_string()[..4].to_string();
    retv.push(format!("󱛟 {} GiB / {} GiB", usedg, totalg));
    retv



}

fn main() {
    let v = get_info();
    let l = do_ascii();
//    println!("{}", v[0]);
//    println!("{}", v[1]);
//    println!("{}", v[2]);
//    println!("{}", v[3]);
//    println!("{}", v[4]);
//    println!("{}", v[5]);
//    for i in &l {
//        println!("{}", i);
//    }
    compiletext(l, v);
}
