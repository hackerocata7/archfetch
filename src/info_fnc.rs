use super::{get_system, get_host, get_kern, get_shell, get_wm, get_ram, get_disk, get_cpu};
use ansi_term::Colour;

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
