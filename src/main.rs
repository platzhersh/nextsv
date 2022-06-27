use clap::{Parser, ValueEnum};
use nextsv_lib::Error;
use nextsv_lib::VersionTag;

#[derive(ValueEnum, Debug, Clone)]
enum ForceOptions {
    Major,
    Minor,
    Patch,
    First,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser, default_value_t = false)]
    verbose: bool,
    #[clap(short, long, value_enum)]
    force: Option<ForceOptions>,
}

fn main() -> Result<(), Error> {
    // What is the latest tag?
    // What are the conventional commits since that tag?
    let args = Cli::parse();

    let mut latest_version = VersionTag::latest("v")?;

    if args.verbose {
        eprintln!("Next Version\n------------\n");
        eprintln!(
            "Conventional commits by type for version: {}",
            &latest_version.name()
        );
        eprintln!("  feat:       {}", latest_version.feat_commits());
        eprintln!("  fix:        {}", latest_version.fix_commits());
        eprintln!("  docs:       {}", latest_version.docs_commits());
        eprintln!("  chore:      {}", latest_version.chore_commits());
        eprintln!("  refactor:   {}", latest_version.refactor_commits());
        if latest_version.breaking() {
            eprintln!("One or more breaking changes");
        } else {
            eprintln!("No breaking change.");
        }
        eprint!("Next Version: ");
    }

    let next_version = if let Some(svc) = args.force {
        match svc {
            ForceOptions::Major => latest_version.force_major().next(),
            ForceOptions::Minor => latest_version.force_minor().next(),
            ForceOptions::Patch => latest_version.force_patch().next(),
            ForceOptions::First => latest_version.promote_first()?.name(),
        }
    } else {
        latest_version.commits()?.next()
    };

    println!("{}", next_version);

    Ok(())
}
