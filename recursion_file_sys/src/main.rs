use std::{env, fs};
fn main() -> std::io::Result<()>{
    let current_directory = env::current_dir()?;
    println!("File finder(version Recursionly finder) is operating");
    println!("------------------------");
    println!("Here is {}",current_directory.display());

    for entry in fs::read_dir(current_directory)? {
        let dir = entry?;
        let dir_path = dir.path();
        println!("{:?}",dir_path);

    }

    Ok(())
}


fn find_directories(directory: &str) {

}