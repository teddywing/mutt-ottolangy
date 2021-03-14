// Copyright (c) 2021  Teddy Wing
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use anyhow::{anyhow, Context, Error};
use exitcode;
use mailparse;
use thiserror;
use whatlang::{self, Lang};
use xdg;

use std::fs::File;
use std::io::{self, Read, Write};
use std::process;


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


#[derive(thiserror::Error, Debug)]
enum OttolangyError {
    #[error("unable to parse email body: {0}")]
    ParseMail(#[from] mailparse::MailParseError),

    #[error("unable to parse email body")]
    ParseMailUnknown,

    #[error(transparent)]
    Xdg(#[from] xdg::BaseDirectoriesError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}


// TODO: exit codes
fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}: error: {}", PROGRAM_NAME, e);

            match e.downcast_ref::<OttolangyError>() {
                Some(OttolangyError::ParseMail(_)) =>
                    process::exit(exitcode::DATAERR),
                Some(OttolangyError::ParseMailUnknown) =>
                    process::exit(exitcode::DATAERR),
                Some(OttolangyError::Xdg(_)) => process::exit(exitcode::IOERR),
                Some(OttolangyError::Io(_)) => process::exit(exitcode::IOERR),
                None => process::exit(exitcode::UNAVAILABLE),
            }
        },
    }
}

/// Get an email from standard input and write a Mutt attribution config based
/// on the language.
fn run() -> Result<(), Error> {
    let mut email_input: Vec<u8> = Vec::new();

    let mut stdin = io::stdin();
    stdin.read_to_end(&mut email_input)
        .context("failed to read from stdin")?;

    let body = get_email_body(&email_input)
        .context("failed to parse email body")?;

    let lang_info = whatlang::detect(&body)
        .ok_or(anyhow!("unable to detect language"))?;

    let attribution_config = if lang_info.lang() == Lang::Fra {
        ATTRIBUTION_FR
    } else {
        ATTRIBUTION_EN
    };

    write_attribution(&attribution_config)
        .context("failed to write attribution config file")?;

    Ok(())
}

/// Extract the body from an email.
///
/// Given an email as input, parses it and extracts the body. For multipart
/// emails, the body is extracted from the text part.
fn get_email_body(email: &[u8]) -> Result<String, OttolangyError> {
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

    Err(OttolangyError::ParseMailUnknown)
}

/// Write the attribution config to a file.
///
/// Store the file in the XDG data directory.
fn write_attribution(config: &str) -> Result<(), OttolangyError> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(PROGRAM_NAME)?;

    let muttrc_path = xdg_dirs.place_data_file(MUTTRC_FILENAME)?;

    let mut file = File::create(muttrc_path)?;
    file.write_all(config.as_bytes())?;

    Ok(())
}
