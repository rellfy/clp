pub struct CommandLineParser {
    args: Vec<String>,
}

impl CommandLineParser {
    pub fn new(args: Vec<String>) -> CommandLineParser {
        let mut clp: CommandLineParser = CommandLineParser {
            args,
        };
        clp
    }

    pub fn key_index(&self, key: &str) -> Option<usize> {
        self.args.iter().position(|x| x == key)
    }

    pub fn has_key(&self, key: &str) -> bool {
        self.key_index(key).is_some()
    }

    pub fn key_value(&self, key: &str) -> Option<String> {
        let index = self.key_index(key);
        if let Some(index) = index {
            if self.args.len() >= index + 2 {
                let value = String::from(&self.args[index+1]);
                return Some(value);
            }

            None
        } else {
            None
        }
    }
}