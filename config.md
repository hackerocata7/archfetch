# Configuration
Basic configuration guide for archfetch. Configuration is written in Rust.

## Basics

### Add text:

    pub fn get_info() -> Vec<String> {
	    let mut retv = Vec::new();
	    retv.push(String::from("Hiii"));
	    retv
    }
### Add text and variable:

    pub fn get_info() -> Vec<String> {
	    let mut retv = Vec::new();
	    // Define a variable
	    let c = 1;
	    retv.push(format!("c is {}", c));
	    retv
    }
## Methods

 - `get_system()` -> Returns os version (`Linux (Arch Linux rolling)`)
 - `get_host()` -> Returns host name (`ThinkPad T480`)
 - `get_kern()` -> Returns linux kernel version (`Linux version 6.18.13-arch1-1`)
 - `get_shell()` -> Returns the shell (`bash`)
 - `get_de()` -> Returns `$XDG_CURRENT_DESKTOP` (`Gnome`)
 - `get_ram()` -> Returns the used up ram in GiB (`3 GiB / 8 GiB`)
 - `get_disk()` -> Returns the total space used in GiB (`50 GiB / 500 GiB`)
 - `get_cpu()` -> returns the motherboard manifacturer and model (`LENOVO 20FVS13C00`)
