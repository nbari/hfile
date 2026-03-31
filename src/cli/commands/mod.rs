use clap::{
    Arg, ArgAction, Command, ValueEnum,
    builder::{
        EnumValueParser,
        styling::{AnsiColor, Effects, Styles},
    },
};

const ABOUT: &str = "hfile generates cryptographic hashes from files.";

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Algorithm {
    Md5,
    Sha1,
    Sha256,
    Sha384,
    Sha512,
    Blake,
}

impl Algorithm {
    #[must_use]
    pub const fn digest_len(self) -> usize {
        match self {
            Self::Md5 => 32,
            Self::Sha1 => 40,
            Self::Sha256 | Self::Blake => 64,
            Self::Sha384 => 96,
            Self::Sha512 => 128,
        }
    }
}

#[must_use]
pub fn new() -> Command {
    let styles = Styles::styled()
        .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
        .error(AnsiColor::Red.on_default() | Effects::BOLD)
        .invalid(AnsiColor::Red.on_default() | Effects::BOLD)
        .valid(AnsiColor::Green.on_default());

    Command::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(ABOUT)
        .styles(styles)
        .arg(
            Arg::new("algorithm")
                .short('a')
                .long("algorithm")
                .value_parser(EnumValueParser::<Algorithm>::new())
                .value_name("ALGORITHM")
                .default_value("blake"),
        )
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .help("Show size of file")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .required_unless_present_any(["path", "duplicates", "check"]),
        )
        .arg(
            Arg::new("duplicates")
                .short('d')
                .long("duplicates")
                .help("Find duplicates")
                .action(ArgAction::SetTrue)
                .requires("path"),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("Create hash for all files under path")
                .value_name("PATH"),
        )
        .arg(
            Arg::new("check")
                .short('c')
                .long("check")
                .help("Read checksums from file and verify them")
                .value_name("CHECK")
                .conflicts_with_all(["path", "duplicates", "size", "file"]),
        )
}

#[cfg(test)]
mod tests {
    use super::new;
    use clap::ColorChoice;

    #[test]
    fn command_debug_assert() {
        new().debug_assert();
    }

    #[test]
    fn help_uses_ansi_styles() {
        let mut command = new().color(ColorChoice::Always);
        let help = command.render_help().ansi().to_string();

        assert!(help.contains("\u{1b}["));
        assert!(help.contains("Usage:"));
        assert!(help.contains("--algorithm"));
    }
}
