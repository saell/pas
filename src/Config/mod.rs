pub struct Config{
    pub interval_s: u32,
}


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
