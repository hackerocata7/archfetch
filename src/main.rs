use archfetch::{compiletext, get_info, do_ascii};

fn main() {
    let v = get_info();
    let l = do_ascii();
    compiletext(l, v);
}
