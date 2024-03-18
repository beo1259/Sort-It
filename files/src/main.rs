extern crate directories;
use mime_guess::{from_ext, mime::STAR_STAR};
use std::{
    collections::HashMap,
    env,
    fs::{self, create_dir},
    path::{ PathBuf},
};

extern crate mime;
extern crate mime_guess;


fn get_current_working_dir() -> PathBuf {
    env::current_dir().unwrap()
}

fn create_folders(dirs_to_make: [&PathBuf; 10]) {
    for d in dirs_to_make {
        match create_dir(&d) {
            Ok(_) => {}  // pass here cuz successful
            Err(_) => {} // otherwise tellem wassup
        }
    }
}

fn sort_dir(
    dir: PathBuf,
    types: HashMap<&PathBuf, &str>,
    complex_types: HashMap<&PathBuf, Vec<&str>>,
) {
    let files = fs::read_dir(&dir).unwrap();

    for file in files {
        let cur_file = file.unwrap().path();

        let mut mime_type = from_ext("dummy.txt").first_or_octet_stream(); // placeholder

        let suffix = cur_file.file_name().unwrap().to_str().unwrap();
        let suffix_path = PathBuf::from(suffix);

        match cur_file.extension().and_then(std::ffi::OsStr::to_str) {
            Some(ext) => {
                //println!("File has an extension: {}", ext);
                mime_type = from_ext(ext).first_or(STAR_STAR);
                if &mime_type == &STAR_STAR {
                    if !cur_file.is_dir() {
                        let new_location: PathBuf = [&dir, &PathBuf::from("ٴOTHER"), &suffix_path]
                            .iter()
                            .collect();

                        match fs::rename(&cur_file, &new_location) {
                            Ok(_) => {
                                continue;
                            }
                            Err(_) => {
                                continue;
                            }
                        }
                    }
                }
            }
            None => {
                if !cur_file.is_dir() {
                    let new_location: PathBuf = [&dir, &PathBuf::from("ٴOTHER"), &suffix_path]
                        .iter()
                        .collect();

                    match fs::rename(&cur_file, &new_location) {
                        Ok(_) => {
                            break;
                        }
                        Err(_) => {
                            break;
                        }
                    }
                }
            } //println!("File does not have an extension or it's not representable as a &str");
        }

        // complex mime types
        for (&key, value) in &complex_types {
            if !cur_file.is_dir() && value.contains(&mime_type.to_string().as_str()) {
                let new_location: PathBuf = [&dir, &key, &suffix_path].iter().collect();

                match fs::rename(&cur_file, &new_location) {
                    Ok(_) => {
                        break;
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        }

        // simple mime types
        for (&key, value) in &types {
            // complex mime types
            if !&cur_file.is_dir() && mime_type.to_string().starts_with(&*value) {
                let new_location: PathBuf = [&dir, &key, &suffix_path].iter().collect();

                match fs::rename(&cur_file, &new_location) {
                    Ok(_) => {
                        break;
                    }
                    Err(_) => {
                        break;
                    }
                }
            }

            // file folder types
            if cur_file.is_dir()
                && !types.contains_key(&cur_file)
                && !complex_types.contains_key(&cur_file)
            {
                let new_location: PathBuf = [&dir, &PathBuf::from("_FOLDERS"), &suffix_path]
                    .iter()
                    .collect();

                match fs::rename(&cur_file, &new_location) {
                    Ok(_) => {
                        break;
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        }
        }
        println!("\n{:?} is now sorted!\n\nIf anything is still unsorted, try moving those files manually and see what the issue is (file open somewhere else, lacking permissions, etc.)!", get_current_working_dir().to_str().unwrap())

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

    let simple_types: HashMap<&PathBuf, &str> = HashMap::from([
        (&audio, "audio"),
        (&video, "video"),
        (&image, "image"),
        (&text, "text"),
        (&apps, "application"),
        // not real up mimetypes for placholder
        (&folders, "folder"),
        (&other, "other"),
    ]);

    let zip_mimetypes: Vec<&str> = vec![
        "application/vnd.rar",
        "application/x-rar-compressed",
        "application/zip",
        "application/x-zip-compressed",
        "multipart/x-zip",
        "application/vnd.cncf.helm.chart.content.v1.tar+gzip",
    ];

    let office_mimetypes: Vec<&str> = vec![
        "application/msword",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "application/vnd.openxmlformats-officedocument.wordprocessingml.template",
        "application/vnd.ms-word.document.macroEnabled.12",
        "application/vnd.ms-word.template.macroEnabled.12",
        "application/vnd.ms-excel",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "application/vnd.openxmlformats-officedocument.spreadsheetml.template",
        "application/vnd.ms-excel.sheet.macroEnabled.12",
        "application/vnd.ms-excel.template.macroEnabled.12",
        "application/vnd.ms-excel.addin.macroEnabled.12",
        "application/vnd.ms-excel.sheet.binary.macroEnabled.12",
        "application/vnd.ms-powerpoint",
        "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        "application/vnd.openxmlformats-officedocument.presentationml.template",
        "application/vnd.openxmlformats-officedocument.presentationml.slideshow",
        "application/vnd.ms-powerpoint.addin.macroEnabled.12",
        "application/vnd.ms-powerpoint.presentation.macroEnabled.12",
        "application/vnd.ms-powerpoint.template.macroEnabled.12",
        "application/vnd.ms-powerpoint.slideshow.macroEnabled.12",
    ];

    let pdf_mimetypes: Vec<&str> = vec!["application/pdf", "application/x-pdf"];

    let complex_types: HashMap<&PathBuf, Vec<&str>> = HashMap::from([
        (&zipped, zip_mimetypes),
        (&work, office_mimetypes),
        (&pdf, pdf_mimetypes),
    ]);


    create_folders(dirs_to_make);
    
    sort_dir(working_dir, simple_types, complex_types);

}
