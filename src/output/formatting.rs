use crate::{Podcast, Episode, Error};
use std::{
    fmt,
    collections::HashMap,
};
use rt_format::{Format, FormatArgument, ParsedFormat, Specifier};

use super::WriteOptions;

const UNKNOWN: &str = "UNKNOWN";

#[derive(Debug, PartialEq, Clone)]
enum Variant<'a> {
    String(&'a String),
    Int(usize),
    OptionInt(&'a Option<usize>),
    OptionString(&'a Option<String>),
}

fn format_int(
    variant: &Variant,
    f: &mut fmt::Formatter,
    format_function: &dyn Fn(&usize, &mut fmt::Formatter) -> fmt::Result,
) -> fmt::Result {
    match variant {
        Variant::Int(val) => format_function(val, f),
        Variant::OptionInt(x) => match x {
            Some(val) => format_function(&val, f),
            None => Err(fmt::Error),
        },
        _ => Err(fmt::Error)
    }
}

impl<'a> FormatArgument for Variant<'a> {
    fn supports_format(&self, spec: &Specifier) -> bool {
        match self {
            Self::Int(_) => true,
            Self::String(_) | Self::OptionString(_) => match spec.format {
                Format::Display | Format::Debug => true,
                _ => false
            },
            Self::OptionInt(x) => {
                match x {
                    Some(_) => true,
                    None => {
                        match spec.format {
                            Format::Debug => true,
                            _ => false
                        }
                    }
                }
            },
        }
    }

    fn fmt_display(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int(val) => fmt::Display::fmt(&val, f),
            Self::String(val) => fmt::Display::fmt(&val, f),
            Self::OptionString(x) => {
                let value = match x {
                    Some(x) => x,
                    None => UNKNOWN,
                };
                fmt::Display::fmt(&value, f)
            }
            Self::OptionInt(x) => {
                match x {
                    Some(value) => fmt::Display::fmt(&value, f),
                    None => fmt::Display::fmt(UNKNOWN, f),
                }
            }
        }
    }

    fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }

    fn fmt_octal(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_int(self, f, &fmt::Octal::fmt)
    }

    fn fmt_lower_hex(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_int(self, f, &fmt::LowerHex::fmt)
    }

    fn fmt_upper_hex(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_int(self, f, &fmt::UpperHex::fmt)
    }

    fn fmt_binary(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_int(self, f, &fmt::Binary::fmt)
    }

    fn fmt_lower_exp(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_int(self, f, &fmt::LowerExp::fmt)
    }

    fn fmt_upper_exp(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_int(self, f, &fmt::UpperExp::fmt)
    }

    fn to_usize(&self) -> Result<usize, ()> {
        match self {
            Variant::Int(val) => Ok(*val),
            Variant::OptionInt(x) => match x {
                Some(val) => Ok(*val),
                None => Err(()),
            }
            _ => Err(()),
        }
    }
}

type OutputOptions<'a> = HashMap<&'static str, Variant<'a>>;
fn episode_options<'a>(podcast: &'a Podcast, episode: &'a Episode) -> OutputOptions<'a> {
    HashMap::from([
        ("podcast_title", Variant::String(&podcast.title)),
        ("episode_title", Variant::String(&episode.title)),
        ("episode_index", Variant::OptionInt(&episode.index)),
        ("episode_author", Variant::OptionString(&episode.author)),
    ])
}

pub fn format_episode(
    podcast: &Podcast,
    episode: &Episode,
    options: &WriteOptions
) -> Result<String, Error> {
    let named_options = episode_options(podcast, episode);
    let args = ParsedFormat::parse(&options.template, &[], &named_options)
        .map_err(|_e| crate::Error::StringFormat)?;
    let mut formatted = format!("{}", args);
    for remove_string in &options.remove_from_output {
        formatted = formatted.replace(remove_string, "");
    }
    return Ok(formatted);
}
