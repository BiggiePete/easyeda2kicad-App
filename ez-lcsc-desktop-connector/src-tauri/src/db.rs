use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use uuid::Uuid;
// TODO, fix issue with DB, if the headers do not exist, the db wont be able to be written to properly
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub proj_name: String,
    pub dir: String,
}

const FILE_PATH: &str = "db.csv";

// Helper function to check if a project name already exists
fn project_name_exists(proj_name: &str) -> Result<bool, Box<dyn Error>> {
    let records = get_all_records()?;
    Ok(records.iter().any(|project| project.proj_name == proj_name))
}

// Helper function to check if the file is empty
fn is_file_empty() -> bool {
    match File::open(FILE_PATH) {
        Ok(file) => {
            let metadata = file
                .metadata()
                .unwrap_or_else(|_| panic!("Failed to get metadata for {}", FILE_PATH));
            metadata.len() == 0
        }
        Err(_) => true, // File doesn't exist, consider it "empty"
    }
}

pub fn add_record(record: &Project) -> Result<(), Box<dyn Error>> {
    // Check if project with the same name already exists
    if project_name_exists(&record.proj_name)? {
        return Err(format!("Project with name '{}' already exists", record.proj_name).into());
    }

    let file_exists = Path::new(FILE_PATH).exists();
    let is_empty = is_file_empty();

    let file = OpenOptions::new()
        .write(true)
        .append(!is_empty) // Only append if the file is not empty
        .create(true)
        .open(FILE_PATH)?;

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(file_exists && is_empty || !file_exists)
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

    // If we have records to write, write them with headers
    if !records.is_empty() {
        let mut wtr = csv::Writer::from_writer(BufWriter::new(file));
        for record in records {
            wtr.serialize(record)?;
        }
        wtr.flush()?;
    }

    Ok(())
}

pub fn get_record_by_id(record_id: String) -> Result<Option<Project>, Box<dyn Error>> {
    if !Path::new(FILE_PATH).exists() || is_file_empty() {
        return Ok(None);
    }

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
    // Handle the case when the file doesn't exist yet or is empty
    if !Path::new(FILE_PATH).exists() || is_file_empty() {
        return Ok(Vec::new());
    }

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
