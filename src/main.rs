extern crate clap;
extern crate termion;
extern crate dirs;

use std::path::Path;
// use std::collections::BTreeSet;
use std::fs::{self, File, OpenOptions};
use std::io::{Write, BufRead, BufReader};
// use std::io::prelude::*;
use clap::{Arg, App};
use termion::color;

fn main() {

    // Command line argument handling
    let matches = App::new("Rtodo")
        .version("0.1")
        .author("Christian Kusabs <christian@kusabs.dev")
        .about("Simple command line todo app")
        .arg(Arg::with_name("v")
            .short("v")
            .help("Sets verbose output")
        )
        .arg(Arg::with_name("add")
            .short("a")
            .long("add")
            .help("Adds a todo note")
        )
        .arg(Arg::with_name("remove")
            .short("r")
            .long("remove")
            .help("removes a note with the specified index")
        )
        .arg(Arg::with_name("todo")
            .help("The todo to be added")
            .index(1)
        )
        .get_matches();


    // Setup verbose related stuff
    let mut verbose_level: i32 = 0;

    if matches.is_present("v") {
        verbose_level = 1;
    }

    if matches.is_present("add") && matches.is_present("remove") {

        println!("{}Error: {}Both add and remove flag used.", color::Fg(color::Red), color::Fg(color::Reset));
        std::process::exit(2); 

    } else if matches.is_present("add") {

        // Check the user actually gave a todo
        if matches.is_present("todo") {

            if verbose_level == 1 {
                println!("{}Adding todo note: {}{}", color::Fg(color::Red), color::Fg(color::Reset), matches.value_of("todo").unwrap());
            }

            // Check if the directory exists, if it doesn't make it
            if !Path::new(dirs::data_dir().unwrap().join("rtodo").as_path()).exists() {
                let _directory_creation_result: Result<(), std::io::Error> = fs::create_dir(dirs::data_dir().unwrap().join("rtodo").as_path());
            }

            // Setup file to be appended too
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(dirs::data_dir().unwrap().join("rtodo").join("todo.list").as_path())
                .unwrap();

            // Append to the file
            if let Err(e) = writeln!(file, "{}", matches.value_of("todo").unwrap()) {
                eprintln!("Error: Couldn't write to file: {}", e);
            } else {
                println!("Added note.");
            }

        // User didn't give a todo
        } else {
            
            println!("{}Error: {}No todo specified with add flag present", color::Fg(color::Red), color::Fg(color::Reset));
            std::process::exit(3);

        }
    
    } else if matches.is_present("remove") {
    
        if matches.is_present("todo") {

            if verbose_level == 1 {
                println!("{}Removing todo note: {}{}", color::Fg(color::Red), color::Fg(color::Reset), matches.value_of("todo").unwrap());
            }

            // Check if the directory exists, if it doesn't make it
            if !Path::new(dirs::data_dir().unwrap().join("rtodo").as_path()).exists() {
                println!("You don't have any todo notes.");
                std::process::exit(0);
            } else if !Path::new(dirs::data_dir().unwrap().join("rtodo").join("todo.list").as_path()).exists() {
                println!("You don't have any todo notes.");
                std::process::exit(0);
            }

            // Get todo list and then delete the original file
            let contents = fs::read_to_string(dirs::data_dir().unwrap().join("rtodo").join("todo.list").as_path()).expect("can't read");
            let _result = fs::remove_file(dirs::data_dir().expect("Couldn't remove file").join("rtodo").join("todo.list").as_path());

            // New file
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(dirs::data_dir().unwrap().join("rtodo").join("todo.list").as_path())
                .unwrap();

            let mut iter = 1;
            
            // Add all lines except the one we're deleting
            for line in contents.lines() {

                if iter != matches.value_of("todo").expect("couldn't unwrap todo").parse::<i32>().expect("Unable to parse argument to string") {
                    writeln!(file, "{}", line).expect("can't write");
                }

                iter += 1;
            }
            
        } else {

            println!("{}Error: {}Please specify a todo item to delete", color::Fg(color::Red), color::Fg(color::Reset));
            std::process::exit(4);

        }

    // No add or remove flag given, display current todos
    } else {

        // Check the data directory exists
        if Path::new(dirs::data_dir().unwrap().join("rtodo").as_path()).exists() {

            if verbose_level == 1 {
                println!("{}V: data folder found{}", color::Fg(color::Green), color::Fg(color::Reset));
            }

            // Check if the todo.list file exists in the data folder
            if Path::new(dirs::data_dir().unwrap().join("rtodo").join("todo.list").as_path()).exists() {

                // TODO: Add error handling for file opening related causes here
                let file = File::open(dirs::data_dir().unwrap().join("rtodo").join("todo.list")).unwrap();
                let reader = BufReader::new(file);

                // Read the file line by line
                for (index, line) in reader.lines().enumerate() {
    
                    // TODO: Add error handling for line unwrap panicking
                    let line = line.unwrap();

                    // Print the todo item into the console
                    println!("{}{}.{} {}", color::Fg(color::Cyan), index + 1, color::Fg(color::Reset), line);

                }

            // todo.list file doesn't exist
            } else {

                if verbose_level == 1 {
                    println!("{}V: list.todo not found{}", color::Fg(color::Green), color::Fg(color::Reset));
                }

                println!("No todos found.");

            }

        // rtodo directory doesn't exist
        } else {
        
            if verbose_level == 1 {
                println!("{}V: no config folder found, creating one..{}", color::Fg(color::Green), color::Fg(color::Reset));                
            }

            // TODO: Add error handling here
            let _directory_creation_result: Result<(), std::io::Error> = fs::create_dir(dirs::data_dir().unwrap().join("rtodo").as_path());

            println!("No todos found.");
        
        }

    }
}
