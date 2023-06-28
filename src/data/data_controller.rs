use calamine::{open_workbook, DataType, Range, Reader, Xlsx, XlsxError};
use std::env;
use std::path::{Path, PathBuf};

pub fn reading_file(s: &str) -> Result<Range<DataType>, XlsxError> {
    let mut execel: Xlsx<_> = open_workbook(s).unwrap();
    match execel.worksheet_range("Sheet1") {
        Some(val) => val,
        None => panic!("not an xlxs file"),
    }
}

pub fn get_file() -> Vec<PathBuf> {
    let dir = env::current_dir()
        .unwrap()
        .join(Path::new("src"))
        .as_path()
        .join(Path::new("data"));

    let mut files = Vec::new();

    for i in dir.read_dir().unwrap() {
        let file = i.unwrap();
        if file.file_name().to_str().unwrap().contains(".xlsx") {
            files.push(file.path());
        }
    }
    files.sort();
    // println!("{:?}", files);
    files
}

pub fn merging_content_in_files(files: &Vec<PathBuf>) -> Vec<Vec<DataType>> {
    let mut merged_rows = Vec::new();
    for file in files {
        let range = reading_file(file.to_str().unwrap()).unwrap();
        let rows: Vec<Vec<DataType>> = range.rows().map(|row| row.to_vec()).collect();
        merged_rows.extend(rows);
    }
    // println!("{:?}", merged_rows);
    merged_rows
}
