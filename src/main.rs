use nextsv_lib::Error;

fn main() -> Result<(), Error> {
    println!("Hello World");

    // What is the latest version tag?
    let latest_version = nextsv_lib::latest_version_tag()?;
    println!("Latest version is: {:?}", &latest_version);

    // What are the conventional commits since that tag?
    let conventional_commits = nextsv_lib::conventional_commits_to_tag(latest_version.name())?;
    println!("Conventional Commits: {:?}", conventional_commits);

    let repo = git2::Repository::open(".")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(git2::Sort::NONE)?;
    revwalk.push_head()?;

    macro_rules! filter_try {
        ($e:expr) => {
            match $e {
                Ok(t) => t,
                Err(e) => return Some(Err(e)),
            }
        };
    }

    #[allow(clippy::unnecessary_filter_map)]
    let revwalk = revwalk.filter_map(|id| {
        let id = filter_try!(id);
        let commit = repo.find_commit(id);
        let commit = filter_try!(commit);
        Some(Ok(commit))
    });

    // let mut counter = 0;
    let mut count_feature = 0;
    let mut count_fix = 0;
    let mut count_docs = 0;
    let mut count_chore = 0;
    let mut count_refactor = 0;
    let mut breaking = false;

    for commit in revwalk {
        let commit = commit?;
        // counter += 1;
        // Break once we find the latest version tag
        if commit.id() == latest_version.id() {
            break;
        }

        if let Ok(conventional) =
            git_conventional::Commit::parse(commit.summary().take().unwrap_or("default"))
        {
            if conventional.type_() == git_conventional::Type::FEAT {
                count_feature += 1;
            }

            if conventional.type_() == git_conventional::Type::FIX {
                count_fix += 1;
            }

            if conventional.type_() == git_conventional::Type::DOCS {
                count_docs += 1;
            }

            if conventional.type_() == git_conventional::Type::CHORE {
                count_chore += 1;
            }

            if conventional.type_() == git_conventional::Type::REFACTOR {
                count_refactor += 1;
            }
            if !breaking {
                breaking = conventional.breaking();
            }

            // if let Some(summary) = &commit.summary() {
            //     println!(
            //         "{}: {} {}",
            //         counter,
            //         commit.id().to_string().get(..6).unwrap_or(""),
            //         summary
            //     );
            // }
        }
    }
    println!("Conventional commits by type:");
    println!("  feat:       {}", count_feature);
    println!("  fix:        {}", count_fix);
    println!("  docs:       {}", &count_docs);
    println!("  chore:      {}", count_chore);
    println!("  refactor:   {}", count_refactor);
    if breaking {
        println!("One or more breaking changes")
    } else {
        println!("No breaking change.")
    }
    Ok(())
}
