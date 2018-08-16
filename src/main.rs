extern crate rand;
extern crate regex;
use regex::Regex;
use rand::Rng;
use std::io;

fn get_response() -> String {
    let responses: [&str; 20] = [
      "it is certain",
      "it is decidedly so",
      "without a doubt",
      "yes — definitely",
      "you may rely on it",
      "as I see it, yes",
      "most likely",
      "outlook good",
      "yes",
      "signs point to yes",
      "reply hazy, try again",
      "ask again later",
      "better not tell you now",
      "cannot predict now",
      "concentrate and ask again",
      "don’t count on it",
      "my reply is no",
      "my sources say no",
      "outlook not so good",
      "very doubtful"
    ];
    let num: usize = rand::thread_rng().gen_range(0, responses.len());
    responses[num].to_string()
}

fn check_input(text: &str) -> bool {
    let re = Regex::new(r".\?$").unwrap();
    re.is_match(text)
}

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input_string: String = input.to_string();
            if check_input(&input_string) {
                let response: String = get_response();
                println!("{}", response);
            } else {
                println!("please ask a legitimate question!");
            }
            
        }
        Err(error) => println!("error: {}", error)
    }
}
