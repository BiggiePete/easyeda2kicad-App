use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub proj_name: String,
    pub dir: String,
}

const FILE_PATH: &str = "db.csv";

pub fn add_record(record: &Project) -> Result<(), Box<dyn Error>> {
    let file_exists = Path::new(FILE_PATH).exists();

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILE_PATH)?;

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(!file_exists)
        .from_writer(file);

    wtr.serialize(record)?;
    wtr.flush()?;
    Ok(())
}

pub fn remove_record_by_id(record_id: String) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open(FILE_PATH)?;
    let mut rdr = csv::Reader::from_reader(BufReader::new(file));
    let records: Vec<Project> = rdr
        .deserialize()
        .filter_map(Result::ok)
        .filter(|rec: &Project| rec.id != record_id)
        .collect();

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_PATH)?;
    let mut wtr = csv::Writer::from_writer(BufWriter::new(file));
    for record in records {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn get_record_by_id(record_id: String) -> Result<Option<Project>, Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open(FILE_PATH)?;
    let mut rdr = csv::Reader::from_reader(BufReader::new(file));
    for result in rdr.deserialize() {
        let record: Project = result?;
        if record.id == record_id {
            return Ok(Some(record));
        }
    }
    Ok(None)
}

pub fn get_all_records() -> Result<Vec<Project>, Box<dyn Error>> {
    let file = File::open(FILE_PATH)?;
    let mut rdr = csv::Reader::from_reader(BufReader::new(file));
    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: Project = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn add_record_with_details(proj_name: &str, dir: &str) -> Result<(), Box<dyn Error>> {
    let new_record = Project {
        id: Uuid::new_v4().to_string(),
        proj_name: proj_name.to_string(),
        dir: dir.to_string(),
    };

    add_record(&new_record)
}
