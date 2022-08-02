use clap::{Parser, Subcommand, ValueEnum};
use nextsv_lib::Error;
use nextsv_lib::Level;
use nextsv_lib::VersionCalculator;

#[derive(ValueEnum, Debug, Clone)]
enum ForceOptions {
    Major,
    Minor,
    Patch,
    First,
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

fn main() -> Result<(), Error> {
    // What is the latest tag?
    // What are the conventional commits since that tag?
    let args = Cli::parse();

    match args.commands {
        Commands::Version {
            logging,
            force,
            prefix,
        } => {
            let mut builder = get_logging(logging.log_level());
            builder.init();
            let latest_version = VersionCalculator::new(&prefix)?;
            version(latest_version, force)?;
        }
        Commands::Level {
            logging,
            force,
            prefix,
        } => {
            let mut builder = get_logging(logging.log_level());
            builder.init();
            let latest_version = VersionCalculator::new(&prefix)?;
            level(latest_version, force)?;
        }
    }

    Ok(())
}

fn version(
    mut latest_version: VersionCalculator,
    force: Option<ForceOptions>,
) -> Result<(), Error> {
    let next_version = if let Some(svc) = force {
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
    Ok(())
}

fn level(latest_version: VersionCalculator, force: Option<ForceOptions>) -> Result<(), Error> {
    println!("Latest version: {:#?}", &latest_version);
    let next_level = if let Some(svc) = force {
        match svc {
            ForceOptions::Major => Level::Major,
            ForceOptions::Minor => Level::Minor,
            ForceOptions::Patch => Level::Patch,
            ForceOptions::First => Level::Release,
        }
    } else {
        let mut latest_version = latest_version.commits()?;
        eprintln!("{:#?}", &latest_version);
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
