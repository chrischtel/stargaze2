
// use std::fs::File;
// use std::io::{self, Read, Write};
// use std::path::Path;

// const PREFS_FILE: &str = "prefs.txt";

// pub fn save_city(city: &str) -> io::Result<()> {
//     let mut file = File::create(PREFS_FILE)?;
//     file.write_all(city.as_bytes())?;
//     Ok(())
// }

// pub fn load_city() -> io::Result<Option<String>> {
//     let path = Path::new(PREFS_FILE);
//     if path.exists() {
//         let mut file = File::open(PREFS_FILE)?;
//         let mut city = String::new();
//         file.read_to_string(&mut city)?;
//         Ok(Some(city))
//     } else {
//         Ok(None)
//     }
// }

use std::fs::File;
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