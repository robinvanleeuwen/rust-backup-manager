extern crate ini;

use std::fs::File;
use std::fs::remove_file;
use std::net::TcpListener;
use std::thread;
use std::io::BufReader;
use std::io::BufRead;
use ini::Ini;
use std::time::Duration;
use std::thread::sleep;



fn main() {

    let listener = TcpListener::bind("0.0.0.0:9123")

        .unwrap_or_else(|e|{
            println!("Could not start server: {}",e);
            std::process::exit(2);

        });

    for stream in listener.incoming() {

        thread::spawn( ||{

            let stream = stream.unwrap();
            handle_client(stream);

        });
    }

}

fn handle_client(stream: std::net::TcpStream){

    match stream.peer_addr() {
        Ok(v) => println!("Got connection from client: {}", v),
        Err(e) => println!("Could not establish a connection: {}", e),
    }

    let entered_line = get_command(stream);
    let command: Vec<&str> = entered_line.split(" ").collect();

    match command[0].as_ref() {
        "start" => handle_start(command[1].trim()),
        _ => println!("Unknown command."),
    }
}

fn handle_start(param: &str) {
    // A start command is given.

    // Create a lock file so that commands are not executed
    // again while still running. If lockfile is created then
    // get the command from the configuration. If lockfile
    // could not be created then our job is done.


    let lockfile_created = create_lock(param);

    if lockfile_created {

        let config = read_config("/etc/backup-manager.conf");

        for (sec, prop) in config.iter() {
            match sec {
                Some(v) => if *v == param.to_string() {
                    match  prop.get("command") {

                        Some(v) => {
                            println!("{:?}", v);
                        },
                        None => println!("No command found for {}", param)
                    }
                },
                None => println!("No section found for {}", param)
            }
        }
        remove_lock(param);
    }
}

fn start_os_process(command: &'static str){

    println!("Starting wait");
    sleep(Duration::from_millis(10000));
    println!("Finished wait");

}

fn remove_lock(filename: &str) {
    let lockfile = format!("/var/run/backup-manager/{}.lock", filename);

    match remove_file(&lockfile) {
        Ok(_t) => println!("Removed lockfile {}", lockfile),
        Err(_e) => println!("Could not remove lockfile {}", lockfile),
    }
}

fn create_lock(filename: &str) -> bool {

    // Create a lockfile. return Ok if lockfile is
    // created or return Err if lockfile already
    // exists.
    let lockfile = format!("/var/run/backup-manager/{}.lock", filename);
    match File::create(&lockfile) {

        Ok(_t) => {
            println!("Created Lockfile {}", lockfile);
            true
        },
        Err(e) => {
            println!("Could not create lockfile {}", lockfile);
            println!("Error: {}",e );
            false
        }

    }

}


fn get_command(stream: std::net::TcpStream) -> String{

    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line);
    line.trim();

    let command: Vec<String> = line.split("").map(|s| s.to_string()).collect();
    let s: String = command.into_iter().collect();

    s.trim_right().to_string()

}

fn read_config(filename: &str) -> Ini {
    println!("Reading configuration file");

    let conf = Ini::load_from_file(filename);
    let conf = match conf {

        Ok(ini) => ini,
        Err(_e) => {
            println!("Could not read config file /etc/backup-manager.conf");
            std::process::exit(1);
        }
    };
    conf
}