pub struct CsvFile {
    csv_header: CsvHeader,
    csv_body: CsvBody,
}
impl CsvFile {
    pub fn new(csv_header: CsvHeader, csv_body: CsvBody) -> Self {
        Self {
            csv_header,
            csv_body,
        }
    }

    pub fn get(&self, header_name: &str, row_index: usize) -> String {
        let row = self.csv_body.get(row_index);
        row.get(header_name)
    }

    pub fn append() {

    }

    pub fn insert(index: usize) {

    }

    pub fn update(index: usize) {

    }

    pub fn remove(index: usize) {

    }
}

pub struct CsvHeader {
    name: Vec<String>,
}
impl CsvHeader {
    pub fn new() -> Self{
        Self {name: Vec::new()}
    }

    pub fn append(&mut self, data: &str) {
        self.name.push(data.to_string());
    }

    pub fn get(&self, index: usize) -> &str {
        self.name.get(index).unwrap()
    }
}

#[derive(Clone)]
pub struct CsvBody {
    rows: Vec<CsvRow>,
}
impl CsvBody {
    pub fn new() -> Self {
        Self {rows: Vec::new()}
    }

    pub fn append(&mut self, data: CsvRow) {
        self.rows.push(data);
    }

    pub fn get(&self, row: usize) -> CsvRow {
        self.rows.get(row).unwrap().clone()
    }
}

#[derive(Clone)]
pub struct CsvRow {
    datas: Vec<CsvData>,
}
impl CsvRow {
    pub fn new() -> Self {
        Self {datas: Vec::new()}
    }

    pub fn append(&mut self, data: CsvData) {
        self.datas.push(data);
    }

    pub fn get(self, header_name: &str) -> String {
        for data in self.datas {
            if data.header_name == header_name {
                return data.value;
            }
        }
        panic!("")
    }
}

#[derive(Clone)]
pub struct CsvData {
    header_name: String,
    value: String,
}
impl CsvData {
    pub fn new(header_name: &str, value: &str) -> Self {
        Self {
            header_name: header_name.to_string(),
            value: value.to_string(),
        }
    }
}
