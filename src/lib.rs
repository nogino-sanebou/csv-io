use std::fs::File;

pub fn read_file(path: &str) -> Result<CsvFile, String> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(format!("ファイルの読み込みに失敗しました。{:?}", e)),
    };
    todo!()
}

pub struct CsvFile {
    csv_header: CsvHeader,
    csv_body: CsvBody,
    csv_row: CsvRow,
    csv_data: CsvData,
}

struct CsvHeader {
    name: Vec<String>,
}

struct CsvBody {
    rows: Vec<CsvRow>,
}

struct CsvRow {
    data: Vec<CsvData>,
}

struct CsvData {
    header_name: String,
    value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
