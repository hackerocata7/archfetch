use sysinfo::{System, Disks, Motherboard};
use bytesize::ByteSize;
use get_shell::get_shell_name;
use std::{fs, env};
use ansi_term::Colour;

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

fn get_kern() -> (String, String, String) {
    let f = fs::read_to_string("/proc/version").expect("Couldnt get kernel version");
    let i = f.split(" ").collect::<Vec<&str>>();
    (i[0].to_string(), i[1].to_string(), i[2].to_string())
}

fn get_ram() -> (String, String) {
    let mut sys = System::new_all();
    sys.refresh_all();



    let ramgb = ByteSize::b(sys.used_memory()).as_gib().to_string()[..4].to_string();
    let ramgbtot = ByteSize::b(sys.total_memory()).as_gib().to_string()[..4].to_string();
    (ramgb, ramgbtot)
}

fn get_disk() -> (String, String) {
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
fn get_info() -> Vec<String> {
    let mut retv = Vec::new();
    
    let mut sys = System::new_all();
    let m = Motherboard::new().unwrap();
    sys.refresh_all();

    retv.push(format!("󰍹 {}", System::long_os_version().unwrap()));

    retv.push(format!("󰌢 {}", fs::read_to_string("/sys/devices/virtual/dmi/id/product_family").expect("Could not read host").trim().to_string()));

    retv.push(format!(" {} {} {}", get_kern().0, get_kern().1, get_kern().2));

    retv.push(format!(" {}", get_shell_name().expect("Could not parse shell name.")));

    retv.push(format!(" {}", env::var("XDG_CURRENT_DESKTOP").expect("Couldnt retrive WM")));

    retv.push(format!(" {} GiB / {} GiB", get_ram().0, get_ram().1));

    retv.push(format!("󱛟 {} GiB / {} GiB", get_disk().1, get_disk().0));

    retv.push(format!(" {} {}", m.vendor_name().unwrap(), m.name().unwrap()));
    
    retv.push(format!(" {} {} {} {} {} {} {} {} ", Colour::Black.paint(""), Colour::Red.paint(""), Colour::Green.paint(""), Colour::Yellow.paint(""), Colour::Blue.paint(""), Colour::Purple.paint(""), Colour::Cyan.paint(""), Colour::White.paint("")));
    
    retv



}

fn main() {
    let v = get_info();
    let l = do_ascii();
    compiletext(l, v);
}
