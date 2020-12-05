use serde::Deserialize;
use std::num::ParseIntError;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use toml;
pub use anyhow::{Context, Result};  // For forward use, fn main() -> Result<()> 
use std::result::Result as StdResult;
use thiserror::Error;
use reqwest;

#[derive(Error, Debug)]
pub enum AOCError {
    #[error("No config")]
    NoConfig {start_folder: PathBuf,},
    #[error("Bad config")]
    BadConfig {source: toml::de::Error,},
    #[error("Bad start folder")]
    BadStartFolder {start_folder: PathBuf,},
    #[error("No input")]
    NoInput,
    #[error("Bad day name")]
    BadDayName {source: ParseIntError,},
    #[error("HTTP error")]
    HttpError {source: reqwest::Error, },
    #[error(transparent)]
    Other(#[from] io::Error),
}

#[derive(Deserialize, Debug)]
struct Config {
    year: String,
    session: String,
}

fn read_config(config_file: &Path) -> StdResult<Config, AOCError> {
    let content = fs::read_to_string(config_file)?;
    toml::from_str::<Config>(&content)
    .map_err(|source| AOCError::BadConfig{source})
}

fn retrieve_input(config: &Config, day_number: usize) -> StdResult<String, AOCError> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", config.year, day_number);
    let client = reqwest::blocking::Client::new();
    client.get(&url)
    .header("Cookie", format!("session={}", config.session))
    .send().map_err(|source| AOCError::HttpError{source})?
    .text().map_err(|source| AOCError::HttpError{source})
}

pub fn get_inputs(start_folder: &Path) -> StdResult<String, AOCError>  {
    // TODO: use input folder for reference instead
    let day_folder = start_folder
    .ancestors()
    .filter(|ff| ff.with_file_name("aoc.toml").exists())
    .next()
    .ok_or_else(|| AOCError::NoConfig{start_folder: start_folder.into()})?;

    let day_folder_name = day_folder
    .components().last()
    .ok_or_else(|| AOCError::BadStartFolder{start_folder: start_folder.into()})?;

    let inputs_file = day_folder
    .with_file_name("inputs")
    .join(day_folder_name)
    .with_extension("txt");

    fs::read_to_string(&inputs_file).or_else(|_|{
        let config = read_config(&day_folder.with_file_name("aoc.toml"))?;
        let day_number = day_folder_name
        .as_os_str().to_string_lossy()[3..].parse::<usize>()
        .map_err(|e| AOCError::BadDayName{ source: e })?;
 
        let input = retrieve_input(&config, day_number)?;
        fs::write(inputs_file, &input)?;
        Ok(input)
    })
}

/// Return inputs from cache or retrieve them from aoc site
///
/// Searches for `aoc.toml` upwards from current working directory.
/// Cache folder is in folder `inputs` next to `aoc.toml` config file.
pub fn get_inputs_pwd() -> StdResult<String, AOCError> {
    get_inputs(&std::env::current_dir()?)
}