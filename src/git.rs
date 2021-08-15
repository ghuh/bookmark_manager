use std::path::Path;
use git2::{Repository, ObjectType, Commit, IndexAddOption};
use anyhow::{Result, Context};
use crate::cli_output::utils::print_warning;

pub struct Git {
    repo: Repository,
}

impl Git {
    pub fn new(csv: &str) -> Option<Self> {
        let mut dir = Path::new(csv);

        // Traverse the directory tree looking for the git repo
        let repo = loop {
            match dir.parent() {
                None => {
                    print_warning("It appears the CSV file is not in a git repo. Use --no-commit to suppress this message");
                    return None;
                }
                Some(new_dir) => {
                    dir = new_dir;
                    let repo_result = Repository::open(dir);
                    if let Ok(repo) = repo_result {
                        break repo;
                    }
                }
            }
        };

        Some(Self { repo })
    }

    pub fn is_clean(&self) -> Result<bool> {
        let statuses = self.repo.statuses(None).context("Could not get git status")?;
        // https://github.com/rust-lang/git2-rs/blob/master/examples/status.rs#L174
        let is_dirty = statuses.iter().any(|e| e.status() != git2::Status::CURRENT);

        Ok(!is_dirty)
    }

    pub fn add_and_commit_bookmark(&self, url: &str, description: &str) -> Result<()> {
        self.add_and_commit(format!("Add bookmark for {} - {}", url, description).as_str())?;
        Ok(())
    }

    // https://zsiciarz.github.io/24daysofrust/book/vol2/day16.html
    // https://github.com/rust-lang/git2-rs/blob/master/examples/add.rs#L71
    pub fn add_and_commit(&self, msg: &str) -> Result<()> {
        // add
        let oid = {
            let mut index = self.repo.index()?;

            // Since we check to make sure that there are no previous uncommitted changes, it is safe to add all
            index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;

            // I don't know why we need to double write, but it is necessary in order for the commit to go through
            index.write()?;

            index.write_tree()?
        };

        // commit
        let tree = self.repo.find_tree(oid)?;
        let parent_commit = self.find_last_commit()?;
        let signature = self.repo.signature()?;
        self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            msg,
            &tree,
            &[&parent_commit],
        )?;

        Ok(())
    }

    fn find_last_commit(&self) -> Result<Commit, git2::Error> {
        let obj = self.repo.head()?.resolve()?.peel(ObjectType::Commit)?;
        obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find last commit"))
    }
}