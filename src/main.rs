use std::fmt;

use clap::{Parser, Subcommand, ValueEnum};
use nextsv::{Error, Level, VersionCalculator};

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

#[derive(Debug, Subcommand)]
enum Commands {
    /// Calculate the version for the next semantic version increase
    #[clap()]
    Version {
        #[clap(flatten)]
        logging: Verbosity,
        #[clap(short, long, value_enum)]
        force: Option<ForceOptions>,
        #[clap(short, long, value_parser, default_value = "v")]
        prefix: String,
        #[clap(short, long)]
        level: bool,
    },
    /// Calculate the level for the next semantic version increase
    #[clap()]
    Level {
        #[clap(flatten)]
        logging: Verbosity,
        #[clap(short, long, value_enum)]
        force: Option<ForceOptions>,
        #[clap(short, long, value_parser, default_value = "v")]
        prefix: String,
    },
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

fn main() {
    let args = Cli::parse();

    match args.commands {
        Commands::Version {
            logging,
            force,
            prefix,
            level,
        } => {
            let mut builder = get_logging(logging.log_level());
            builder.init();
            log::info!("Calculating the next version number.");
            let latest_version = match VersionCalculator::new(&prefix) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("{}", e.to_string());
                    std::process::exit(1)
                }
            };

            match version(latest_version, force, level) {
                Ok(_) => {}
                Err(e) => {
                    log::error!("{}", e.to_string());
                    std::process::exit(2)
                }
            }
        }
        Commands::Level {
            logging,
            force,
            prefix,
        } => {
            let mut builder = get_logging(logging.log_level());
            builder.init();
            log::info!("Calculating the level change for the next version number.");
            let latest_version = match VersionCalculator::new(&prefix) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("{}", e.to_string());
                    std::process::exit(1)
                }
            };

            match level(latest_version, force) {
                Ok(_) => {}
                Err(e) => {
                    log::error!("{}", e.to_string());
                    std::process::exit(3)
                }
            };
        }
    }
}

fn version(
    mut latest_version: VersionCalculator,
    force: Option<ForceOptions>,
    level: bool,
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

    println!("{}", next_version);
    if level {
        println!("{}", bump);
    }
    Ok(())
}

fn level(latest_version: VersionCalculator, force: Option<ForceOptions>) -> Result<(), Error> {
    if let Some(f) = &force {
        log::debug!("Force option set to {}", f);
    };

    let next_level = if let Some(svc) = force {
        match svc {
            ForceOptions::Major => Level::Major,
            ForceOptions::Minor => Level::Minor,
            ForceOptions::Patch => Level::Patch,
            ForceOptions::First => Level::Release,
        }
    } else {
        let mut latest_version = latest_version.commits()?;
        latest_version.next_level()?
    };

    println!("{}", next_level);
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
