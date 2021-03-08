use mailparse;

use std::io::{self, Read};


fn main() {
    let mut email_input: Vec<u8> = Vec::new();

    let mut stdin = io::stdin();
    stdin.read_to_end(&mut email_input).unwrap();

    let email = mailparse::parse_mail(&email_input).unwrap();
    if email.subparts.is_empty() {
        let body = email.get_body().unwrap();
        println!("{}", body);
    } else {
        for part in email.subparts {
            for header in part.get_headers() {
                println!("{}: {}", header.get_key(), header.get_value());

                if header.get_key() == "Content-Type"
                    && header.get_value().starts_with("text/plain")
                {
                    print!("{}", part.get_body().unwrap());
                }
            }
        }
    }
}
