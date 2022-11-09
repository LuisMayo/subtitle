use clap::ArgMatches;

pub struct Arguments<'a> {
    pub lang: Option<&'a str>,
    pub path: Vec<&'a str>
}

impl <'a> Arguments<'a> {
    pub fn new(matches: &'a ArgMatches) -> Self {
        let path;
        if matches.value_of("file").is_some() {
            path = matches.values_of("file").unwrap().collect();
        } else {
            path = Vec::new();
        }
        return Arguments {
            lang: matches.value_of("lang"),
            path
        }
    }
}
