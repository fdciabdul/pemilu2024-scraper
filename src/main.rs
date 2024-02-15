use std::error::Error;
use std::fs::File;

mod models;
mod scraper;
mod logger; 

fn main() -> Result<(), Box<dyn Error>> {
    logger::init(); 

    let client = reqwest::blocking::Client::new();
    let mut wtr = csv::Writer::from_writer(File::create("data_rekap.csv")?);

    scraper::scrape_election_data(&client, &mut wtr)?;

    Ok(())
}
