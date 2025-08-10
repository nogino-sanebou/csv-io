mod csv;

use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::csv::{CsvBody, CsvData, CsvFile, CsvHeader, CsvRow};

pub fn read(path: &str) -> Result<CsvFile, String> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(format!("ファイルの読み込みに失敗しました。{:?}", e)),
    };
    let mut buffer = BufReader::new(file);
    let mut line= String::from("");

    // 最初の一行目はヘッダにする
    let mut csv_header = CsvHeader::new();
    buffer.read_line(&mut line).unwrap();
    for data in line.split(",") {
        csv_header.append(data);
    }

    // 2行目以降はデータにする
    let mut csv_body = CsvBody::new();
    while let Ok(_) = buffer.read_line(&mut line) {
        let mut csv_row = CsvRow::new();
        for (index, data) in line.split(",").enumerate() {
            let csv_data = CsvData::new(
                csv_header.get(index),
                data
            );
            csv_row.append(csv_data);
        }
        csv_body.append(csv_row);
    }

    Ok(CsvFile::new(csv_header, csv_body))
}

pub fn write(path: &str) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
