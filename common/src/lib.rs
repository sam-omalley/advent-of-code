#[derive(Debug, Clone)]
pub struct Config {
    pub file_path: String,
    pub max: usize,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let usage = "usage: <program> <filepath>";

        // Consume argv[0]
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err(usage),
        };

        let max: usize = if let Some(max) = args.next() {
            let val = max.parse();
            if val.is_err() {
                return Err("Invalid number: {max}");
            }
            val.unwrap()
        } else {
            0
        };

        if args.next().is_some() {
            return Err(usage);
        }

        Ok(Self { file_path, max })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_path() {
        let result = Config::build(vec!["app".to_string(), "input.txt".to_string()].into_iter());
        assert!(matches!(result, Ok(_)));

        assert_eq!(result.unwrap().file_path, "input.txt");

        let result = Config::build(
            vec!["app".to_string(), "input.txt".to_string(), "5".to_string()].into_iter(),
        );
        assert!(matches!(result, Ok(_)));

        assert_eq!(result.clone().expect("Config good").file_path, "input.txt");
        assert_eq!(result.clone().expect("Config good").max, 5);
    }

    #[test]
    fn bad_path() {
        let result = Config::build(vec!["app".to_string()].into_iter());
        assert!(matches!(result, Err(_)));

        let result = Config::build(
            vec![
                "app".to_string(),
                "input.txt".to_string(),
                "other".to_string(),
            ]
            .into_iter(),
        );
        assert!(matches!(result, Err(_)));
    }
}
