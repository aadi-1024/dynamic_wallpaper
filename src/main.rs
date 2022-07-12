// use std::fs::File;
use std::io;
use std::path::PathBuf;
fn main() {
    let path_buf = &mut PathBuf::new();
    let path_dark = input_path("Enter Path to Dark Wallpaper", path_buf);
    println!("{:?}", path_dark);
}

fn input_path<'a>(question: &str, path_obj: &'a mut PathBuf) -> &'a PathBuf {
    println!("{}", question);
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Couldnt properly read input");
    buf = buf.trim().to_string();
    path_obj.clear();
    *path_obj = PathBuf::from(buf);
    path_obj
}