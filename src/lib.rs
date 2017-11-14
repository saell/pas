use std::time;

use std::io;
use std::io::Stdin;

pub struct Config{
    interval_s: u32,
}

impl Config{
    pub fn new( arguments: &[String] )-> Result<Config, &'static str>{
        if arguments.len() == 1 {
            let interval_s = 10;
            return Ok(Config{interval_s});
        }

        if arguments.len() < 2 {
          return Err("not enough arguments");
        }

        //let interval_s = arguments[1].clone();
        let interval_s = 10;//for now as log as no changeable config is needed
        Ok(Config{interval_s})
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_name() {
        unimplemented!()
    }
}

struct Activity {
    starttime :time::Instant,
    duration: time::Duration,
    endtime :time::Instant,
    description :String,
    categories :Box<String>
}

pub fn run(config :Config){
    println!("Interval {} s", config.interval_s);

    let elapsed_time_minutes = config.interval_s/60;
    println!( "What have you done the in the last {} minutes? Choose!:", elapsed_time_minutes );

    // let mut categories :Box<String>;
    //todo  fill categories from already known ones
    // for categorie in categories {
    //     println!( "{}", categorie );
    // }
    println!( "x - enter new Activity" );

    let mut enterNewCategorie = false;

    //check intervall is over
    //ask for activity

}
