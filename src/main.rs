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
        #[clap(short, long, value_parser, default_value_t = false)]
        verbose: bool,
        #[clap(short, long, value_enum)]
        force: Option<ForceOptions>,
        #[clap(short, long, value_parser, default_value = "v")]
        prefix: String,
    },
    /// Calculate the level for the next semantic version increase
    #[clap()]
    Level {
        #[clap(short, long, value_parser, default_value_t = false)]
        verbose: bool,
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
            verbose,
            force,
            prefix,
        } => {
            let latest_version = VersionCalculator::new(&prefix)?;
            verbosity(verbose, &latest_version);
            version(latest_version, force)?;
        }
        Commands::Level {
            verbose,
            force,
            prefix,
        } => {
            let latest_version = VersionCalculator::new(&prefix)?;
            verbosity(verbose, &latest_version);
            level(latest_version, force)?;
        }
    }

    Ok(())
}

fn verbosity(verbose: bool, latest_version: &VersionCalculator) {
    if verbose {
        eprintln!("Next Version\n------------\n");
        eprintln!(
            "Conventional commits by type for version: {}",
            &latest_version.name()
        );
        eprintln!("  feat:       {}", latest_version.types("feat"));
        eprintln!("  fix:        {}", latest_version.types("fix"));
        eprintln!("  docs:       {}", latest_version.types("docs"));
        eprintln!("  chore:      {}", latest_version.types("chore"));
        eprintln!("  refactor:   {}", latest_version.types("refactor"));
        if latest_version.breaking() {
            eprintln!("One or more breaking changes");
        } else {
            eprintln!("No breaking change.");
        }
        eprint!("Next Version: ");
    }
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
