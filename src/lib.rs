use std::fs::{DirBuilder, self};
use std::io::{Write, self};
use std::path::PathBuf;
use std::env;
use std::process;
use std::error::Error;

//allows to group the wallpaper metadata in a single object
pub struct WallData<'a> {
    dark: &'a PathBuf,
    light: &'a PathBuf,
    name: &'a str,
}

//verifies whether the provided path actually corresponds to a supported file
//to prevent unexpected results
pub fn verify_image(path_obj: &PathBuf) -> Result<(), &'static str> {
    if !path_obj.exists() { //path is actually valid
        Err("File doesnt exist")
    } else {
        if let Some(t) = path_obj.extension() { //has an extension to verify whether its a supported format
            if let Some(x) = t.to_str() { 
                match x {
                    "png" | "jpg" | "jpeg" => Ok(()), //is a supported format
                    _ => Err("Not a supported file format"),
                }
            }
            else {
                Err("Empty string returned") //shouldnt really occur but anyways
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
        let name = self.name;
        let xml_path = &PathBuf::from(temp);

        if !xml_path.exists() {
            DirBuilder::new().create(xml_path)?; //create the directory if not present
        }
        //verify no other wallpaper metadata of the same name present
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
    // after all verification proceed with making the needed files
    //TODO: Copy the wallpapers to .local/share/backgrounds/{wallpapername}/
    //in order to prevent errors if the original source images are moved
    pub fn make(self) -> Result<(), Box<dyn Error>>{
        if let Err(e) = self.verify() {
            return Err(e);
        }
        let xml_data = format!("<?xml version=\"1.0\" encoding=\"UTF-8\" ?><!DOCTYPE wallpapers SYSTEM \"gnome-wp-list.dtd\"><wallpapers><wallpaper deleted=\"false\"><name>{}</name><filename>{}</filename><filename-dark>{}</filename-dark><options>zoom</options><shade_type>solid</shade_type><pcolor>#3465a4</pcolor><scolor>#000000</scolor></wallpaper></wallpapers>", self.name, self.light.to_str().expect("Couldnt convert light to str"), self.dark.to_str().expect("Couldnt convert dark to str"));
        let home = env::var("HOME")?;
        let xml_path = PathBuf::from(format!("{}/.local/share/gnome-background-properties/{}.xml", home, self.name.trim()));
        let mut xml = fs::File::create(xml_path).expect("Couldnt create the xml");
        xml.write_all(xml_data.as_bytes())?;
        Ok(())
    }
}

//sort of a bootstrapper
pub fn run(args: Vec<String>) { 
    let path_dark = PathBuf::from(&args[1]);
    match verify_image(&path_dark) {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    }

    let path_light = PathBuf::from(&args[2]);
    match verify_image(&path_light) {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    }

    let wall = WallData {
        dark: &path_dark,
        light: &path_light,
        name: &args[3],
    };

    match wall.make() {
        Ok(_) => println!("Done! You can set the wallpaper now"),
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    }
}