use std::{
    thread,
    time,
    fs::File,
    path::Path,
    io::{
        stdout,
        self,
        Write,
        BufRead
    }
};
use crossterm::{
    ExecutableCommand,
    cursor::{
        MoveToColumn,
        MoveToRow
    },
    terminal::{
        Clear,
        ClearType
    }
};
use colored::Colorize;

// Import local modules
mod socha;
mod sfetch;

pub static OS_NAME: &str = "Xenon 1.2";

fn main() {
    boot();
    cli();
}

fn boot() {
    // Ramsay Boot screen
    println!("{} {}",
        "Welcome to".purple().bold(),
        OS_NAME.purple().bold()
    );
    println!("{}",
        "Run 'help' to get started".purple().bold()
    );

    let duration = time::Duration::from_secs(3);

    let start_time = std::time::Instant::now();
    while start_time.elapsed() < duration {
            thread::sleep(time::Duration::from_millis(100));
    }

    println!();
}

pub fn clear() {
    let mut stdout = stdout();
    let _ = stdout.execute(MoveToColumn(0));            // Move to left-most column
    let _ = stdout.execute(MoveToRow(0));               // Move to top-most row
    let _ = stdout.execute(Clear(ClearType::All));      // Clear screen
    let _ = stdout.execute(Clear(ClearType::Purge));    // Clear history
}

// Parse file info
fn get_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;       // Open file
    Ok(io::BufReader::new(file).lines())    // Put file lines into a buffer?
}

// Output file info
fn read_lines(file: &str) {
    if let Ok(lines) = get_lines(file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            println!("{}", line.bold().purple());
        }
    }
}

pub fn cli() {
    loop {
        // Display a prompt for the user
        print!("{}>", "Xenon".bold().purple());
        io::stdout().flush().unwrap();

        // Read the user's input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Remove the newline character from the input
        let input = input.trim();

        // Parse arguments
        let mut input_pieces = input.split_whitespace();
        let command = input_pieces.next().unwrap_or("");

        // Execute the command
        match command {
            "help" => {
                read_lines("help.txt");
            }
            "clr" => {
                clear();
            }
            "say" => {
                let text = input_pieces.collect::<Vec<&str>>().join(" ");

                println!("{}", text);
                io::stdout().flush().unwrap();
            }
            "sfetch" => {
                sfetch::run();
            }
            "socha" => {
                let _ = socha::run();
            }
            "credits" => {
                read_lines("credits.txt");
            }
            "shutdwn" => {
                println!("Goodbye!...");
                std::process::exit(0);
            }
            _ => {
                println!("Command not recognized. Type 'help' for a list of commands.");
            }
        }
    }
}
