use std::ffi::OsStr;
use std::fmt;
use std::path::PathBuf;

use clap::{ArgEnum, ArgGroup, Args, Parser};
use serde::Serialize;
use url::Url;

const ABOUT: &str = r#"🔍🀄️ Review your Tenhou or Mahjong Soul (Jantama) log with mjai-compatible mahjong AIs.

Basic usage:
  $ mjai-reviewer -b mortal -u "https://tenhou.net/0/?log=2019050417gm-0029-0000-4f2a8622&tw=2"

For more details, please visit the repo at <https://github.com/Equim-chan/mjai-reviewer>."#;

#[derive(Debug, Parser)]
#[clap(version, about = ABOUT)]
#[clap(group(
    ArgGroup::new("input-methods")
        .args(&["in-file", "tenhou-id", "url"]),
))]
pub struct Options {
    /// The ID of the player to review, which is a number within 0-3. It is the
    /// number after "&tw=" in Tenhou's log URL, namely, the player sitting at
    /// the East at E1 is 0, and his shimocha (right) will be 1, toimen (across)
    /// will be 2, kamicha (left) will be 3. This option has higher priority
    /// over the "&tw=" in --url if specified.
    #[clap(short = 'a', long, value_name = "ID", parse(try_from_str = parse_player_id))]
    pub player_id: Option<u8>,

    /// The display name of the player to review. This option has higher
    /// priority over the "&tw=" in --url if specified.
    #[clap(short = 'n', long, value_name = "NAME", conflicts_with = "player-id")]
    pub player_name: Option<String>,

    #[clap(flatten, next_help_heading = "INPUT OPTIONS")]
    pub input_opts: InputOptions,

    #[clap(flatten, next_help_heading = "OUTPUT OPTIONS")]
    pub output_opts: OutputOptions,

    /// Kyokus to review. If LIST is empty, review all kyokus. Example:
    /// "E1,E4,S3.1", which means to review East-1, East-4, and South3-1.
    #[clap(short, long, value_name = "LIST")]
    pub kyokus: Option<String>,

    /// Do not review at all, but only download and save files.
    #[clap(long)]
    pub no_review: bool,

    /// Print verbose logs.
    #[clap(short, long)]
    pub verbose: bool,

    /// The backend to use for review.
    #[clap(
        short,
        long,
        arg_enum,
        required_unless_present = "no-review",
        requires = "input-methods"
    )]
    pub backend: Option<Backend>,

    #[clap(flatten, next_help_heading = "MORTAL OPTIONS")]
    pub mortal_opts: MortalOptions,

    #[clap(flatten, next_help_heading = "AKOCHAN OPTIONS")]
    pub akochan_opts: AkochanOptions,
}

#[derive(Debug, Args)]
pub struct InputOptions {
    /// The name of a tenhou.net/6 format log file to input. If FILE is "-" or
    /// empty, read from stdin.
    #[clap(short, long, value_name = "FILE")]
    pub in_file: Option<PathBuf>,

    /// The ID of a Tenhou log to review. Example:
    /// "2019050417gm-0029-0000-4f2a8622".
    #[clap(short, long, value_name = "ID")]
    pub tenhou_id: Option<String>,

    /// Tenhou log URL, as an alternative to --tenhou-id.
    #[clap(short, long, value_name = "URL", parse(try_from_str))]
    pub url: Option<Url>,
}

#[derive(Debug, Args)]
pub struct OutputOptions {
    /// The name of the generated HTML/JSON report file to output. If FILE is
    /// "-", write to stdout; if FILE is empty, write to
    /// "{backend}-{tenhou_id}&tw={actor}.{format}" if a Tenhou log ID is known,
    /// "{backend}-report.{format}" otherwise.
    #[clap(short, long, value_name = "FILE")]
    pub out_file: Option<PathBuf>,

    /// Save the downloaded tenhou.net/6 format log to FILE, which requires
    /// --tenhou-id to be specified. If FILE is "-", write to stdout.
    #[clap(long, value_name = "FILE")]
    pub tenhou_out: Option<PathBuf>,

    /// Save the converted mjai format log to FILE. If FILE is "-", write to
    /// stdout.
    #[clap(long, value_name = "FILE")]
    pub mjai_out: Option<PathBuf>,

    /// Output review result in JSON instead of HTML.
    #[clap(long)]
    pub json: bool,

    /// Do not include log viewer in the generated HTML report.
    #[clap(long)]
    pub without_log_viewer: bool,

    /// Do not include player names in the generated HTML report.
    #[clap(long)]
    pub anonymous: bool,

    /// Do not automatically open the output file in browser.
    #[clap(long)]
    pub no_open: bool,
}

#[derive(Debug, Args)]
pub struct MortalOptions {
    #[clap(
        long,
        value_name = "FILE",
        default_value_os = OsStr::new("./mortal/mortal")
    )]
    pub mortal_exe: PathBuf,

    #[clap(
        long,
        value_name = "FILE",
        default_value_os = OsStr::new("./mortal/config.toml")
    )]
    pub mortal_cfg: PathBuf,
}

#[derive(Debug, Args)]
pub struct AkochanOptions {
    #[clap(
        long,
        value_name = "DIR",
        default_value_os = OsStr::new("./akochan")
    )]
    pub akochan_dir: PathBuf,

    #[clap(
        long,
        value_name = "FILE",
        default_value_os = OsStr::new("./akochan/tactics.json")
    )]
    pub akochan_tactics: PathBuf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, ArgEnum)]
pub enum Backend {
    Mortal,
    Akochan,
}

// TODO: remove for now
// #[derive(Debug, Clone, Copy, Serialize, ArgEnum)]
// pub enum Language {
//     // The string is used in html lang attribute, as per BCP47.
//     #[serde(rename = "ja")]
//     Japanese,
//     #[serde(rename = "en")]
//     English,
// }

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Mortal => "mortal",
            Self::Akochan => "akochan",
        };
        fmt::Display::fmt(s, f)
    }
}

fn parse_player_id(s: &str) -> Result<u8, String> {
    let id = s.parse().map_err(|_| format!("{s} is not a number"))?;
    if id >= 4 {
        Err(format!("{s} is not within 0-3"))
    } else {
        Ok(id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use clap::CommandFactory;

    #[test]
    fn cli_parse() {
        Options::command().debug_assert();
    }
}
