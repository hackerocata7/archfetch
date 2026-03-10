use sysinfo::{System, Disks, Motherboard};
use bytesize::ByteSize;
use get_shell::get_shell_name;
use std::{fs, env};
use ansi_term::Colour;

pub fn compiletext(acii: Vec<String>, details: Vec<String>) {
    let mut prtv:  Vec<String> = Vec::new();
    for i in 0..acii.len() {
        prtv.push(format!("{} {}", acii[i], if i % 2 == 0 {if i / 2 >= details.len() {"".to_string()} else {details[i / 2].clone()}} else {"".to_string()}));
    }

    for i in prtv {
        println!("{}", i);
    }
}

pub fn do_ascii() -> Vec<String> {
    let mut retv = Vec::new();
    let homedir = std::env::var("HOME").expect("Coudnt locate home directory.");
    let ascii = fs::read_to_string(format!("{homedir}/.config/archfetch/logo.txt")).expect("Couldnt locate ~/.config/archfetch/logo.txt");
    let spa = ascii.split("\n");
    for i in spa {
        retv.push(i.to_string());
    }
    retv
}

fn get_kern_low() -> (String, String, String) {
    let f = fs::read_to_string("/proc/version").expect("Couldnt get kernel version");
    let i = f.split(" ").collect::<Vec<&str>>();
    (i[0].to_string(), i[1].to_string(), i[2].to_string())
}

fn get_ram_low() -> (String, String) {
    let mut sys = System::new_all();
    sys.refresh_all();



    let ramgb = ByteSize::b(sys.used_memory()).as_gib().to_string()[..4].to_string();
    let ramgbtot = ByteSize::b(sys.total_memory()).as_gib().to_string()[..4].to_string();
    (ramgb, ramgbtot)
}

fn get_disk_low() -> (String, String) {
    let disks = Disks::new_with_refreshed_list();
    let mut total_sp: u64 = 0;
    let mut used_sp: u64 = 0;
    for disk in &disks {
        total_sp += disk.total_space();
        used_sp += disk.total_space() - disk.available_space();
    }

    let totalg = ByteSize::b(total_sp).as_gib().to_string()[..5].to_string();
    let usedg = ByteSize::b(used_sp).as_gib().to_string()[..4].to_string();
    
    (totalg, usedg)
}

fn get_ram() -> String {
    format!("{} GiB / {} GiB", get_ram_low().0, get_ram_low().1)
}

fn get_kern() -> String {
    format!("{} {} {}", get_kern_low().0, get_kern_low().1, get_kern_low().2)
}

fn get_system() -> String {
    System::long_os_version().unwrap()
}

fn get_host() -> String {
    fs::read_to_string("/sys/devices/virtual/dmi/id/product_family").expect("Could not read host").trim().to_string()
}

fn get_shell() -> String {
    get_shell_name().expect("Could not parse shell name.")
}

fn get_wm()-> String {
    env::var("XDG_CURRENT_DESKTOP").expect("Couldnt retrive WM")
}

fn get_cpu() -> String {
    let m = Motherboard::new().unwrap();
    format!("{} {}", m.vendor_name().unwrap(), m.name().unwrap())
}

fn get_disk() -> String {
    format!("{} GiB / {} GiB", get_disk_low().1, get_disk_low().0)
}

pub fn get_info() -> Vec<String> {
    let mut retv = Vec::new();
        
    retv.push(format!("󰍹 {}", get_system()));

    retv.push(format!("󰌢 {}", get_host()));

    retv.push(format!(" {}", get_kern()));

    retv.push(format!(" {}", get_shell()));

    retv.push(format!(" {}", get_wm()));

    retv.push(format!(" {}", get_ram()));

    retv.push(format!("󱛟 {}", get_disk()));

    retv.push(format!(" {}", get_cpu()));
    
    retv.push(format!(" {} {} {} {} {} {} {} {} ", Colour::Black.paint(""), Colour::Red.paint(""), Colour::Green.paint(""), Colour::Yellow.paint(""), Colour::Blue.paint(""), Colour::Purple.paint(""), Colour::Cyan.paint(""), Colour::White.paint("")));
    
    retv



}
