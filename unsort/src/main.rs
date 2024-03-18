extern crate directories;
use std::{
    env, fs::{self}, io, path::PathBuf
};

fn get_current_working_dir() -> PathBuf {
    env::current_dir().unwrap()
}


fn remove_dir_if_empty(dir: &PathBuf) -> io::Result<()> {
    fs::remove_dir(dir)?;
    Ok(())
}

fn unsort(sorted_dirs: [&PathBuf; 10], working_dir: PathBuf){
    for dir in sorted_dirs{
        if dir.exists(){
            let files = fs::read_dir(&dir).unwrap();

            for file in files{
                let cur_file = file.unwrap().path();

                let suffix = cur_file.file_name().unwrap().to_str().unwrap();
                let suffix_path = PathBuf::from(suffix);

                let old_location: PathBuf = [&working_dir, &suffix_path].iter().collect();

                match fs::rename(&cur_file, old_location){
                    Ok(_) => {continue;},
                    Err(_) => {continue;}
                }
            }
        }
    }

    for dir in sorted_dirs{
        match remove_dir_if_empty(dir){
            Ok(_) => {continue;},
            Err(_) => {continue;},
        }
    }

    println!("\n{:?} is no longer sorted!\n\nIf anything remains sorted, try moving those files yourself and see what the issue is (file open somewhere else, lacking permissions, etc.)!", get_current_working_dir().to_str().unwrap())

}

fn main() {
    let working_dir = get_current_working_dir();    

    // simple mimetypes directories
    let audio: PathBuf = [&working_dir, &PathBuf::from("Audio")].iter().collect();
    let video: PathBuf = [&working_dir, &PathBuf::from("Video")].iter().collect();
    let image: PathBuf = [&working_dir, &PathBuf::from("Images")].iter().collect();
    let text: PathBuf = [&working_dir, &PathBuf::from("Text & Code")].iter().collect();
    let apps: PathBuf = [&working_dir, &PathBuf::from("Apps")].iter().collect();

    // complex mimetypes firectories
    let zipped: PathBuf = [&working_dir, &PathBuf::from("Zipped")].iter().collect();
    let work: PathBuf = [&working_dir, &PathBuf::from("Work")].iter().collect();
    let pdf: PathBuf = [&working_dir, &PathBuf::from("PDF")].iter().collect();

    //  folder/other type of files
    let folders: PathBuf = [&working_dir, &PathBuf::from("_FOLDERS")].iter().collect();
    let other: PathBuf = [&working_dir, &PathBuf::from("ٴOTHER")].iter().collect();

    let dirs_to_make: [&PathBuf; 10] = [
        &audio, &video, &image, &folders, &text, &other, &apps, &zipped, &work, &pdf,
    ];

    unsort(dirs_to_make, working_dir);

}
