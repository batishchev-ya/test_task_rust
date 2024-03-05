use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;
use reqwest::blocking::Response;
use sha256::digest;
use reqwest;
use dotenv::dotenv;

fn main() -> io::Result<()> {
    let start_time: Instant = Instant::now();
    println!("Started!");
    dotenv().expect("please create .env file!");
    // по-хорошему, тут нужно созать конфиг и все складывать в него (https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html)
    let input_filename: &String = &env::var("INPUT_FILENAME").expect("Please provide INPUT_FILENAME env var!");
    let dirname: &String = &env::var("DIRNAME").expect("Please provide DIRNAME env var!");
    create_directory_if_not_exists(dirname)?;
    let urls: Vec<String> = read_urls(input_filename)?;
    urls.par_iter().enumerate().for_each(|(line_number, url)| {
        let start_time_thread: Instant = Instant::now();
        if let Err(err) = download_file(url, &(line_number as i32), dirname) {
            eprintln!("Error downloading file from {}: {}", url, err);
        }
        println!("Finished downloading file {} in {} ms", url, start_time_thread.elapsed().as_millis());
    });
    println!("Finished! Total time of execution: {} ms", start_time.elapsed().as_millis());
    Ok(())
}

fn create_directory_if_not_exists(dirname: &str) -> io::Result<()> {
    if !Path::new(dirname).exists() {
        std::fs::create_dir(dirname)?;
        println!("Directory '{}' created successfully", dirname);
    }
    Ok(())
}

fn read_urls(filename: &str) -> io::Result<Vec<String>> {
    let file: File = File::open(filename)?;
    let urls: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect();
    Ok(urls)
}

fn download_file(url: &str, number: &i32, dirname: &String) -> io::Result<()> {
    let response: Response = reqwest::blocking::get(url).unwrap();
    let header: &str = response.headers().get("Content-Type").unwrap().to_str().unwrap();
    let extension: &str = header.split('/').last().unwrap_or("jpeg");
    let filename: String = format!("{}_{}.{}", number, calculate_hash(url), extension);
    let path: PathBuf = Path::new(dirname).join(filename);
    let mut file: File = File::create(path)?;
    io::copy(&mut response.bytes().unwrap().as_ref(), &mut file)?;
    // println!("File {} downloaded successfully", filename);
    Ok(())
}

fn calculate_hash(str: &str) -> String {
    digest(str)
}
