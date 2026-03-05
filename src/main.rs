extern crate os_release;

use os_release::OsRelease;

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

    let hostname = hostname::get().expect("Failed to parse hostname.");
    retv.push(hostname);


    retv



}

fn main() {
    let v = get_info();
    println!("{}", v[0]);
    println!("{}", v[1])
}
