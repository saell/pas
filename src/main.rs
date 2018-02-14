extern crate personal_activity_sampler;

use personal_activity_sampler::configuration;
// struct Activity {
//     starttime :time::Instant,
//     duration: time::Duration,
//     endtime :time::Instant,
//     description :String,
//     categories :Box<String>
// }

// pub fn anagrams_for<'a>( inp: &[&'a str]) -> Vec<&'a str> {
pub fn list_categories<'a>( inp: &[&'a str]){
    for categorie in inp {
        println!( "{}", categorie );
    }
}

pub fn ask_for_activity <'a>(elapsed_time_s: u32, categories: &[&'a str]){
    let elapsed_time_minutes = elapsed_time_s/60;
    println!( "What have you done the in the last {} minutes? Choose!:", elapsed_time_minutes );

    list_categories( &categories );
}

pub fn run(config: configuration::Config){

    //todo  load already known categories from file
    let mut categories: Vec<&str> = vec!["Bugfixing", "organisatorisch", "newFeatures"];

    //Choose categories from list or enter new one
    ask_for_activity(config.interval_s, &categories);

    println!( "x - enter new Activity" );


    // let mut enterNewCategorie = false;

    //check intervall is over
    //ask for activity

}

use std::{thread, time};
use std::env;
use std::process;



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
    run( config );
}
