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
    match run() {
        Ok(_) => (),
        Err(e) => eprintln!("{}: error: {}", PROGRAM_NAME, e),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut email_input: Vec<u8> = Vec::new();

    let mut stdin = io::stdin();
    stdin.read_to_end(&mut email_input)?;

    let body = get_email_body(&email_input)?;
    print!("{}", body);

    let lang_info = whatlang::detect(&body)
        .ok_or("unable to detect language")?;
    println!("{:?}", lang_info);

    let attribution_config = if lang_info.lang() == Lang::Fra {
        ATTRIBUTION_FR
    } else {
        ATTRIBUTION_EN
    };

    write_attribution(&attribution_config)?;

    Ok(())
}

fn get_email_body(email: &[u8]) -> Result<String, Box<dyn Error>> {
    let email = mailparse::parse_mail(&email)?;

    if email.subparts.is_empty() {
        let body = email.get_body()?;

        return Ok(body);
    } else {
        for part in email.subparts {
            for header in part.get_headers() {
                println!("{}: {}", header.get_key(), header.get_value());

                if header.get_key() == "Content-Type"
                    && header.get_value().starts_with("text/plain")
                {
                    return Ok(part.get_body()?);
                }
            }
        }
    }

    Err("parse".into())
}

fn write_attribution(config: &str) -> Result<(), Box<dyn Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(PROGRAM_NAME)?;

    let muttrc_path = xdg_dirs.place_data_file(MUTTRC_FILENAME)?;

    let mut file = File::create(muttrc_path)?;
    file.write_all(config.as_bytes())?;

    Ok(())
}
