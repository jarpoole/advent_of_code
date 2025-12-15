use std::collections::HashMap;
use std::error::Error;
use std::fs::{self};
use std::hash::{Hash, Hasher};

fn hash(string: &str) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    string.hash(&mut hasher);
    hasher.finish().to_string()
}

fn get_page(url: &str) -> Result<String, Box<dyn Error>> {
    let session_cookie = format!(
        "session={}",
        read_dotenv()?.get("SESSION").ok_or(
            "SESSION key missing in .env file. Please retrieve this from the cookie header on AoC"
        )?
    );
    let cache_dir = "cache";
    let cache_filename = format!("{}/{}.cache", &cache_dir, hash(url));
    if let Ok(exists) = fs::exists(&cache_filename)
        && exists
        && let Ok(cached_data) = fs::read_to_string(&cache_filename)
    {
        return Ok(cached_data);
    }
    let client = reqwest::blocking::Client::new();
    let data = client
        .get(url)
        .header(reqwest::header::COOKIE, session_cookie)
        .send()?
        .error_for_status()?
        .text()?;

    if let Ok(exists) = fs::exists(&cache_dir)
        && !exists
    {
        fs::create_dir(&cache_dir)?;
    }
    fs::write(cache_filename, &data)?;
    Ok(data)
}

pub fn get_input(year: u16, number: u8) -> Result<String, Box<dyn Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, number);
    get_page(&url)
}

pub fn read_dotenv() -> Result<HashMap<String, String>, Box<dyn Error>> {
    Ok(fs::read_to_string(".env")?
        .split("\n")
        .filter_map(|line| {
            let mut x = line.split("=");
            return Option::zip(
                x.nth(0).map(|s| s.to_owned()),
                x.nth(0).map(|s| s.to_owned()),
            );
        })
        .collect::<HashMap<String, String>>())
}
