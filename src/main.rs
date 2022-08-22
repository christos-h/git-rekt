use std::{env};
use std::fs::File;
use std::io::{Write};
use std::path::{Path, PathBuf};
use git2::{Commit, ObjectType, Repository, Signature, Time};
use structopt::StructOpt;
use anyhow::Result;
use rand::Rng;

static SECONDS_IN_YEAR: i64 = 86400 * 365;

fn main() -> Result<()> {
    rekt(Args::from_args())
}

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

fn rekt(args: Args) -> Result<()> {
    let path = env::current_dir()?.join(&args.repo_name);
    let repo = Repository::init(&path)?;
    let name = args.username;
    let email = args.email;

    let mut index = repo.index()?;

    let file_path = path.join("dummy");
    let mut file = File::create(&file_path)?;

    let mut ts = chrono::Utc::now().timestamp() - (SECONDS_IN_YEAR * args.years);

    let mut rng = rand::thread_rng();

    while ts < chrono::Utc::now().timestamp() {
        let times_per_day = rng.gen_range(1..10);
        for _ in 1..times_per_day {
            let rand_bytes: [u8; 4] = rng.gen();
            file.write_all(&rand_bytes)?;
            file.flush()?;

            index.add_path(Path::new("dummy"))?;

            let signature = Signature::new(
                &name,
                &email,
                &Time::new(ts, 0),
            )?;

            let oid = index.write_tree()?;

            let tree = repo.find_tree(oid)?;

            match find_last_commit(&repo) {
                Ok(commit) => {
                    repo.commit(
                        Some("HEAD"),
                        &signature,
                        &signature,
                        "lmao",
                        &tree,
                        &[&commit],
                    )?;
                }
                Err(_) => {
                    repo.commit(
                        Some("HEAD"),
                        &signature,
                        &signature,
                        "git rekt",
                        &tree,
                        &[],
                    )?;
                }
            };
        }
        ts += 86400;
    }

    Ok(())
}

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt()]
    repo_name: PathBuf,
    #[structopt(long)]
    username: String,
    #[structopt(long)]
    email: String,
    #[structopt(long, default_value = "5")]
    years: i64,
}