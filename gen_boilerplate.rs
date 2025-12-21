use git2::{Repository, Status, StatusOptions};

fn git_repo_has_uncommitted_changes(repo: &Repository) -> Result<bool, git2::Error> {
    let mut options = StatusOptions::new();
    options.include_untracked(true).recurse_untracked_dirs(true);
    Ok(repo
        .statuses(Some(&mut options))?
        .iter()
        .any(|file| file.status() != Status::CURRENT))
}

fn main() {
    let year = std::env::args()
        .nth(1)
        .expect("year should be provided as the first argument");
    let day = std::env::args()
        .nth(2)
        .expect("day should be provided as the second argument");

    let repo = Repository::init(".").expect("should always be run in the repository root");
    println!("repo state: {:?}", git_repo_has_uncommitted_changes(&repo));
}
