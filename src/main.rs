use std::fs::{DirBuilder, self};
use std::io::{Write, self};
use std::path::PathBuf;
use std::env;
use std::process;
use std::error::Error;

const FAILURE: i32 = 1;

struct WallData<'a> {
    m_dark: &'a PathBuf,
    m_light: &'a PathBuf,
    m_name: &'a str,
}

fn main() {
    let mut path_dark = PathBuf::new();
    match input_path("Enter Path to Dark Wallpaper", &mut path_dark) {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e);
            process::exit(FAILURE);
        },
    };
    match verify_image(&path_dark) {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e);
            process::exit(FAILURE);
        }
    }

    let mut path_light = PathBuf::new();
    match input_path("Enter Path to Light Wallpaper", &mut path_light) {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e);
            process::exit(FAILURE);
        },
    };
    match verify_image(&path_light) {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e);
            process::exit(FAILURE);
        }
    }

    println!("Enter Name of the Wallpaper");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .unwrap_or_else(|err| {
            println!("Error: {}", err);
            process::exit(FAILURE);
        });

    let wall = WallData {
        m_dark: &path_dark,
        m_light: &path_light,
        m_name: &name,
    };
    
    match wall.make() {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e);
            process::exit(FAILURE);
        }
    }
}

//takes input path and returns a PathBuf
fn input_path<'a>(question: &str, path_obj: &'a mut PathBuf) -> Result<(), Box<dyn Error>> {
    println!("{}", question);

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    buf = buf.trim().to_string();

    path_obj.clear();
    *path_obj = PathBuf::from(buf);
    Ok(())
}

//checks whether the provided path is valid and leads to a jpg, jpeg or png
fn verify_image(path_obj: &PathBuf) -> Result<(), &'static str> { 
    if !path_obj.exists() {
        Err("File doesnt exist")
    } else {
        if let Some(t) = path_obj.extension() {
            if let Some(x) = t.to_str() {
                match x {
                    "png" | "jpg" | "jpeg" => Ok(()),
                    _ => Err("Not a supported file format"),
                }
            }
            else {
                Err("Empty string returned")
            }
        } else {
            Err("No file extension found")
        }
    }
}

impl<'a> WallData<'a> {
    //verifies that the struct is ready to be used 
    fn verify(&self) -> Result<(), Box<dyn Error>> {

        let home = env::var("HOME")?;

        let temp = format!("{}/.local/share/gnome-background-properties", home);
        let name = self.m_name;
        let xml_path = &PathBuf::from(temp);

        // match xml_path.exists() {
        //     true => (),
        //     false => if let Err(e) = DirBuilder::new().create(xml_path) {
        //         Err(e)
        //     }
        // }
        if !xml_path.exists() {
            DirBuilder::new().create(xml_path)?
        }
        // match std::fs::read_dir(xml_path) { // rewriting because
        //     Ok(f) => {                      // shit code
        //         for i in f {
        //             match i {
        //                 Ok(t) => {
        //                     if let Some(t) = t.file_name().to_str() {
        //                         if t == format!("{}.xml", name.trim()) {
        //                             Err("Wallpaper with the name already exists")
        //                         }
        //                     } else {
        //                         ()
        //                     }
        //                 },
        //                 Err(_e) => ()
        //             }
        //         }
        //     }
        //     Err(e) => Err(e),
        // }
        let f = fs::read_dir(xml_path)?;
        for i in f {
            let j = i?;
            if let Some(t) = j.file_name().to_str() {
                if format!("{}.xml", name.trim()) == t {
                    Err(format!("Wallpaper named {} already exists", name.trim()).as_str())?
                }
            }
        }
        Ok(())
    }

    pub fn make(self) -> Result<(), Box<dyn Error>>{
        if let Err(e) = self.verify() {
            return Err(e);
        }
        let xml_data = format!("<?xml version=\"1.0\" encoding=\"UTF-8\" ?><!DOCTYPE wallpapers SYSTEM \"gnome-wp-list.dtd\"><wallpapers><wallpaper deleted=\"false\"><name>{}</name><filename>{}</filename><filename-dark>{}</filename-dark><options>zoom</options><shade_type>solid</shade_type><pcolor>#3465a4</pcolor><scolor>#000000</scolor></wallpaper></wallpapers>", self.m_name, self.m_light.to_str().expect("Couldnt convert m_light to str"), self.m_dark.to_str().expect("Couldnt convert m_dark to str"));
        let home = env::var("HOME")?;
        let xml_path = PathBuf::from(format!("{}/.local/share/gnome-background-properties/{}.xml", home, self.m_name.trim()));
        let mut xml = fs::File::create(xml_path).expect("Couldnt create the xml");
        xml.write_all(xml_data.as_bytes())?;
        Ok(())
    }
}