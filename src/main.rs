// use email_parser::email::Email;
// use mailparse;
// use email_format::Email;
// use email_format::rfc5322::Parsable;
use email::rfc5322::Rfc5322Parser;

use std::io::{self, Read};


fn main() {
    // let mut email_input: Vec<u8> = Vec::with_capacity(2048);
    let mut email_input: Vec<u8> = Vec::new();

    let mut stdin = io::stdin();
    stdin.read_to_end(&mut email_input).unwrap();

    // println!("{}", String::from_utf8(email_input).unwrap());

    // email-parser
    // let email = Email::parse(&email_input).unwrap();
    // println!("{:?}", email.body);

    // mailparse
    // let email = mailparse::parse_mail(&email_input).unwrap();
    // let body = email.get_body().unwrap();
    // println!("{:?}", body);

    // email-format
    // let email = Email::parse(&email_input).unwrap().0;
    // print!("{:?}", email.get_body().unwrap());

    // email
    let email = Rfc5322Parser::
}
