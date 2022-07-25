use std::fs::{DirBuilder, self};
use std::io::{Write, self};
use std::path::PathBuf;
use std::env;
use std::process;

const FAILURE: i32 = 1;
const SUCCESS: i32 = 0;

struct WallData<'a> {
    m_dark: &'a PathBuf,
    m_light: &'a PathBuf,
    m_name: &'a str,
}

fn main() {
    let mut path_dark = PathBuf::new();
    match input_path("Enter Path to Dark Wallpaper", &mut path_dark) {
        Ok(_) => (),
        Err(_) => {
            println!("Couldnt read path from stdin");
            process::exit(FAILURE);
        },
    };
    verify_image(&path_dark);

    let mut path_light = PathBuf::new();
    match input_path("Enter Path to Light Wallpaper", &mut path_light) {
        Ok(_) => (),
        Err(_) => {
            println!("Couldnt read path from stdin");
            process::exit(FAILURE);
        },
    };
    verify_image(&path_light);

    println!("Enter Name of the Wallpaper");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Couldnt read name");

    let wall = WallData {
        m_dark: &path_dark,
        m_light: &path_light,
        m_name: &name,
    };
    
    wall.make();
}

//takes input path and returns a PathBuf
fn input_path<'a>(question: &str, path_obj: &'a mut PathBuf) -> Result<(), std::io::Error> {
    println!("{}", question);

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    buf = buf.trim().to_string();

    path_obj.clear();
    *path_obj = PathBuf::from(buf);
    Ok(())
}

//checks whether the provided path is valid and leads to a jpg, jpeg or png
fn verify_image(path_obj: &PathBuf) -> () { 
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

impl<'a> WallData<'a> {
    //verifies that the struct is ready to be used 
    fn _verify(&self) {
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
                        Err(_e) => ()
                    }
                }
            }
            Err(e) => panic!("{}", e),
        }
    }

    pub fn make(self) {
        self._verify();
        let xml_data = format!("<?xml version=\"1.0\" encoding=\"UTF-8\" ?><!DOCTYPE wallpapers SYSTEM \"gnome-wp-list.dtd\"><wallpapers><wallpaper deleted=\"false\"><name>{}</name><filename>{}</filename><filename-dark>{}</filename-dark><options>zoom</options><shade_type>solid</shade_type><pcolor>#3465a4</pcolor><scolor>#000000</scolor></wallpaper></wallpapers>", self.m_name, self.m_light.to_str().expect("Couldnt convert m_light to str"), self.m_dark.to_str().expect("Couldnt convert m_dark to str"));
        let home = match env::var("HOME") {
            Ok(t) => t,
            Err(e) => panic!("{}", e),
        };
        let xml_path = PathBuf::from(format!("{}/.local/share/gnome-background-properties/{}.xml", home, self.m_name.trim()));
        let mut xml = fs::File::create(xml_path).expect("Couldnt create the xml");
        match xml.write_all(xml_data.as_bytes()) {
            Ok(_) => println!("Done! You can set wallpaper from the settings"),
            Err(e) => panic!("{}", e),
        }
    }
}