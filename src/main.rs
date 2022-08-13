use core::panic;
use std::{fs, path::PathBuf};
use time::OffsetDateTime;
use std::path::Path;
use std::ffi::OsStr;

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

fn main() {
    let dir_path = std::env::args().nth(1).expect("No path given");
    println!("dir_path: {:?}", dir_path);
    match rename_contents(dir_path) {
        Ok(_) => return,
        Err(err) => panic!("{:?}", err),
    }
}

fn rename_contents(dir_path: String) -> Result<(), std::io::Error> {
    match fs::read_dir(dir_path.clone()) {
        Ok(dir) => {
            for file in dir {
                let t = file.as_ref().unwrap().file_type().unwrap();
                let file_path = file.unwrap().path().into_os_string().into_string().unwrap();
                let created_at = fs::metadata(file_path.clone())?.modified()?;
                let format = time::format_description::parse(
                    "[year]-[month]-[day] [hour]_[minute]_[second] [offset_hour \
                         sign:mandatory]_[offset_minute]_[offset_second]",
                ).unwrap();
                let formatted_creation = OffsetDateTime::from(created_at).format(&format).unwrap();
                let from = file_path;
                let extension = get_extension_from_filename(&from).unwrap();
                let to = dir_path.clone() + "/" + &formatted_creation + "." + &extension;
                if from != to {
                    fs::rename(from, to).unwrap();
                }
                println!("{:?}", formatted_creation);
            }
        },
        Err(err) => panic!("{:?}", err)
    }
    return Ok(());
}