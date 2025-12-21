use git2::{Repository, Status, StatusOptions};
use std::fs;
use toml_edit::{ArrayOfTables, DocumentMut, Item, Table};

const MAIN_RS_CONTENT: &str = r#"
#[path = "../../helpers.rs"]
mod helpers;

#[cfg(test)]
mod tests;

fn main() {
    println!("hello world");
}
"#;

const TESTS_RS_CONTENT: &str = r#"
static EXAMPLE_INPUT: &str = "";

use super::*;

#[test]
fn part1_example() {
    assert_eq!(true, true);
}

#[test]
fn part2_example() {
    assert_eq!(true, true);
}
"#;

fn git_repo_has_uncommitted_changes(repo: &Repository) -> Result<bool, git2::Error> {
    let mut options = StatusOptions::new();
    options.include_untracked(true).recurse_untracked_dirs(true);
    Ok(repo
        .statuses(Some(&mut options))?
        .iter()
        .any(|file| file.status() != Status::CURRENT))
}

struct Bin {
    name: String,
    path: String,
}

fn update_cargo_toml(new_bin: Bin) {
    let cargo_toml_str = fs::read_to_string("./cargo.toml").unwrap();
    let mut cargo_toml = cargo_toml_str.parse::<DocumentMut>().expect("invalid doc");
    let bins = cargo_toml
        .entry("bin")
        .or_insert(Item::ArrayOfTables(ArrayOfTables::new()))
        .as_array_of_tables_mut()
        .unwrap();

    // Create a new [[bin]] table
    let mut bin_table = Table::new();
    bin_table["name"] = Item::Value(new_bin.name.into());
    bin_table["path"] = Item::Value(new_bin.path.into());

    // Push it into the config
    bins.push(bin_table);

    // write the config back to disk
    fs::write("./cargo.toml", cargo_toml.to_string()).unwrap();
}

fn main() {
    let year = std::env::args()
        .nth(1)
        .expect("year should be provided as the first argument");
    let day = std::env::args()
        .nth(2)
        .expect("day should be provided as the second argument");

    let repo = Repository::init(".").expect("should always be run in the repository root");
    if git_repo_has_uncommitted_changes(&repo).unwrap() {
        //panic!("Repository should be clean before generating files")
    }

    // create the parent directories if they don't already exist
    let path = format!("./{year}/{day}");
    std::fs::create_dir_all(&path).unwrap();

    // write code files
    let main_rs_path = format!("{path}/main.rs");
    fs::write(&main_rs_path, MAIN_RS_CONTENT).unwrap();
    fs::write(format!("{path}/tests.rs"), TESTS_RS_CONTENT).unwrap();

    update_cargo_toml(Bin {
        name: format!("{year}_{day}"),
        path: main_rs_path,
    });
}
