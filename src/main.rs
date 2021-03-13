use mailparse;
use whatlang::{self, Lang};
use xdg;

use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};


const PROGRAM_NAME: &'static str = "ottolangy";

/// Filename used for the generated attribution config file.
const MUTTRC_FILENAME: &'static str = "attribution.muttrc";

/// French attribution config.
const ATTRIBUTION_FR: &'static str =
    r#"set attribution = "Le %{%e %b. %Y à %H:%M %Z}, %f a écrit:"
"#;

/// English attribution config.
const ATTRIBUTION_EN: &'static str =
    r#"set attribution = "On %{%b %e, %Y, at %I:%M %p %Z}, %f wrote:"
"#;


fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => eprintln!("{}: error: {}", PROGRAM_NAME, e),
    }
}

/// Get an email from standard input and write a Mutt attribution config based
/// on the language.
fn run() -> Result<(), Box<dyn Error>> {
    let mut email_input: Vec<u8> = Vec::new();

    let mut stdin = io::stdin();
    stdin.read_to_end(&mut email_input)?;

    let body = get_email_body(&email_input)?;

    let lang_info = whatlang::detect(&body)
        .ok_or("unable to detect language")?;

    let attribution_config = if lang_info.lang() == Lang::Fra {
        ATTRIBUTION_FR
    } else {
        ATTRIBUTION_EN
    };

    write_attribution(&attribution_config)?;

    Ok(())
}

/// Extract the body from an email.
///
/// Given an email as input, parses it and extracts the body. For multipart
/// emails, the body is extracted from the text part.
fn get_email_body(email: &[u8]) -> Result<String, Box<dyn Error>> {
    let email = mailparse::parse_mail(&email)?;

    if email.subparts.is_empty() {
        let body = email.get_body()?;

        return Ok(body);
    }

    for part in email.subparts {
        for header in part.get_headers() {
            if header.get_key() == "Content-Type"
                && header.get_value().starts_with("text/plain")
            {
                return Ok(part.get_body()?);
            }
        }
    }

    Err("unable to parse email body".into())
}

/// Write the attribution config to a file.
///
/// Store the file in the XDG data directory.
fn write_attribution(config: &str) -> Result<(), Box<dyn Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(PROGRAM_NAME)?;

    let muttrc_path = xdg_dirs.place_data_file(MUTTRC_FILENAME)?;

    let mut file = File::create(muttrc_path)?;
    file.write_all(config.as_bytes())?;

    Ok(())
}
