use std::{
    env,
    fs,
    io,
    path::PathBuf
};
use crossterm::{
    event,
    event::KeyCode
};
use crate::clear;

// File manager function
pub fn run() -> io::Result<()> {
    let mut current_dir = env::current_dir()?;

    loop {
        // Clear the terminal
        clear();

        // Show the current directory
        println!("Current Directory: {}", current_dir.display());
        println!("-----------------------------");

        // List files and directories in the current directory
        let entries: Vec<_> = fs::read_dir(&current_dir)?.collect::<Result<Vec<_>, _>>()?;
        let mut dirs: Vec<String> = Vec::new();
        let mut files: Vec<String> = Vec::new();

        for entry in &entries {
            let entry_name = entry.file_name().to_string_lossy().to_string();
            if entry.metadata()?.is_dir() {
                dirs.push(entry_name);
            } else {
                files.push(entry_name);
            }
        }

        // Display numbered options
        println!("Directories:");
        for (i, dir) in dirs.iter().enumerate() {
            println!("{}: {}", i + 1, dir);
        }
        println!("Files:");
        for (i, file) in files.iter().enumerate() {
            println!("{}: {}", i + 1, file);
        }

        // Display available actions
        println!("\n1: Change Directory | 2: Remove File | 3: Rename File | 4: Copy File | e: Exit");

        if event::poll(std::time::Duration::from_secs(1))? {
            if let event::Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('1') => change_directory(&mut current_dir, &entries)?,
                    KeyCode::Char('2') => remove_file(&entries)?,
                    KeyCode::Char('3') => rename_file(&entries)?,
                    KeyCode::Char('4') => copy_file(&entries)?,
                    KeyCode::Char('e') => break,
                    _ => {}
                }
            }
        }
    }

    fn change_directory(current_dir: &mut PathBuf, entries: &[fs::DirEntry]) -> io::Result<()> {
        println!("Select a directory to enter, or press '.' to move up:");
    
        for (index, entry) in entries.iter().enumerate() {
            if entry.path().is_dir() {
                println!("{}: {}", index + 1, entry.file_name().to_string_lossy());
            }
        }
        println!(".: Go up a directory");
    
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
    
        if input == "." {
            *current_dir = current_dir.parent().unwrap_or(current_dir).to_path_buf();
            return Ok(());
        }
    
        let selected_index: usize = match input.parse::<usize>() {
            Ok(num) if num > 0 && num <= entries.len() => num - 1, // Convert to 0-based index
            _ => {
                println!("Invalid selection.");
                return Ok(());
            }
        };
    
        let selected_entry = &entries[selected_index];
        if selected_entry.path().is_dir() {
            *current_dir = selected_entry.path().to_path_buf();
        } else {
            println!("Selected item is not a directory.");
        }
    
        Ok(())
    }

    fn remove_file(entries: &[fs::DirEntry]) -> io::Result<()> {
        println!("Enter the number of the file to delete:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if let Ok(index) = input.trim().parse::<usize>() {
            if let Some(entry) = entries.get(index - 1) {
                let path = entry.path();
                if path.is_file() {
                    fs::remove_file(path)?;
                    println!("File removed.");
                } else {
                    println!("Not a file!");
                }
            }
        }
        Ok(())
    }
    
    fn rename_file(entries: &[fs::DirEntry]) -> io::Result<()> {
        println!("Select the number of the file/folder to rename:");
    
        for (index, entry) in entries.iter().enumerate() {
            println!("{}: {}", index + 1, entry.file_name().to_string_lossy());
        }
    
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
    
        if input.is_empty() {
            println!("No selection made.");
            return Ok(());
        }
    
        let selected_index: usize = match input.parse::<usize>() {
            Ok(num) if num > 0 && num <= entries.len() => num - 1, // Convert to 0-based index
            _ => {
                println!("Invalid selection.");
                return Ok(());
            }
        };
    
        let selected_entry = &entries[selected_index];
        println!("Enter new name:");
    
        let mut new_name = String::new();
        io::stdin().read_line(&mut new_name)?;
        let new_name = new_name.trim();
    
        if new_name.is_empty() {
            println!("No new name provided.");
            return Ok(());
        }
    
        let old_path = selected_entry.path();
        let new_path = old_path.with_file_name(new_name);
    
        fs::rename(old_path, new_path)?;
    
        println!("Renamed successfully.");
        Ok(())
    }
    
    
    fn copy_file(entries: &[fs::DirEntry]) -> io::Result<()> {
        println!("Enter the number of the file to copy:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if let Ok(index) = input.trim().parse::<usize>() {
            if let Some(entry) = entries.get(index - 1) {
                let source_path = entry.path();
                println!("Enter the destination path:");
                let mut dest_path = String::new();
                io::stdin().read_line(&mut dest_path)?;
                let dest_path = dest_path.trim();
                let destination = PathBuf::from(dest_path).join(entry.file_name());
                fs::copy(source_path, destination)?;
                println!("File copied.");
            }
        }
        Ok(())
    }
    Ok(())                  
}
