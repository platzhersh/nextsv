use std::ffi::OsString;
use std::fmt;

use clap::{Parser, ValueEnum};
use nextsv::{EnforceLevel, Error, ForceLevel, VersionCalculator};

const EXIT_NOT_CREATED_CODE: i32 = 1;
const EXIT_NOT_CALCULATED_CODE: i32 = 2;
const EXIT_MISSING_REQUIRED_CODE: i32 = 3;

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
    logging: clap_verbosity_flag::Verbosity,
    /// Force the calculation of the version number
    #[arg(short, long, value_enum)]
    force: Option<ForceOptions>,
    /// Prefix string to identify version number tags
    #[arg(short, long, value_parser, default_value = "v")]
    prefix: String,
    /// Report the level of the version number change
    #[arg(long)]
    level: bool,
    /// Report the version number
    #[arg(long)]
    number: bool,
    /// Require changes to these file before building release
    #[arg(short, long)]
    require: Vec<OsString>,
    /// Level at which required files should be enforced
    #[clap(short, long, default_value = "feature")]
    enforce_level: EnforceLevel,
    /// Check level meets minimum for setting
    ///
    /// This option can be used to check the calculated level
    /// meets a minimum before applying an update. The program
    /// exits with an error of the threshold is not met.
    #[clap(short, long, default_value = "other")]
    check: Option<EnforceLevel>,
}

fn main() {
    let args = Cli::parse();

    let mut builder = get_logging(args.logging.log_level_filter());
    builder.init();

    match (args.number, args.level) {
        (false, false) => log::info!("Calculating the next version level"),
        (false, true) => log::info!("Calculating the next version level"),
        (true, false) => log::info!("Calculating the next version number"),
        (true, true) => log::info!("Calculating the next version number and level"),
    };

    let latest_version = match VersionCalculator::new(&args.prefix) {
        Ok(v) => v,
        Err(e) => {
            log::error!("{}", e.to_string());
            std::process::exit(EXIT_NOT_CREATED_CODE)
        }
    };

    log::trace!("require: {:#?}", args.require);

    // Encapsulate the list of required files in an option
    let files = if args.require.is_empty() {
        Option::None
    } else {
        Option::Some(args.require)
    };

    match calculate(
        latest_version,
        args.force,
        args.level,
        args.number,
        files,
        args.enforce_level,
    ) {
        Ok(output) => {
            println!("{output}")
        }
        Err(e) => {
            log::error!("{}", &e.to_string());
            if let Error::MissingRequiredFile(f) = e {
                log::debug!("Required file {:?} not in the release candidate.", &f);
                std::process::exit(EXIT_MISSING_REQUIRED_CODE);
            }
            std::process::exit(EXIT_NOT_CALCULATED_CODE)
        }
    }
}

fn calculate(
    mut latest_version: VersionCalculator,
    force: Option<ForceOptions>,
    level: bool,
    number: bool,
    files: Option<Vec<OsString>>,
    require_level: EnforceLevel,
) -> Result<String, Error> {
    if let Some(f) = &force {
        log::debug!("Force option set to {}", f);
    };
    latest_version = latest_version.walk_commits()?;
    if let Some(f) = files {
        latest_version.has_required(f, require_level)?;
    }
    let (next_version, bump) = if let Some(svc) = force {
        match svc {
            ForceOptions::Major => latest_version.force(ForceLevel::Major).next_version(),
            ForceOptions::Minor => latest_version.force(ForceLevel::Minor).next_version(),
            ForceOptions::Patch => latest_version.force(ForceLevel::Patch).next_version(),
            ForceOptions::First => latest_version.promote_first()?,
        }
    } else {
        latest_version.next_version()
    };

    Ok(match (number, level) {
        (false, false) => format!("{bump}"),
        (false, true) => format!("{bump}"),
        (true, false) => format!("{next_version}"),
        (true, true) => format!("{next_version}\n{bump}"),
    })
}

pub fn get_logging(level: log::LevelFilter) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level);

    builder.format_timestamp_secs().format_module_path(false);

    builder
}
