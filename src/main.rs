use mailparse;
use whatlang::{self, Lang};
use xdg;

use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};


const PROGRAM_NAME: &'static str = "ottolangy";
const MUTTRC_FILENAME: &'static str = "attribution.muttrc";

const ATTRIBUTION_FR: &'static str =
    r#"set attribution = "Le %{%e %b. %Y à %H:%M %Z}, %f a écrit:"
"#;
const ATTRIBUTION_EN: &'static str =
    r#"set attribution = "On %{%b %e, %Y, at %I:%M %p %Z}, %f wrote:"
"#;


fn main() {
    let mut email_input: Vec<u8> = Vec::new();

    let mut stdin = io::stdin();
    stdin.read_to_end(&mut email_input).unwrap();

    let body = get_email_body(&email_input).unwrap();
    print!("{}", body);

    let lang_info = whatlang::detect(&body).unwrap();
    println!("{:?}", lang_info);

    let attribution_config = if lang_info.lang() == Lang::Fra {
        ATTRIBUTION_FR
    } else {
        ATTRIBUTION_EN
    };

    write_attribution(&attribution_config).unwrap();
}

fn get_email_body(email: &[u8]) -> Result<String, Box<dyn Error>> {
    let email = mailparse::parse_mail(&email).unwrap();

    if email.subparts.is_empty() {
        let body = email.get_body().unwrap();

        return Ok(body);
    } else {
        for part in email.subparts {
            for header in part.get_headers() {
                println!("{}: {}", header.get_key(), header.get_value());

                if header.get_key() == "Content-Type"
                    && header.get_value().starts_with("text/plain")
                {
                    return Ok(part.get_body().unwrap());
                }
            }
        }
    }

    Err("parse".into())
}

fn write_attribution(config: &str) -> Result<(), Box<dyn Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(PROGRAM_NAME).unwrap();

    let muttrc_path = xdg_dirs.place_data_file(MUTTRC_FILENAME).unwrap();

    let mut file = File::create(muttrc_path).unwrap();
    file.write_all(config.as_bytes()).unwrap();

    Ok(())
}
