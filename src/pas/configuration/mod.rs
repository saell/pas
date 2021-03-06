#[derive(Debug, PartialEq)]
pub struct Config {
    pub interval_s: u32,
}

const DEFAULT_INTERVAL_S: u32 = 600;

pub fn new(arguments: &[String]) -> Result<Config, &'static str> {
    if arguments.len() == 1 {
        let interval_s = DEFAULT_INTERVAL_S;
        return Ok(Config { interval_s });
    }

    if arguments.len() < 2 {
        return Err("not enough arguments");
    }

    //let interval_s = arguments[1].clone();
    let interval_s = DEFAULT_INTERVAL_S; //for now as no changeable config is needed
    Ok(Config { interval_s })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        // let arguments= String::from("cmd").collect();
        let mut arguments = Vec::new();
        arguments.push(String::from("cmd"));
        assert_eq!(
            Ok(Config { interval_s: DEFAULT_INTERVAL_S }),
            new(&arguments)
        )
    }
}
