use super::{get_system, get_host, get_kern, get_shell, get_wm, get_ram, get_disk, get_cpu};
use ansi_term::Colour;

// Define some variables that are used in lib.rs and here.
pub const COLOR_ICON: &str = "";
pub const SEPARATION: u8 = 2;

pub fn get_info() -> Vec<String> {
    // Create retv to later return it.
    let mut retv = Vec::new();

    // To add some text, push to retv
    retv.push(format!("󰍹 {}", get_system()));

    retv.push(format!("󰌢 {}", get_host()));

    retv.push(format!(" {}", get_kern()));

    retv.push(format!(" {}", get_shell()));

    retv.push(format!(" {}", get_wm()));

    retv.push(format!(" {}", get_ram()));

    retv.push(format!("󱛟 {}", get_disk()));

    retv.push(format!(" {}", get_cpu()));
    
    retv.push(format!(" {} {} {} {} {} {} {} {} ", Colour::Black.paint(COLOR_ICON), Colour::Red.paint(COLOR_ICON), Colour::Green.paint(COLOR_ICON), Colour::Yellow.paint(COLOR_ICON), Colour::Blue.paint(COLOR_ICON), Colour::Purple.paint(COLOR_ICON), Colour::Cyan.paint(COLOR_ICON), Colour::White.paint(COLOR_ICON)));
    
    // Return retv so it can be used upstream.
    retv



}
