use crate::models::{Area, ElectionData};
use csv::Writer;
use log::{info,warn};
use reqwest::blocking::Client;
use std::error::Error;

pub fn scrape_election_data(client: &Client, wtr: &mut Writer<std::fs::File>) -> Result<(), Box<dyn Error>> {
    let provinces_url = "https://sirekap-obj-data.kpu.go.id/wilayah/pemilu/ppwp/0.json";
    info!("Memulai proses scraping.");
    let provinces: Vec<Area> = client.get(provinces_url).send()?.json()?;

    for province in provinces.iter() {
        info!("Proses scraping wilayah : {}", province.nama);
        let cities_url = format!("https://sirekap-obj-data.kpu.go.id/wilayah/pemilu/ppwp/{}.json", province.kode);
        let cities: Vec<Area> = client.get(&cities_url).send()?.json()?;

        for city in cities.iter() {
            warn!("Proses kota : {}", province.nama);
            let districts_url = format!("https://sirekap-obj-data.kpu.go.id/wilayah/pemilu/ppwp/{}/{}.json", province.kode, city.kode);
            
            let districts: Vec<Area> = client.get(&districts_url).send()?.json()?;

            for district in districts.iter() {
                info!("Proses kabupaten : {}, city: {}", province.nama, city.nama);
                let subdistricts_url = format!("https://sirekap-obj-data.kpu.go.id/wilayah/pemilu/ppwp/{}/{}/{}.json", province.kode, city.kode, district.kode);
                let subdistricts: Vec<Area> = client.get(&subdistricts_url).send()?.json()?;

                for subdistrict in subdistricts.iter() {
                    info!("Proses kecamatan di provinsi: {}, kota: {}, kecamatan: {}", province.nama, city.nama, district.nama);
                    let villages_url = format!("https://sirekap-obj-data.kpu.go.id/wilayah/pemilu/ppwp/{}/{}/{}/{}.json", province.kode, city.kode, district.kode, subdistrict.kode);
                    let villages: Vec<Area> = client.get(&villages_url).send()?.json()?;

                    for village in villages.iter() {
                        info!("Proses desa di provinsi: {}, kota: {}, kecamatan: {}, desa: {}", province.nama, city.nama, district.nama, subdistrict.nama);
                        let election_data_url = format!("https://sirekap-obj-data.kpu.go.id/pemilu/hhcw/ppwp/{}/{}/{}/{}/{}.json", province.kode, city.kode, district.kode, subdistrict.kode, village.kode);
                        match client.get(&election_data_url).send()?.json::<ElectionData>() {
                            Ok(election_data) => {
                                if let Some(table) = election_data.table {
                                    for (_tps, votes) in table.iter() {
                                        let paslon1 = votes.get("100025").unwrap_or(&0);
                                        let paslon2 = votes.get("100026").unwrap_or(&0);
                                        let paslon3 = votes.get("100027").unwrap_or(&0);

                                        wtr.write_record(&[
                                            &province.nama,
                                            &city.nama,
                                            &district.nama,
                                            &subdistrict.nama,
                                            &village.nama,
                                            &paslon1.to_string(),
                                            &paslon2.to_string(),
                                            &paslon3.to_string(),
                                        ])?;
                                    }
                                }
                            },
                            Err(_) => eprintln!("Failed to deserialize election data for village: {}", village.nama),
                        }
                    }
                }
            }
        }
    }

    wtr.flush()?;
    Ok(())
}
