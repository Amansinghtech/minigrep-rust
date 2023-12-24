pub struct Config {
    pub(crate) query: String,
    pub(crate) filepath: String
}

impl Config {
    pub(crate) fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments")
        }

        let query = args[1].clone();
        let filepath = args[2].clone();

        Ok(Config { query, filepath })
    }
}
