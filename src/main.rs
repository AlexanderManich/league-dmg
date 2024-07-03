use std::{
    fs::{self, File},
    io::{self, Cursor, Read},
    path::{Path, PathBuf},
};

use clap::{Arg, ArgAction, Command};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    let matches = Command::new("League Damage")
        .version("1.0")
        .author("Lennonnn")
        .arg(
            Arg::new("update")
                .short('u')
                .long("update")
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("update") {
        let latest_patch_version = &get_latest_patch_version()?;

        let patch_dir = Path::new("./patch/");

        if !patch_dir.is_dir() {
            fs::create_dir(patch_dir)?
        }

        let entries = fs::read_dir(patch_dir)?
            .map(|res| res.map(|entry| entry.path()))
            .collect::<io::Result<Vec<_>>>()?;

        let latest_patch = format!("dragontail-{latest_patch_version}.tgz");
        let latest_patch_path: PathBuf = [patch_dir, Path::new(&latest_patch)].iter().collect();

        if !entries.is_empty() {
            for entry in entries {
                match entry.extension() {
                    Some(extension) => {
                        if extension == "tgz" && entry != latest_patch_path {
                            fs::remove_dir_all(patch_dir)?;
                            get_patch(patch_dir, latest_patch_version.to_owned())?;
                        } else {
                            println!("Patch is up to date");
                            break;
                        }
                    }
                    None => continue,
                }
            }
        } else {
            get_patch(patch_dir, latest_patch_version.to_owned())?;
        }
    }

    Ok(())
}

fn get_patch(patch_dir: &Path, patch_version: String) -> Result<()> {
    let mut target = String::from("https://ddragon.leagueoflegends.com/cdn/");

    let patch = format!("dragontail-{patch_version}.tgz");

    target.push_str(&patch);

    println!("Fetching patch {patch_version}");

    let client = reqwest::blocking::Client::builder().timeout(None).build()?;

    let res = client.get(target).send()?;
    let path: PathBuf = [patch_dir, Path::new(&patch)].iter().collect();

    let mut file = File::create(path)?;
    let mut content = Cursor::new(res.bytes()?);
    io::copy(&mut content, &mut file)?;

    println!("Finished fetching patch");
    Ok(())
}

fn get_latest_patch_version() -> Result<String> {
    println!("Fetching latest patch version");

    let client = reqwest::blocking::Client::builder().timeout(None).build()?;

    let mut res = client
        .get("https://ddragon.leagueoflegends.com/api/versions.json")
        .send()?;

    let mut latest_patch = String::new();

    let mut body = String::new();
    res.read_to_string(&mut body)?;

    for c in body.chars() {
        if c == ',' {
            break;
        } else if c != '[' && c != '"' {
            latest_patch.push(c)
        }
    }

    println!("Latest patch is: {latest_patch}");

    Ok(latest_patch)
}
