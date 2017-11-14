extern crate personal_activity_sampler;

use std::{thread, time};
use std::env;
use std::process;

use personal_activity_sampler::Config;

fn main() {
    println!("Plant Activity Sampler!");

    let arguments: Vec<String> = env::args().collect();

    let config = Config::new(&arguments).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    let seconds = time::Duration::from_secs(1);
    let now = time::Instant::now();

    thread::sleep(seconds);

    assert!(now.elapsed() >= seconds);

    println!("{}", now.elapsed().as_secs());

    personal_activity_sampler::run( config );
}
