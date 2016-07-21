use std::fmt;
use std::num::ParseIntError;

use getopts;


/// A **misfire** is a thing that can happen instead of listing files -- a
/// catch-all for anything outside the program’s normal execution.
#[derive(PartialEq, Debug)]
pub enum Misfire {

    /// The getopts crate didn’t like these arguments.
    InvalidOptions(getopts::Fail),

    /// The user asked for help. This isn’t strictly an error, which is why
    /// this enum isn’t named Error!
    Help(String),

    /// The user wanted the version number.
    Version,

    /// Two options were given that conflict with one another.
    Conflict(&'static str, &'static str),

    /// An option was given that does nothing when another one either is or
    /// isn't present.
    Useless(&'static str, bool, &'static str),

    /// An option was given that does nothing when either of two other options
    /// are not present.
    Useless2(&'static str, &'static str, &'static str),

    /// A numeric option was given that failed to be parsed as a number.
    FailedParse(ParseIntError),
}

impl Misfire {

    /// The OS return code this misfire should signify.
    pub fn error_code(&self) -> i32 {
        if let Misfire::Help(_) = *self { 2 }
                                   else { 3 }
    }

    /// The Misfire that happens when an option gets given the wrong
    /// argument. This has to use one of the `getopts` failure
    /// variants--it’s meant to take just an option name, rather than an
    /// option *and* an argument, but it works just as well.
    pub fn bad_argument(option: &str, otherwise: &str) -> Misfire {
        Misfire::InvalidOptions(getopts::Fail::UnrecognizedOption(format!("--{} {}", option, otherwise)))
    }
}

impl fmt::Display for Misfire {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Misfire::*;

        match *self {
            InvalidOptions(ref e)  => write!(f, "{}", e),
            Help(ref text)         => write!(f, "{}", text),
            Version                => write!(f, "exa {} {}", env!("CARGO_PKG_VERSION"),include_str!("../../.most_recent_commit")),
            Conflict(a, b)         => write!(f, "Option --{} conflicts with option {}.", a, b),
            Useless(a, false, b)   => write!(f, "Option --{} is useless without option --{}.", a, b),
            Useless(a, true, b)    => write!(f, "Option --{} is useless given option --{}.", a, b),
            Useless2(a, b1, b2)    => write!(f, "Option --{} is useless without options --{} or --{}.", a, b1, b2),
            FailedParse(ref e)     => write!(f, "Failed to parse number: {}", e),
        }
    }
}
