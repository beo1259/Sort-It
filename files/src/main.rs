extern crate directories;
use chrono::{DateTime, Local, TimeZone, Utc};
use directories::UserDirs;
use mime_guess::{from_ext, mime::STAR_STAR};
use std::{
    collections::HashMap,
    env,
    fs::{self, create_dir},
    path::{Path, PathBuf},
};

extern crate mime;
extern crate mime_guess;

extern crate chrono;

fn get_current_working_dir() -> PathBuf {
    env::current_dir().unwrap()
}

//     let dirs_to_make: [&PathBuf; 10] = [&audio, &video, &image, &folders, &text, &other, &apps, &zipped, &work, &pdf,];

// fn find_key_for_value<'a>(map: &'a HashMap<&PathBuf, &str>, value: &str) -> Option<&'a i32> {
//     map.iter()
//         .find_map(|(key, &val)| if val == value { Some(key) } else { None })
// }

//     let dirs_to_make: [&PathBuf; 10] = [&audio, &video, &image, &folders, &text, &other, &apps, &zipped, &work, &pdf,];

fn create_folders(
    dir: &PathBuf,
    types: &HashMap<&PathBuf, &str>,
    complex_types: &HashMap<&PathBuf, Vec<&str>>,
) {
    let files = fs::read_dir(&dir).unwrap();

    let simple_values: Vec<&str> = types.values().cloned().collect();
    let complex_values: Vec<Vec<&str>> = complex_types.values().cloned().collect();

    let other_dir: PathBuf = [&dir, &&PathBuf::from("ٴOTHER")].iter().collect();

    if !other_dir.exists() {
        match create_dir(other_dir) {
            Ok(_) => {}  // pass here cuz successful
            Err(_) => {} // otherwise tellem wassup
        }
    }

    for file in files {
        let cur_file = file.unwrap().path();

        if cur_file.is_dir() {
            let folders_dir: PathBuf = [&dir, &&PathBuf::from("_FOLDERS")].iter().collect();

            if !folders_dir.exists() {
                match create_dir(folders_dir) {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
            continue;
        }

        match cur_file.extension().and_then(std::ffi::OsStr::to_str) {
            Some(ext) => {
                let cur_type = from_ext(ext).first_or(STAR_STAR);
                let dir_val = simple_values
                    .iter()
                    .find(|&value| cur_type.as_ref().starts_with(value));
                let dir_val_complex = complex_values
                    .iter()
                    .find(|&value| value.contains(&cur_type.as_ref()));

                // LOGIC FOR COMPLEX MIME_TYPES
                if complex_values
                    .iter()
                    .any(|value: &Vec<&str>| value.contains(&cur_type.as_ref()))
                {
                    let value_of_complex_dir = dir_val_complex.unwrap();
                    let desired_complex_key = complex_types.iter().find_map(|(key, val)| {
                        if val == value_of_complex_dir {
                            Some(key)
                        } else {
                            None
                        }
                    });
                    if !desired_complex_key.unwrap().exists() {
                        match create_dir(&desired_complex_key.unwrap()) {
                            Ok(_) => {}  // pass here cuz successful
                            Err(_) => {} // otherwise tellem wassup
                        }
                    }
                    continue;
                }

                // LOGIC FOR SIMPLE MIME_TYPES
                if simple_values
                    .iter()
                    .any(|&value| cur_type.as_ref().starts_with(value))
                {
                    let value_of_dir = dir_val.unwrap();
                    let desired_key = types.iter().find_map(|(&key, &val)| {
                        if &val == value_of_dir {
                            Some(key)
                        } else {
                            None
                        }
                    });
                    if !&desired_key.unwrap().exists() {
                        match create_dir(&desired_key.unwrap()) {
                            Ok(_) => {}  // pass here cuz successful
                            Err(_) => {} // otherwise tellem wassup
                        }
                    }
                }
                continue;
            }
            None => {}
        }
    }
}

fn file_already_exists_add_date(dir: &PathBuf, name: &PathBuf, key: &PathBuf) -> PathBuf {
    let suffix = name.file_name().unwrap().to_str().unwrap();
    let suffix_path: PathBuf = PathBuf::from(suffix);

    let date_time = Local::now();
    let dt_formatted = format!("{}", date_time.format("%Y-%m-%d %H%M%S"));

    let no_ext = Path::file_stem(&suffix_path).unwrap().to_str().unwrap();

    let mut new_location: PathBuf = PathBuf::from("placeholder");

    match name.extension().and_then(std::ffi::OsStr::to_str) {
        Some(ext) => {
            let extension_str = name.extension().unwrap().to_str().unwrap();

            let end_of_path = format!("{} - {}.{}", no_ext, dt_formatted, extension_str);

            new_location = [&dir, &key, &PathBuf::from(&end_of_path)].iter().collect();

            new_location
        }
        None => name.to_path_buf(),
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
        let suffix_path: PathBuf = PathBuf::from(suffix);

        // file folder types
        if cur_file.is_dir()
            && !types.contains_key(&cur_file)
            && !complex_types.contains_key(&cur_file)
        {
            let mut new_location: PathBuf = [&dir, &PathBuf::from("_FOLDERS"), &suffix_path]
                .iter()
                .collect();

            if new_location.exists() {
            
                if new_location.exists() {
                    new_location =
                        file_already_exists_add_date(&dir, &new_location, &PathBuf::from("_FOLDERS"));
                }
            }

            let final_location = new_location;

            match fs::rename(&cur_file, &final_location) {
                Ok(_) => {
                    continue;
                }
                Err(_) => {
                    continue;
                }
            }
        }

        match cur_file.extension().and_then(std::ffi::OsStr::to_str) {
            Some(ext) => {
                //println!("File has an extension: {}", ext);
                mime_type = from_ext(ext).first_or(STAR_STAR);
                if &mime_type == &STAR_STAR {
                    if !cur_file.is_dir() {
                        let mut new_location: PathBuf =
                            [&dir, &PathBuf::from("ٴOTHER"), &suffix_path]
                                .iter()
                                .collect();

                        if new_location.exists() {
                            if new_location.exists() {
                                new_location = file_already_exists_add_date(
                                    &dir,
                                    &new_location,
                                    &PathBuf::from("ٴOTHER"),
                                );
                            }
                        }

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
                    let mut new_location: PathBuf = [&dir, &PathBuf::from("ٴOTHER"), &suffix_path]
                        .iter()
                        .collect();

                    if new_location.exists() {
                        if new_location.exists() {
                            new_location = file_already_exists_add_date(
                                &dir,
                                &new_location,
                                &PathBuf::from("ٴOTHER"),
                            );
                        }
                    }

                    match fs::rename(&cur_file, &new_location) {
                        Ok(_) => {
                            continue;
                        }
                        Err(_) => {
                            continue;
                        }
                    }
                }
            } //println!("File does not have an extension or it's not representable as a &str");
        }

        // complex mime types
        for (&key, value) in &complex_types {
            if !cur_file.is_dir() && value.contains(&mime_type.to_string().as_str()) {
                let mut new_location: PathBuf = [&dir, &key, &suffix_path].iter().collect();

                if new_location.exists() {
                    if new_location.exists() {
                        new_location = file_already_exists_add_date(&dir, &new_location, key);
                    }
                }

                let final_location = new_location;

                match fs::rename(&cur_file, &final_location) {
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
            if !&cur_file.is_dir() && mime_type.to_string().starts_with(&*value) {
                let mut new_location: PathBuf = [&dir, &key, &suffix_path].iter().collect();

                if new_location.exists() {
                    new_location = file_already_exists_add_date(&dir, &new_location, key);
                }

                let final_location = new_location;

                match fs::rename(&cur_file, &final_location) {
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
    println!("\n{:?} is now sorted!\n\nIf anything is still unsorted, try moving those files manually and see what the issue is (file open somewhere else, lacking permissions, etc.)!\n", get_current_working_dir().to_str().unwrap())
}

fn main() {
    let working_dir = get_current_working_dir();

    //let working_dir = PathBuf::from("C:\\Users\\oneil\\OneDrive\\Desktop\\_FOLDERS\\Projects");

    // simple mimetypes directories
    let audio: PathBuf = [&working_dir, &PathBuf::from("Audio")].iter().collect();
    let video: PathBuf = [&working_dir, &PathBuf::from("Video")].iter().collect();
    let image: PathBuf = [&working_dir, &PathBuf::from("Images")].iter().collect();
    let text: PathBuf = [&working_dir, &PathBuf::from("Text & Code")]
        .iter()
        .collect();
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

    create_folders(&working_dir, &simple_types, &complex_types);

    sort_dir(working_dir, simple_types, complex_types);
}
