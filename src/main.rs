use std::fs::{File, DirBuilder};
use std::io;
use std::path::PathBuf;
use std::env;

struct Wall_data<'a> {
    m_dark: &'a PathBuf,
    m_light: &'a PathBuf,
    m_name: &'a str,
}

fn main() {
    let mut path_dark = PathBuf::new();
    input_path("Enter Path to Dark Wallpaper", &mut path_dark);
    _verify_image(&path_dark);

    let mut path_light = PathBuf::new();
    input_path("Enter Path to Light Wallpaper", &mut path_light);
    _verify_image(&path_light);

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Couldnt read name");

    let mut wall = Wall_data {
        m_dark: &path_dark,
        m_light: &path_light,
        m_name: &name,
    };
    
    wall._verify();
}

//takes input path and returns a PathBuf
fn input_path<'a>(question: &str, path_obj: &'a mut PathBuf) -> () {
    println!("{}", question);
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Couldnt properly read input");
    buf = buf.trim().to_string();
    path_obj.clear();
    *path_obj = PathBuf::from(buf);
}

//checks whether the provided path is valid and leads to a jpg, jpeg or png
fn _verify_image(path_obj: &PathBuf) -> () { 
    if !path_obj.exists() {
        panic!("File doesnt exist");
    } else {
        if let Some(t) = path_obj.extension() {
            if let Some(x) = t.to_str() {
                match x {
                    "png" | "jpg" | "jpeg" => (),
                    _ => panic!("Not a supported file format"),
                }
            }
            else {
                panic!();
            }
        } else {
            panic!("Path specified doesnt exist");
        }
    }
}

impl<'a> Wall_data<'a> {
    pub fn _verify(&self) {
        let home = match env::var("HOME") {
            Ok(t) => t,
            Err(e) => panic!("{}", e),
        };
        let temp = format!("{}/.local/share/gnome-background-properties", home);
        let name = self.m_name;
        let xml_path = &PathBuf::from(temp);
        match xml_path.exists() {
            true => (),
            false => match DirBuilder::new().create(xml_path) {
                Ok(_) => (),
                Err(e) => panic!("Couldnt create directory due to {} ", e),
            }
        }
        match std::fs::read_dir(xml_path) {
            Ok(f) => {
                for i in f {
                    match i {
                        Ok(t) => {
                            if let Some(t) = t.file_name().to_str() {
                                if t == format!("{}.xml", name.trim()) {
                                    panic!("File with name {} already exists", name);
                                }
                            } else {
                                ()
                            }
                        },
                        Err(e) => ()
                    }
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
}