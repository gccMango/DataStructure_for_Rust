use std::fs;
use std::path::Path;
fn main() {

    find_directories("./search_test");
}

fn find_directories<T>(directory: T) -> std::io::Result<()>
where
    T: AsRef<Path>
{
    for entry in fs::read_dir(directory)? {
        // input "." current directory
        // show subdirectory just Top level(1-level)
        let dir = entry?;
        println!("{}",dir.path().display());
        // it will show the .\src, .\Cargo.toml ..etc
        // find_directories("")
        find_directories(dir.path().as_path());
    }

    Ok(())
}

// fn main() {
//     if let Ok(entries) = fs::read_dir(".") {
//         for entry in entries {
//             if let Ok(entry) = entry {
//                 // Here, `entry` is a `DirEntry`.
//                 if let Ok(file_type) = entry.file_type() {
//                     // Now let's show our entry's file type!
//                     println!("{:?}: {:?}", entry.path(), file_type);
//                 } else {
//                     println!("Couldn't get file type for {:?}", entry.path());
//                 }
//             }
//         }

// }
// }