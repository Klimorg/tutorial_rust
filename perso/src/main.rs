use error_chain::error_chain;
use glob::{glob_with, MatchOptions};
use std::{fs, io};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        SystemTimeError(std::time::SystemTimeError);
        Glob(glob::GlobError);
        Pattern(glob::PatternError);
    }
}

fn main() -> Result<()> {
    let current_dir = give_path_dir();
    let extension = "/**/*.png";

    println!(
        "Entries found in {:?} with the following extension {:?}:",
        current_dir, extension
    );

    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    let files_wanted = add_extension(current_dir, extension);

    for entry in glob_with(&files_wanted, options).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }

    // for entry in fs::read_dir(&current_dir)? {
    //     let entry = entry?;
    //     let path = entry.path();

    //     let metadata = fs::metadata(&path)?;
    //     println!(
    //         "is read only: {:?}, size: {:?} bytes, filename: {:?}",
    //         metadata.permissions().readonly(),
    //         metadata.len(),
    //         path.file_name().ok_or("No filename")?
    //     );
    //let last_modified = metadata.modified()?.elapsed()?.as_secs();

    // if last_modified < 24 * 3600 && metadata.is_file() {
    //     println!(
    //         "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
    //         last_modified,
    //         metadata.permissions().readonly(),
    //         metadata.len(),
    //         path.file_name().ok_or("No filename")?
    //     );
    // }
    // }

    Ok(())
}

fn give_path_dir() -> String {
    println!("Please enter the directory path you want to search");

    let mut current_dir = String::new();

    io::stdin()
        .read_line(&mut current_dir)
        .expect("Nothing to read !");

    current_dir = current_dir.trim().parse().expect("No path found !"); // the trim() function return a slice of a string, so you have to parse it again to get a string

    return current_dir;
}

fn add_extension(mut current_dir: String, extension: &str) -> String {
    current_dir.push_str(extension);

    println!("{:?}", current_dir);

    return current_dir;
}

// step1 : give the dir_path, if none, get current directory
// step2 : Find all files with given pattern ignoring filename case.

// use error_chain::error_chain;
// use glob::{glob_with, MatchOptions};

// fn main() -> Result<()> {
// let options = MatchOptions {
//     case_sensitive: false,
//     ..Default::default()
// };

// for entry in glob_with("/media/img_[0-9]*.png", options)? {
//     println!("{}", entry?.display());
// }

//     Ok(())
// }
