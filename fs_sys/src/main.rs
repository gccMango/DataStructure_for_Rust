use std::fs;
use std::path::Path;

fn main() {
    let mut num:usize= 2;
    find_directories("./search_test", &mut num);
}

fn find_directories<T>(directory: T, num: &mut usize) -> std::io::Result<()>
where
    T: AsRef<Path>,
{
    println!("Calling number {num:?}");
    for entry in fs::read_dir(directory)? {
        // input "." current directory
        // show subdirectory just Top level(1-level)
        let dir = entry?;

        // it will show the .\src, .\Cargo.toml ..etc
        // find_directories("")
        println!("directory name =>{}", dir.path().display());
        *num +=1;
        find_directories(dir.path().as_path(), num );
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
