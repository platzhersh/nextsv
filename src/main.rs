use clap::Parser;
use nextsv_lib::Error;
use nextsv_lib::VersionTag;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct NextsvCliArgs {
    // #[clap(short, long, value_parser)]
    // name: String,
    // #[clap(short, long, value_parser, default_value_t = 1)]
    // count: u8,
    #[clap(short, long, value_parser, default_value_t = false)]
    verbose: bool,
}

fn main() -> Result<(), Error> {
    // What is the latest tag?
    // What are the conventional commits since that tag?
    let args = NextsvCliArgs::parse();

    let latest_version = VersionTag::latest("v")?.commits()?;
    if args.verbose {
        eprintln!("Next Version\n------------\n");
        eprintln!(
            "Conventional commits by type for version: {}",
            latest_version.name()
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
    println!("{}", latest_version.next());
    Ok(())
}
