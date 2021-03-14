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

use exitcode;
use mailparse;
use thiserror::Error;
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


#[derive(Error, Debug)]
enum WrapError {
    #[error("unable to parse email body: {0}")]
    ParseMail(#[from] mailparse::MailParseError),

    #[error("unable to parse email body")]
    ParseMailUnknown,

    #[error(transparent)]
    Xdg(#[from] xdg::BaseDirectoriesError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug)]
enum OttolangyError {
    #[error("failed to read from stdin: {0}")]
    ReadStdin(#[from] std::io::Error),

    #[error("unable to detect language")]
    DetectLanguage,

    #[error("failed to write attribution config file")]
    WriteConfig(WrapError),

    #[error(transparent)]
    Wrapped(WrapError),
}


fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}: error: {}", PROGRAM_NAME, e);

            match e {
                OttolangyError::Wrapped(WrapError::ParseMail(_))
                | OttolangyError::Wrapped(WrapError::ParseMailUnknown) =>
                    process::exit(exitcode::DATAERR),
                OttolangyError::Wrapped(WrapError::Xdg(_))
                | OttolangyError::Wrapped(WrapError::Io(_))
                | OttolangyError::WriteConfig(_) =>
                    process::exit(exitcode::IOERR),
                OttolangyError::ReadStdin(_) =>
                    process::exit(exitcode::NOINPUT),
                OttolangyError::DetectLanguage =>
                    process::exit(exitcode::SOFTWARE),
            }
        },
    }
}

/// Get an email from standard input and write a Mutt attribution config based
/// on the language.
fn run() -> Result<(), OttolangyError> {
    let mut email_input: Vec<u8> = Vec::new();

    let mut stdin = io::stdin();
    stdin.read_to_end(&mut email_input)
        .map_err(|e| OttolangyError::ReadStdin(e))?;

    let body = get_email_body(&email_input)
        .map_err(|e| OttolangyError::Wrapped(e))?;

    let lang_info = whatlang::detect(&body)
        .ok_or(OttolangyError::DetectLanguage)?;

    let attribution_config = if lang_info.lang() == Lang::Fra {
        ATTRIBUTION_FR
    } else {
        ATTRIBUTION_EN
    };

    write_attribution(&attribution_config)
        .map_err(|e| OttolangyError::WriteConfig(e))?;

    Ok(())
}

/// Extract the body from an email.
///
/// Given an email as input, parses it and extracts the body. For multipart
/// emails, the body is extracted from the text part.
fn get_email_body(email: &[u8]) -> Result<String, WrapError> {
    let email = mailparse::parse_mail(&email)?;

    println!("ctype: {:?}", email.ctype);

    if email.subparts.is_empty() {
        let body = email.get_body()?;

        return Ok(body);
    }

    // TODO: If no plain text part, use html one
    // TODO: New predicate function for text/plain
    // TODO: Maybe split into functions
    for part in email.subparts {
        // if part.headers.get_app_values() is one of:
        // "multipart/alternative"
        // "text/plain"

        println!("part ctype: {:?}", part.ctype);

        for header in part.get_headers() {
            if header.get_key() == "Content-Type"
                && header.get_value().starts_with("multipart/alternative")
            {
                for alternative_part in &part.subparts {
                    println!("apart ctype: {:?}", alternative_part.ctype);
                    for alternative_header in alternative_part.get_headers() {
                        if alternative_header.get_key() == "Content-Type"
                            && alternative_header.get_value().starts_with("text/plain")
                        {
                            return Ok(alternative_part.get_body()?);
                        }
                    }
                }
            }

            if header.get_key() == "Content-Type"
                && header.get_value().starts_with("text/plain")
            {
                return Ok(part.get_body()?);
            }
        }
    }

    Err(WrapError::ParseMailUnknown)
}

/// Write the attribution config to a file.
///
/// Store the file in the XDG data directory.
fn write_attribution(config: &str) -> Result<(), WrapError> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(PROGRAM_NAME)?;

    let muttrc_path = xdg_dirs.place_data_file(MUTTRC_FILENAME)?;

    let mut file = File::create(muttrc_path)?;
    file.write_all(config.as_bytes())?;

    Ok(())
}
