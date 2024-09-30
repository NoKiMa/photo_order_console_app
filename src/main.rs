use chrono::{DateTime, TimeZone, Utc};

use std::fs;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct ListItem {
    metadata: i64,
    file_path: String,
}

impl ListItem {
    fn new(metadata: i64, file_path: String) -> ListItem {
        ListItem {
            metadata: metadata,
            file_path: file_path,
        }
    }
}

fn list_files_in_folder(folder_path: &str) -> io::Result<Vec<ListItem>> {
    let paths = fs::read_dir(folder_path)?;
    let mut file_list: Vec<ListItem> = Vec::new();

    for path in paths {
        let entry = path?;
        let metadata = entry.metadata()?;

        // Check if the entry is a file
        if metadata.is_file() {
            // Try to get the creation time of the file
            if let Ok(system_time) = metadata.created() {
                let creation_time = system_time
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_else(|_| SystemTime::now().duration_since(UNIX_EPOCH).unwrap())
                    .as_secs() as i64;

                // Create a new ListItem and push to the list
                let list_item = ListItem::new(creation_time, entry.path().display().to_string());
                file_list.push(list_item);
            } else {
                // Handle platforms where creation time might not be available
                println!(
                    "Creation time not available for file: {}",
                    entry.path().display()
                );
            }
        }
    }
    Ok(file_list)
}

fn remove_quotes(input: &str) -> String {
    let trimmed_input = input.trim();

    // Check if input starts and ends with single quotes
    if trimmed_input.starts_with('\'') && trimmed_input.ends_with('\'') {
        // Remove the first and last characters (the quotes)
        return trimmed_input[1..trimmed_input.len() - 1].to_string();
    }

    // Return the input as-is if no quotes were found
    trimmed_input.to_string()
}

fn check_extension(file_name: &str) -> bool {
    // Convert the file name to uppercase
    let upper_case_file_name = file_name.to_uppercase();

    // Check for valid extensions
    upper_case_file_name.contains(".JPG")
        || upper_case_file_name.contains(".JPEG")
        || upper_case_file_name.contains(".PNG")
        || upper_case_file_name.contains(".PDF")
        || upper_case_file_name.contains(".JS")
}

fn convert_sec_to_ymd(seconds: i64) -> String {
    // Create a NaiveDateTime from the seconds since UNIX epoch
    let naive_datetime: DateTime<Utc> = Utc.timestamp_opt(seconds, 0).single().expect("Invalid timestamp");

    // Format the date as YYYY.MM.DD
    let formatted_date = naive_datetime.format("%Y.%m.%d").to_string();

    formatted_date
}

fn create_dir_if_not_exists(dir_path: &String) -> std::io::Result<()> {
    // Try to create the directory
    match fs::create_dir(dir_path) {
        Ok(_) => {
            println!("Directory created at: {}", dir_path);
        }
        Err(e) => {
            // Check if the error is that the directory already exists
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                println!("Directory already exists at: {}", dir_path);
            } else {
                // If it's another error, return it
                return Err(e);
            }
        }
    }
    Ok(())
}

fn move_file(source: &str, destination: &str) -> io::Result<()> {
    // Use fs::rename to move the file from source to destination
    fs::rename(source, destination)?;
    println!("Moved file from {} to {}", source, destination);
    Ok(())
}

fn file_hendler(file_list: Vec<ListItem>, parent_folder_path: &str) {
    for file in file_list {
        let file_path: &str = &file.file_path[..];
        if check_extension(&file_path) {
            let folder_name: String = convert_sec_to_ymd(file.metadata);
            let child_folder_path = format!("{}/{}", parent_folder_path, folder_name);
            let _ = create_dir_if_not_exists(&child_folder_path);
            let source = file_path;
            let destination = &format!(
                "{}/{}",
                child_folder_path,
                file_path.split("/").last().unwrap()
            )[..];
            let _ = move_file(source, &destination);
        }
    }
}

fn photo_order() {
    let mut attempt = 0;

    loop {
        // Prompt the user for input
        println!("Drag and drop needed folder to terminal to get the path.");
        println!("Or copy path manualy.");
        println!("The path needs to start sorting files in the source folder.");
        println!("Let's do it:");
        io::stdout().flush().unwrap(); // Ensure prompt is shown

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Remove any trailing newline or spaces
        let input = remove_quotes(&input);

        if !input.is_empty() {
            // Try to list files in the folder and handle potential errors
            match list_files_in_folder(&input) {
                Ok(file_list) => {
                    println!("Successfully listed files in folder.");
                    file_hendler(file_list, &input);
                }
                Err(e) => {
                    println!("Failed to read folder. Error: {}", e);
                    attempt += 1;
                    if attempt == 2 {
                        println!("Goodbye");
                        break;
                    } else {
                        println!("Invalid path, try again.");
                        continue;
                    }
                }
            }
            break;
        } else {
            attempt += 1;
            if attempt == 2 {
                println!("Goodbye");
                break;
            } else {
                println!("Input was empty, try again.");
            }
        }
    }
}

fn main() {
    photo_order();
}
