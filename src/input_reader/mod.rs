use std::io;

pub struct Input {
    pub first_done: bool,
    pub widget_data: String,
}

impl Input {
    pub fn get(&self, input: &str) -> String {
        println!("{}", input);
        let mut return_data = String::new();
        io::stdin()
            .read_line(&mut return_data)
            .expect("Failed to read input");
        return_data
    }
    pub fn n_or_val(&self, input: &str) -> Option<String> {
        let data = self.get(input);
        return match data.trim() {
            "n" => {
                None
            }
            _ => Some(data),
        };
    }
    pub fn y_or_n(&self, input: &str) -> bool {
        loop {
            let what = self.n_or_val(input);
            match what {
                None => {
                    return false;
                }
                Some(data) => {
                    if data == *"y" {
                        return true;
                    }
                }
            }
        }
    }
}