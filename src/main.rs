use std::{
    fs::{self, File},
    io::{self, Cursor, Read},
    path::{Path, PathBuf},
};

fn main() -> io::Result<()> {
    let patch_dir = Path::new("./patches");

    let latest_patch_version = get_latest_patch_version();

    if patch_dir.is_dir() {
        let is_empty = patch_dir.read_dir()?.next().is_none();
        if is_empty {
            get_patch(patch_dir, latest_patch_version)
        }
    } else {
        get_patch(patch_dir, latest_patch_version);
    }
    Ok(())
}

fn get_patch(patch_dir: &Path, patch_version: String) {
    if !patch_dir.is_dir() {
        if let Err(err) = fs::create_dir(patch_dir) {
            println!("Error occured while creating directory: {err}");
        };
    }

    let mut target = String::from("https://ddragon.leagueoflegends.com/cdn/");

    let patch = format!("dragontail-{patch_version}.tgz");

    target.push_str(&patch);

    println!("Downloading patch {patch_version}");

    let res = reqwest::blocking::get(target);

    match res {
        Ok(res) => {
            let path: PathBuf = [patch_dir, Path::new(&patch)].iter().collect();

            if let Ok(mut latest_patch) = File::create(path) {
                match res.bytes() {
                    Ok(bytes) => {
                        let mut content = Cursor::new(bytes);
                        if let Err(err) = io::copy(&mut content, &mut latest_patch) {
                            println!("Error occured while writing file: {err}")
                        }
                    }
                    Err(err) => println!("Error occured while decoding response to bytes: {err}"),
                }
            };
        }
        Err(err) => println!("Error occured while getting latest patch: {err}"),
    }
}

fn get_latest_patch_version() -> String {
    let res = reqwest::blocking::get("https://ddragon.leagueoflegends.com/api/versions.json");
    let mut latest_patch = String::new();

    match res {
        Ok(mut res) => {
            let mut body = String::new();
            if let Err(err) = res.read_to_string(&mut body) {
                println!("Error occured while reading response to string: {err}");
            }

            for c in body.chars() {
                if c == ',' {
                    break;
                } else if c != '[' && c != '"' {
                    latest_patch.push(c)
                }
            }
        }
        Err(err) => print!("Error occured while getting latest patch version: {err}"),
    }

    latest_patch
}
