use std::fs;
use std::path::Path;

fn print_tree<P: AsRef<Path>>(path: P, prefix: &str) -> std::io::Result<()> {
    let metadata = fs::metadata(&path)?;
    if metadata.is_dir() {
        let mut entries = fs::read_dir(&path)?;

        while let Some(entry) = entries.next() {
            let entry = entry?;
            let child_path = entry.path();
            let child_name = entry.file_name().to_string_lossy();

            let mut new_prefix = String::from(prefix);
            new_prefix.push_str("├── ");

            if let Some(entry) = entries.next() {
                println!("{}{}{}", prefix, "├── ", child_name);
                print_tree(child_path, &new_prefix)?;
            } else {
                println!("{}{}{}", prefix, "└── ", child_name);
                print_tree(child_path, &new_prefix)?;
            }
        }
    }
    Ok(())
}

fn main() {
    let root_path = "."; // Change this to the directory you want to start printing from
    let prefix = "";

    if let Err(err) = print_tree(root_path, prefix) {
        eprintln!("Error: {}", err);
    }
}
