extern crate personal_activity_sampler;

// // struct Activity {
// //     starttime :time::Instant,
// //     duration: time::Duration,
// //     endtime :time::Instant,
// //     description :String,
// //     categories :Box<String>
// // }
//
// pub fn run(config: config::Config){
//     println!("Interval {} s", config.interval_s);
//
//     let elapsed_time_minutes = config.interval_s/60;
//     println!( "What have you done the in the last {} minutes? Choose!:", elapsed_time_minutes );
//
//     // let mut categories :Box<String>;
//     //todo  fill categories from already known ones
//     // for categorie in categories {
//     //     println!( "{}", categorie );
//     // }
//     println!( "x - enter new Activity" );
//
//     // let mut enterNewCategorie = false;
//
//     //check intervall is over
//     //ask for activity
//
// }
//
use std::{thread, time};
use std::env;
use std::process;

use personal_activity_sampler::configuration;

fn main() {
    println!("Plant Activity Sampler!");

    let arguments: Vec<String> = env::args().collect();

    let config = personal_activity_sampler::configuration::new(&arguments).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let seconds = time::Duration::from_secs(1);
    let now = time::Instant::now();

    thread::sleep(seconds);

    assert!(now.elapsed() >= seconds);

    println!("{}", now.elapsed().as_secs());

    // personal_activity_sampler::run( config );
}
