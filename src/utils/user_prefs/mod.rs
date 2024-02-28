//  use std::fs::{self, File};
//  use std::io::{self, Read, Write};
//  use std::path::PathBuf;
// use dirs::home_dir;

// // const PREFS_FILE: &str = ".stargaze_prefs.txt";

// // fn get_prefs_path() -> io::Result<PathBuf> {
// //     let mut path = home_dir().ok_or(io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
// //     path.push(PREFS_FILE);
// //     Ok(path)
// // }

pub fn delete_prefs() -> io::Result<()> {
    let path = get_prefs_path()?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}



// pub fn save_city(city: &str) -> io::Result<()> {
//     let path = get_prefs_path()?;
//     let mut file = File::create(path)?;
//     file.write_all(city.as_bytes())?;
//     Ok(())
// }

// pub fn load_city() -> io::Result<Option<String>> {
//     let path = get_prefs_path()?;
//     if path.exists() {
//         let mut file = File::open(path)?;
//         let mut city = String::new();
//         file.read_to_string(&mut city)?;
//         Ok(Some(city))
//     } else {
//         Ok(None)
//     }
// }

// in user_prefs.rs

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use dirs::home_dir;

const PREFS_FILE: &str = ".stargaze_prefs.txt";

fn get_prefs_path() -> io::Result<PathBuf> {
    let mut path = home_dir().ok_or(io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
    path.push(PREFS_FILE);
    Ok(path)
}

pub fn save_city(city: &str) -> io::Result<()> {
    let path = get_prefs_path()?;
    let mut file = File::create(path)?;
    file.write_all(city.as_bytes())?;
    Ok(())
}

pub fn load_city() -> io::Result<Option<String>> {
    let path = get_prefs_path()?;
    if path.exists() {
        let mut file = File::open(path)?;
        let mut city = String::new();
        file.read_to_string(&mut city)?;
        Ok(Some(city))
    } else {
        Ok(None)
    }
}