extern crate personal_activity_sampler;

use personal_activity_sampler::configuration;
// struct Activity {
//     starttime :time::Instant,
//     duration: time::Duration,
//     endtime :time::Instant,
//     description :String,
//     categories :Box<String>
// }

pub fn list_categories<'a>(categories: &[&'a str]) {
    for (index, category) in categories.iter().enumerate() {
        println!("{}\t{}", index, category);
    }
}

#[macro_use]
extern crate text_io;
pub fn ask_for_activity<'a>(elapsed_time_s: u32, categories: &[&'a str]) {
    let elapsed_time_minutes = elapsed_time_s / 60;
    println!(
        "What have you done the in the last {} minutes?",
        elapsed_time_minutes
    );

    list_categories(&categories);

    println!("Choose Activity or write new name to add to list\n>");

    let choice: String = read!();
    println!("{}", choice );
    //todo check choice
    //if choice is text
    //      if not known activity(already in list)
    //          add activity to list
    //else if number
    //      if not in list
    //          error
    //update activity
}

pub fn run(config: configuration::Config) {

    //todo load already known categories from file
    let categories: Vec<&str> = vec!["bug-fixing", "organize", "new feature Box"];

    //Choose categories from list or enter new one
    ask_for_activity(config.interval_s, &categories);

    //check interval is over

    //todo store already known categories to file
}

use std::{thread, time};
use std::env;
use std::process;

fn main() {
    println!("Plant Activity Sampler!");
    println!("=======================\n");
    let arguments: Vec<String> = env::args().collect();

    let config = personal_activity_sampler::configuration::new(&arguments).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let seconds = time::Duration::from_secs(1);
    let now = time::Instant::now();

    thread::sleep(seconds);

    assert!(now.elapsed() >= seconds);

    // println!("{}", now.elapsed().as_secs());

    // personal_activity_sampler::run( config );
    run(config);
}
