// use std::fs::File;
use std::io;
use std::path::PathBuf;
fn main() {
    let path_buf = &mut PathBuf::new();
    let path_dark = input_path("Enter Path to Dark Wallpaper", path_buf);
    dbg!(verify_image(path_dark)); 
}

//takes input path and returns a PathBuf
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

//checks whether the provided path is valid and leads to a jpg, jpeg or png
fn verify_image(path_obj: &PathBuf) -> bool { 
    if !path_obj.exists() {
        false
    } else {
        if let Some(t) = path_obj.extension() {
            dbg!(t);
            if let Some(x) = t.to_str() {
                dbg!(x);
                match x {
                    "png" | "jpg" | "jpeg" => true,
                    _ => false
                }
            }
            else {
                false
            }
        } else {
            false
        }
    }
}