use anyhow::{Context, Result};
use reqwest;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use structopt::StructOpt;
use toml;

#[derive(Deserialize, Debug)]
struct Config {
    year: String,
    session: String,
}

fn read_config(config_file: &Path) -> Result<Config> {
    Ok(toml::from_str::<Config>(&fs::read_to_string(config_file)?)
        .with_context(|| format!("Error reading config file {:?}", config_file))?)
}

fn retrieve_input(config: &Config, day_number: usize) -> Result<String> {
    let url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        config.year, day_number
    );
    let client = reqwest::blocking::Client::new();
    Ok(client
        .get(&url)
        .header("Cookie", format!("session={}", config.session))
        .send()?
        .error_for_status()
        .context("Input not available (too soon?)")?
        .text()?)
}

fn get_inputs(base_folder: &Path, day_name: &str) -> Result<()> {
    let input_file = base_folder.join(day_name).join("input.txt");

    if input_file.exists() {
        println!("Input file {:?} exists, not retrieving", &input_file);
        return Ok(());
    }

    let input = {
        let config = read_config(&base_folder.join("aoc.toml"))?;
        let day_number = day_name[3..].parse::<usize>()?;
        retrieve_input(&config, day_number)
    }?;

    fs::write(&input_file, input)?;
    Ok(())
}

fn copy_skeleton(base_folder: &Path, day_name: &str) -> Result<()> {
    let day_folder = base_folder.join(day_name);
    if day_folder.exists() {
        println!("Day folder exists, not copying skeleton");
        return Ok(());
    }

    let skeleton_folder = base_folder.join("skeleton");
    let mut cargo: toml::Value = fs::read_to_string(skeleton_folder.join("Cargo.toml"))
        .context("Unable to read skeleton/Cargo.toml")?
        .parse()
        .context("While reading skeleton/Cargo.toml")?;

    cargo
        .get_mut("package")
        .unwrap()
        .as_table_mut()
        .unwrap()
        .insert(
            "name".to_string(),
            toml::Value::String(day_name.to_string()),
        );

    let src_folder = day_folder.join("src");
    fs::create_dir_all(&src_folder)?;
    fs::write(day_folder.join("Cargo.toml"), cargo.to_string())?;

    fs::copy(skeleton_folder.join("main.rs"), src_folder.join("main.rs"))?;

    Ok(())
}

/// An advent of code skeleton tool
#[derive(StructOpt, Debug)]
struct Opt {
    /// Day name. Format should be "day##"
    day_name: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let base_folder = std::env::current_dir()?;
    copy_skeleton(&base_folder, &opt.day_name).and_then(|_| get_inputs(&base_folder, &opt.day_name))
}
