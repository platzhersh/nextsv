use nextsv_lib::Error;
use nextsv_lib::VersionTag;

fn main() -> Result<(), Error> {
    println!("Next Version\n------------\n");

    // What is the latest tag?
    // What are the conventional commits since that tag?
    let latest_version = VersionTag::latest("v")?.commits()?;

    println!(
        "Conventional commits by type for version: {}",
        latest_version.name()
    );
    println!("  feat:       {}", latest_version.feat_commits());
    println!("  fix:        {}", latest_version.fix_commits());
    println!("  docs:       {}", latest_version.docs_commits());
    println!("  chore:      {}", latest_version.chore_commits());
    println!("  refactor:   {}", latest_version.refactor_commits());
    if latest_version.breaking() {
        println!("One or more breaking changes");
    } else {
        println!("No breaking change.");
    }

    Ok(())
}
