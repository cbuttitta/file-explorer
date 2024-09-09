use owo_colors::OwoColorize;
use std::{env::current_dir, fs, path::Path,ffi::OsStr};
fn main() {
    println!("{}", std::env::current_dir().unwrap().display().to_string().on_green());
    match load_dir(Path::new(std::env::current_dir().unwrap().as_os_str())) {
        Ok(files) => read_out(files),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("{}", "directory not found".on_red());
            }
        }
    }
}
fn load_dir(directory: &Path) -> Result<fs::ReadDir, std::io::Error> {
    let files = fs::read_dir(directory)?;
    Ok(files)
}
fn read_out(files: fs::ReadDir) {
    for file in files {
        let dash = "";
        let file_access = file.unwrap();
        if file_access.path().is_file() {
            let localized_path = localize(file_access.path().display().to_string().as_str());
            let file_extension = file_access.path().extension().unwrap_or(OsStr::new("none")).to_str().unwrap().to_uppercase();
            let file_string = format!("{}{}{}   {}: {}", "│ ".repeat(localized_path.1-1),"├───", dash, file_extension.bold(),localized_path.0.red());
            println!("{}",file_string.red());
        } else if file_access.path().is_dir() {
            let localized_path = localize(file_access.path().display().to_string().as_str());
            let dir_string = format!("{}{}{} {}", "│ ".repeat(localized_path.1), "├──", dash,localized_path.0);
            println!("{}",dir_string.green());
            match load_dir(Path::new(file_access.path().display().to_string().as_str())) {
                Ok(files) => read_out(files),
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        println!("{}", "directory not found".on_red());
                    }
                }
            }
        }
        else if file_access.path().is_symlink() {
            let localized_path = localize(file_access.path().display().to_string().as_str());
            let symlink_string = format!("{} {}", dash.repeat(localized_path.1),localized_path.0);
            println!("{}",symlink_string.yellow());
        }
    }
}
fn localize(absolute_path: &str) -> (String,usize){
    let indices: Vec<usize> = absolute_path.replace(current_dir().unwrap().display().to_string().as_str(),"").match_indices("/").map(|(i, _)|i).collect();
    let layers = indices.len();
    //let layers = indices.len();
    (absolute_path.replace(std::env::current_dir().unwrap().display().to_string().as_str(),""),layers)
}