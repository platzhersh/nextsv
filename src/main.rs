use std::ffi::OsString;
use std::fmt;

use clap::{Parser, ValueEnum};
use nextsv::{Error, ForceLevel, Level, Semantic, TypeHierarchy, VersionCalculator};

const EXIT_SUCCESS: i32 = 0;
// const EXIT_UNEXPECTED_ERROR: i32 = 10;
const EXIT_NOT_CREATED_CODE: i32 = 11;
const EXIT_NOT_CALCULATED_CODE: i32 = 12;
const EXIT_MISSING_REQUIRED_CODE: i32 = 13;
const EXIT_NOT_REQUIRED_LEVEL: i32 = 14;

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
    enforce_level: TypeHierarchy,
    /// Check level meets minimum for setting
    ///
    /// This option can be used to check the calculated level
    /// meets a minimum before applying an update. The program
    /// exits with an error if the threshold is not met.
    #[clap(short, long)]
    check: Option<TypeHierarchy>,
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
        // args.level,
        // args.number,
        files,
        args.enforce_level,
    ) {
        Ok(output) => match args.check {
            Some(minimum_level) => {
                log::debug!("level expected is {:?}", &minimum_level);
                log::debug!("level reported is {:?}", &output.2,);
                if let Some(type_level) = output.2 {
                    if type_level >= minimum_level {
                        log::debug!("the minimum level is met");
                        std::process::exit(EXIT_SUCCESS)
                    } else {
                        log::debug!("the minimum level is not met");
                        std::process::exit(EXIT_NOT_REQUIRED_LEVEL)
                    };
                }
            }
            None => {
                log::debug!("not checking so print the output");
                print_output(args.number, args.level, output.0, output.1)
            }
        },
        Err(e) => {
            log::error!("{}", &e.to_string());
            if let Error::MissingRequiredFile(f) = e {
                log::debug!("Required file {:?} not in the release candidate.", &f);
                std::process::exit(EXIT_MISSING_REQUIRED_CODE);
            }
            std::process::exit(EXIT_NOT_CALCULATED_CODE)
        }
    };
}

fn calculate(
    mut latest_version: VersionCalculator,
    force: Option<ForceOptions>,
    // level: bool,
    // number: bool,
    files: Option<Vec<OsString>>,
    require_level: TypeHierarchy,
) -> Result<(Level, Semantic, Option<TypeHierarchy>), Error> {
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

    let top_level = latest_version.top_level();

    Ok((bump, next_version, top_level))
}

pub fn get_logging(level: log::LevelFilter) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level);

    builder.format_timestamp_secs().format_module_path(false);

    builder
}

/// Print the output from the calculation
///
fn print_output(number: bool, level: bool, bump: Level, next_version: Semantic) {
    match (number, level) {
        (false, false) => println!("{bump}"),
        (false, true) => println!("{bump}"),
        (true, false) => println!("{next_version}"),
        (true, true) => println!("{next_version}\n{bump}"),
    }
}
