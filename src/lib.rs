pub mod Config;

// use Config::Config;
use std::time;
// use std::io;
// use std::io::Stdin;

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

pub fn run(config: Config::Config){
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
