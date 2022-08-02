use std::fmt;

use clap::{Parser, ValueEnum};
use nextsv::{Error, VersionCalculator};

#[derive(ValueEnum, Debug, Clone)]
enum ForceOptions {
    Major,
    Minor,
    Patch,
    First,
}

impl fmt::Display for ForceOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ForceOptions::Major => write!(f, "major"),
            ForceOptions::Minor => write!(f, "minor"),
            ForceOptions::Patch => write!(f, "patch"),
            ForceOptions::First => write!(f, "first"),
        }
    }
}
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    logging: Verbosity,
    /// Force the calculation of the version number
    #[clap(short, long, value_enum)]
    force: Option<ForceOptions>,
    /// Prefix string to identify version number tags
    #[clap(short, long, value_parser, default_value = "v")]
    prefix: String,
    /// Report the level of the version number change
    #[clap(long)]
    level: bool,
    /// Report the version number
    #[clap(long)]
    number: bool,
}

fn main() {
    let args = Cli::parse();

    let mut builder = get_logging(args.logging.log_level());
    builder.init();
    log::info!("Calculating the next version number.");
    let latest_version = match VersionCalculator::new(&args.prefix) {
        Ok(v) => v,
        Err(e) => {
            log::error!("{}", e.to_string());
            std::process::exit(1)
        }
    };

    match calculate(latest_version, args.force, args.level, args.number) {
        Ok(_) => {}
        Err(e) => {
            log::error!("{}", e.to_string());
            std::process::exit(2)
        }
    }
}

fn calculate(
    mut latest_version: VersionCalculator,
    force: Option<ForceOptions>,
    level: bool,
    number: bool,
) -> Result<(), Error> {
    if let Some(f) = &force {
        log::debug!("Force option set to {}", f);
    };
    let (next_version, bump) = if let Some(svc) = force {
        match svc {
            ForceOptions::Major => latest_version.force_major().next_version(),
            ForceOptions::Minor => latest_version.force_minor().next_version(),
            ForceOptions::Patch => latest_version.force_patch().next_version(),
            ForceOptions::First => latest_version.promote_first()?,
        }
    } else {
        latest_version.commits()?.next_version()
    };

    match (number, level) {
        (false, false) => println!("{}", bump),
        (false, true) => println!("{}", bump),
        (true, false) => println!("{}", next_version),
        (true, true) => {
            println!("{}", next_version);
            println!("{}", bump);
        }
    }
    Ok(())
}

pub fn get_logging(level: log::Level) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level.to_level_filter());

    builder.format_timestamp_secs().format_module_path(false);

    builder
}

#[derive(clap::Args, Debug, Clone)]
pub struct Verbosity {
    /// Pass many times for less log output
    #[clap(long, short, parse(from_occurrences))]
    quiet: i8,

    /// Pass many times for more log output
    ///
    /// By default, it'll report info. Passing `-v` one time adds debug
    /// logs, `-vv` adds trace logs.
    #[clap(long, short, parse(from_occurrences))]
    verbose: i8,
}

impl Verbosity {
    /// Get the log level.
    pub fn log_level(&self) -> log::Level {
        let verbosity = 2 - self.quiet + self.verbose;

        match verbosity {
            i8::MIN..=0 => log::Level::Error,
            1 => log::Level::Warn,
            2 => log::Level::Info,
            3 => log::Level::Debug,
            4..=i8::MAX => log::Level::Trace,
        }
    }
}
