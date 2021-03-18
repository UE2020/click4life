extern crate enigo;
use enigo::*;
use std::thread;
extern crate clap;
use clap::{Arg, App, SubCommand};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::io::{self, Write};

fn main() {
    let mut enigo = Enigo::new();
    let matches = App::new("click4life")
    .version("1.0")
    .author("Aspect#8445")
    .about("A simple cross-platform autoclicker.")
    .arg(Arg::with_name("delay")
         .help("Sets the length of the delay (in milliseconds) before the payload begins.")
         .required(true)
         .index(1))
    .arg(Arg::with_name("duration")
         .help("Sets the duration (in millseconds) of the payload.")
         .required(true)
         .index(2))
    .get_matches();
    let mut temp = String::new();
    print!("Are you sure you want to enable the payload? [Y/n] ");
    std::io::stdout().flush().unwrap();
    let duration = matches.value_of("duration").unwrap().parse().unwrap();
    let delay:usize = matches.value_of("delay").unwrap().parse().unwrap();
    std::io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    let temp = temp.trim();

    if temp == "Y" || temp == "y" || temp == "yes" || temp == "Yes" || temp == "YES" {
        println!("Beginning {} second payload in {} ms.", duration, delay);
    }
    else if temp == "N" || temp == "n" || temp == "no" || temp == "No" || temp == "NO" {
        println!("Canceling...");
        return;
    }
    thread::sleep(Duration::from_millis(delay as u64));
    let start = SystemTime::now();
    loop {
        if start.elapsed().unwrap().as_millis() >= duration {
            break;
        }
        enigo.mouse_click(MouseButton::Left);
        thread::sleep(Duration::from_millis(5));
    }
    println!("Payload ended.");
}
