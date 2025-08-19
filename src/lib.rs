use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

pub fn read(path: &str) -> Result<CsvFile, String> {
    // ファイルを比較
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(format!("openに失敗しました。{}", e)),
    };

    // 1行ごとにファイルを読み込み、CsvFileを作成する
    let buffer = BufReader::new(file);
    let mut csv_header = CsvHeader::new();
    let mut csv_body = CsvBody::new();
    for (index, line) in buffer.lines().enumerate() {
        // 最初の一行目はヘッダにする
        if index == 0 {
            for data in line.unwrap().split(",") {
                csv_header.append(data);
            }
            continue;
        }

        // 2行目以降はデータにする
        let mut csv_row = CsvRow::new();
        for (index, data) in line.unwrap().split(",").enumerate() {
            let csv_data = CsvData::new(
                csv_header.get_name(index)?,
                data
            );
            csv_row.append(csv_data);
        }
        csv_body.append(csv_row);
    }

    Ok(CsvFile::new(csv_header, csv_body))
}

pub fn write(path: &str, csv_file: &CsvFile) -> Result<(), String> {
    // 書き込みデータを生成する
    let mut data: Vec<u8> = Vec::new();
    // ヘッダーの処理
    for name in &csv_file.csv_header.name {
        data.extend(name.as_bytes());
        data.push(',' as u8);
    }
    // 末尾の,は削除する
    data.pop();
    // 改行コードを挿入する
    data.push('\n' as u8);

    // データの処理
    for row in &csv_file.csv_body.rows {
        for csv_data in &row.data {
            data.extend(csv_data.value.as_bytes());
            data.push(',' as u8);
        }
        // 末尾の,は削除する
        data.pop();
        // 改行コードを挿入する
        data.push('\n' as u8);
    }

    // 書き込み処理
    // 対象のファイルがあるかを検証する
    let exists = match fs::exists(path) {
        Ok(value) => value,
        Err(e) => return Err(format!("existsに失敗しました。[{}]", e)),
    };
    let file= if !exists {
        // ファイルが存在しなかった場合、ファイルを生成する
        match File::create(path) {
            Ok(value ) => value,
            Err(e) => return Err(format!("createに失敗しました。[{}]", e)),
        }
    } else {
        // ファイルが存在した場合、そのファイルを開く
        match OpenOptions::new().write(true).open(path) {
            Ok(value) => value,
            Err(e) => return Err(format!("openに失敗しました。[{}]", e)),
        }
    };

    let mut writer = BufWriter::new(file);

    if let Err(e) = writer.write(&data) {
        return Err(format!("writeに失敗しました。[{}]", e));
    }
    if let Err(e) = writer.flush() {
        return Err(format!("flushに失敗しました。[{}]", e));
    }

    Ok(())
}

pub struct CsvFile {
    csv_header: CsvHeader,
    csv_body: CsvBody,
}
impl CsvFile {
    fn new(csv_header: CsvHeader, csv_body: CsvBody) -> Self {
        Self {
            csv_header,
            csv_body,
        }
    }

    pub fn get_value(&self, header_name: &str, row_index: usize) -> Result<String, String> {
        match self.csv_body.get_row(row_index) {
            Ok(value) => {
                match value.get_value(header_name) {
                    Ok(value) => Ok(value),
                    Err(e) => Err(format!("値の取得に失敗しました。[{}]", e)),
                }
            },
            Err(e) => Err(format!("値の取得に失敗しました。[{}]", e)),
        }
    }

    pub fn get_header(&self) -> CsvHeader {
        self.csv_header.clone()
    }

    pub fn get_body(&self) -> CsvBody {
        self.csv_body.clone()
    }

    fn build_row(&self, data: Vec<String>) -> Result<CsvRow, String> {
        let mut row = CsvRow::new();
        for (index, header_name) in self.csv_header.name.iter().enumerate() {
            let data = CsvData::new(header_name, match data.get(index) {
                Some(value) => value.as_str(),
                None => return Err(String::from("追加データの取得に失敗しました。")),
            });
            row.append(data)
        }

        Ok(row)
    }

    pub fn append(&mut self, data: Vec<String>) -> Result<(), String> {
        if data.len() != self.csv_header.len() {
            return Err(format!("行のサイズが不正です。必要数=[{}], 渡した数=[{}]"
                        , self.csv_header.len(), data.len()));
        }

        match self.build_row(data) {
            Ok(row) => {
                self.csv_body.rows.push(row);
                Ok(())
            }
            Err(e) => Err(format!("行の追加に失敗しました。[{}]", e))
        }
    }

    pub fn insert(&mut self, index: usize, data: Vec<String>) -> Result<(), String> {
        if data.len() != self.csv_header.len() {
            return Err(format!("行のサイズが不正です。必要数=[{}], 渡した数=[{}]"
                               , self.csv_header.len(), data.len()));
        }
        if index >= self.csv_header.len() {
            return Err(format!("不正なインデックスです。指定したインデックス=[{}], 許容範囲=[{}]"
                               , index, self.csv_header.len()));
        }

        match self.build_row(data) {
            Ok(row) => {
                self.csv_body.rows.insert(index, row);
                Ok(())
            }
            Err(e) => Err(format!("行の追加に失敗しました。[{}]", e))
        }
    }

    pub fn update(&mut self, index: usize, data: Vec<String>) -> Result<(), String>{
        if data.len() != self.csv_header.len() {
            return Err(format!("行のサイズが不正です。必要数=[{}], 渡した数=[{}]"
                               , self.csv_header.len(), data.len()));
        }
        if index >= self.csv_header.len() {
            return Err(format!("不正なインデックスです。指定したインデックス=[{}], 許容範囲=[{}]"
                               , index, self.csv_header.len()));
        }

        match self.build_row(data) {
            Ok(row) => {
                self.csv_body.rows.remove(index);
                self.csv_body.rows.insert(index, row);
                Ok(())
            }
            Err(e) => Err(format!("行の追加に失敗しました。[{}]", e))
        }
    }

    pub fn remove(&mut self, index: usize) -> Result<(), String> {
        if index >= self.csv_header.len() {
            return Err(format!("不正なインデックスです。指定したインデックス=[{}], 許容範囲=[{}]"
                               , index, self.csv_header.len()));
        }

        self.csv_body.rows.remove(index);

        Ok(())
    }
}

#[derive(Clone)]
pub struct CsvHeader {
    name: Vec<String>,
}
impl CsvHeader {
    fn new() -> Self{
        Self {name: Vec::new()}
    }

    fn append(&mut self, data: &str) {
        self.name.push(data.to_string());
    }

    pub fn get_name(&self, index: usize) -> Result<&str, String> {
        if index >= self.name.len() {
            return Err(format!("範囲外のインデックスが指定されました。[{}]", index));
        }

        Ok(self.name.get(index).unwrap())
    }

    pub fn len(&self) -> usize {
        self.name.len()
    }
}

#[derive(Clone)]
pub struct CsvBody {
    rows: Vec<CsvRow>,
}
impl CsvBody {
    fn new() -> Self {
        Self {rows: Vec::new()}
    }

    fn append(&mut self, data: CsvRow) {
        self.rows.push(data);
    }

    pub fn get_row(&self, index: usize) -> Result<CsvRow, String> {
        if index >= self.rows.len() {
            return Err(format!("範囲外のインデックスが指定されました。[{}]", index));
        }

        match self.rows.get(index) {
            Some(value) => Ok(value.clone()),
            None => Err(String::from("CsvRowの取得に失敗しました。"))
        }
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

#[derive(Clone)]
pub struct CsvRow {
    data: Vec<CsvData>,
}
impl CsvRow {
    fn new() -> Self {
        Self {data: Vec::new()}
    }

    fn append(&mut self, csv_data: CsvData) {
        self.data.push(csv_data);
    }

    pub fn get_value(&self, header_name: &str) -> Result<String, String> {
        for csv_data in &self.data {
            if csv_data.header_name == header_name {
                return Ok(csv_data.value.to_string());
            }
        }
        Err(format!("存在しないヘッダー名です。[{}]", header_name))
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[derive(Clone)]
pub struct CsvData {
    header_name: String,
    value: String,
}
impl CsvData {
    fn new(header_name: &str, value: &str) -> Self {
        Self {
            header_name: header_name.to_string(),
            value: value.to_string(),
        }
    }
}





////////////////////////////////////////////////////////////////////////////////
//
//
//   Test
//
//
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_row_len() {
        assert_eq!(3, make_csv_row().len());
    }

    #[test]
    fn csv_row_get_value() {
        let row = make_csv_row();

        assert_eq!("海賊うさぎ", row.get_value("ヘッダー2").unwrap());
    }

    #[test]
    fn csv_row_get_value_error() {
        let row = make_csv_row();

        match row.get_value("ヘッダー4") {
            Ok(_) => panic!("エラーが発生しませんでした。"),
            Err(e) => assert_eq!("存在しないヘッダー名です。[ヘッダー4]", e),
        }
    }

    #[test]
    fn csv_body_len() {
        assert_eq!(3, make_body().len());
    }

    #[test]
    fn csv_body_get_row() {
        let body = make_body();
        let row = body.get_row(0).unwrap();

        assert_eq!("いるかねこ", row.get_value("ヘッダー1").unwrap());
        assert_eq!("海賊うさぎ", row.get_value("ヘッダー2").unwrap());
        assert_eq!("やかまし", row.get_value("ヘッダー3").unwrap());
    }

    #[test]
    fn csv_body_get_row_error() {
        let body = make_body();
        match body.get_row(5) {
            Ok(_) => panic!("エラーが発生しませんでした。"),
            Err(e) => assert_eq!("範囲外のインデックスが指定されました。[5]", e),
        }


    }

    #[test]
    fn csv_header_len() {
        assert_eq!(3, make_header().len());
    }

    #[test]
    fn csv_header_get_name() {
        let header = make_header();

        assert_eq!("ヘッダー2", header.get_name(1).unwrap());
    }

    #[test]
    fn csv_header_get_name_error() {
        let header = make_header();

        match header.get_name(5) {
            Ok(_) => panic!("エラーが発生しませんでした。"),
            Err(e) => assert_eq!("範囲外のインデックスが指定されました。[5]", e),
        }
    }

    #[test]
    fn csv_file_get_value() {
        let file = make_file();

        assert_eq!("船長メイド", file.get_value("ヘッダー3", 2).unwrap());
    }

    #[test]
    fn csv_file_get_value_error_header_name() {
        let file = make_file();

        match file.get_value("ヘッダー5", 2) {
            Ok(_) => panic!("エラー発生しませんでした。"),
            Err(e) =>
                assert_eq!("値の取得に失敗しました。[存在しないヘッダー名です。[ヘッダー5]]", e),
        }
    }

    #[test]
    fn csv_file_get_value_error_row_index() {
        let file = make_file();

        match file.get_value("ヘッダー2", 5) {
            Ok(_) => panic!("エラー発生しませんでした。"),
            Err(e) =>
                assert_eq!("値の取得に失敗しました。[範囲外のインデックスが指定されました。[5]]", e),
        }
    }

    #[test]
    fn csv_file_append_len() {
        let mut file = make_file();

        let data = vec![
            String::from("きつねおおかみ"),
            String::from("いぬてんし"),
            String::from("おけぶろ"),
        ];

        file.append(data).unwrap();

        assert_eq!(4, file.csv_body.len());
    }

    #[test]
    fn csv_file_append() {
        let mut file = make_file();

        let data = vec![
            String::from("きつねおおかみ"),
            String::from("いぬてんし"),
            String::from("おけぶろ"),
        ];

        file.append(data).unwrap();

        let expect = ["いるかねこ", "いぬねこ", "塩鯱", "きつねおおかみ"];

        for (index, row) in file.csv_body.rows.iter().enumerate() {
            assert_eq!(expect[index], row.get_value("ヘッダー1").unwrap());
        }
    }

    #[test]
    fn csv_file_append_error() {
        let mut file = make_file();

        let data = vec![
            String::from("きつねおおかみ"),
            String::from("いぬてんし"),
        ];

        match file.append(data) {
            Ok(_) => panic!("エラーが発生しませんでした。"),
            Err(e) => assert_eq!(e
                                 , "行のサイズが不正です。必要数=[3], 渡した数=[2]"),
        }
    }

    #[test]
    fn csv_file_insert_len() {
        let mut file = make_file();

        let data = vec![
            String::from("きつねおおかみ"),
            String::from("いぬてんし"),
            String::from("おけぶろ"),
        ];

        file.insert(2, data).unwrap();

        assert_eq!(4, file.csv_body.len());
    }

    #[test]
    fn csv_file_insert() {
        let mut file = make_file();

        let data = vec![
            String::from("きつねおおかみ"),
            String::from("いぬてんし"),
            String::from("おけぶろ"),
        ];

        file.insert(2, data).unwrap();

        let expect = ["いるかねこ", "いぬねこ", "きつねおおかみ", "塩鯱"];

        for (index, row) in file.csv_body.rows.iter().enumerate() {
            assert_eq!(expect[index], row.get_value("ヘッダー1").unwrap());
        }
    }

    #[test]
    fn csv_file_insert_error() {
        let mut file = make_file();

        let data = vec![
            String::from("きつねおおかみ"),
            String::from("いぬてんし"),
        ];

        match file.insert(1, data) {
            Ok(_) => panic!("エラーが発生しませんでした。"),
            Err(e) => assert_eq!(e
                                 , "行のサイズが不正です。必要数=[3], 渡した数=[2]"),
        }
    }

    #[test]
    fn csv_file_insert_error2() {
        let mut file = make_file();

        let data = vec![
            String::from("きつねおおかみ"),
            String::from("いぬてんし"),
            String::from("おけぶろ"),
        ];

        match file.insert(5, data) {
            Ok(_) => panic!("エラーが発生しませんでした。"),
            Err(e) => assert_eq!(e
                                 , "不正なインデックスです。指定したインデックス=[5], 許容範囲=[3]"),
        }
    }

    #[test]
    fn csv_file_update_len() {
        let mut file = make_file();

        let data = vec![
            String::from("きつねおおかみ"),
            String::from("いぬてんし"),
            String::from("おけぶろ"),
        ];

        file.update(1, data).unwrap();

        assert_eq!(3, file.csv_body.len());
    }

    #[test]
    fn csv_file_update() {
        let mut file = make_file();

        let data = vec![
            String::from("きつねおおかみ"),
            String::from("いぬてんし"),
            String::from("おけぶろ"),
        ];

        file.update(1, data).unwrap();

        let expect = ["いるかねこ", "きつねおおかみ", "塩鯱"];

        for (index, row) in file.csv_body.rows.iter().enumerate() {
            assert_eq!(expect[index], row.get_value("ヘッダー1").unwrap());
        }
    }

    #[test]
    fn csv_file_update_error() {
        let mut file = make_file();

        let data = vec![
            String::from("きつねおおかみ"),
            String::from("いぬてんし"),
        ];

        match file.update(1, data) {
            Ok(_) => panic!("エラーが発生しませんでした。"),
            Err(e) => assert_eq!(e
                                 , "行のサイズが不正です。必要数=[3], 渡した数=[2]"),
        }
    }

    #[test]
    fn csv_file_update_error2() {
        let mut file = make_file();

        let data = vec![
            String::from("きつねおおかみ"),
            String::from("いぬてんし"),
            String::from("おけぶろ"),
        ];

        match file.update(5, data) {
            Ok(_) => panic!("エラーが発生しませんでした。"),
            Err(e) => assert_eq!(e
                                 , "不正なインデックスです。指定したインデックス=[5], 許容範囲=[3]"),
        }
    }

    #[test]
    fn csv_file_remove_len() {
        let mut file = make_file();

        file.remove(2).unwrap();

        assert_eq!(2, file.csv_body.len());
    }

    #[test]
    fn csv_file_remove() {
        let mut file = make_file();

        file.remove(2).unwrap();

        let expect = ["いるかねこ", "いぬねこ"];

        for (index, row) in file.csv_body.rows.iter().enumerate() {
            assert_eq!(expect[index], row.get_value("ヘッダー1").unwrap());
        }
    }

    #[test]
    fn csv_file_remove_error() {
        let mut file = make_file();

        match file.remove(5) {
            Ok(_) => panic!("エラーが発生しませんでした。"),
            Err(e) => assert_eq!(e
                                 , "不正なインデックスです。指定したインデックス=[5], 許容範囲=[3]"),
        }
    }

    #[test]
    fn read_csv_header() {
        let csv = read("test/test.csv").unwrap();

        let header = csv.get_header();
        let expect = ["ヘッダー1", "ヘッダー2", "ヘッダー3"];
        for (index, value) in header.name.iter().enumerate() {
            assert_eq!(expect[index], value);
        }
    }

    #[test]
    fn read_csv_body() {
        let expect = [
            ["いるかねこ", "船長うさぎ", "やかまし",],
            ["しおしゃち", "いぬねこ", "船長メイド",],
            ["いぬてんし", "おけぶろ", "すもっく",],
        ];

        let csv = read("test/test.csv").unwrap();

        let header = csv.get_header();
        let body = csv.get_body();

        for (index, row) in body.rows.iter().enumerate() {
            for (index2, name) in header.name.iter().enumerate() {
                let value = row.get_value(name).unwrap();
                assert_eq!(expect[index][index2], value);
            }
        }
    }

    #[test]
    fn read_csv_write_new_file() {
        let mut csv = read("test/test2.csv").unwrap();

        let data = vec![
            String::from("いぬ海賊"),
            String::from("海賊プリンセス"),
            String::from("まがまが"),
        ];

        csv.update(1, data).unwrap();

        write("test/test2-2.csv", &csv).unwrap();

        let expect = [
            ["ヘッダー1", "ヘッダー2", "ヘッダー3",],
            ["いるかねこ", "船長うさぎ", "やかまし",],
            ["いぬ海賊", "海賊プリンセス", "まがまが",],
            ["いぬてんし", "おけぶろ", "すもっく"],
        ];
        let csv = read("test/test2-2.csv").unwrap();

        assert_write_file(expect, csv);
    }

    #[test]
    fn read_csv_write_over_write_file() {
        let mut csv = read("test/test3.csv").unwrap();

        let data = vec![
            String::from("いぬ海賊"),
            String::from("海賊プリンセス"),
            String::from("まがまが"),
        ];

        csv.update(1, data).unwrap();


        write("test/test3.csv", &csv).unwrap();

        let expect = [
            ["ヘッダー1", "ヘッダー2", "ヘッダー3",],
            ["いるかねこ", "船長うさぎ", "やかまし",],
            ["いぬ海賊", "海賊プリンセス", "まがまが",],
            ["いぬてんし", "おけぶろ", "すもっく"],
        ];
        let csv = read("test/test3.csv").unwrap();

        assert_write_file(expect, csv);
    }

    fn assert_write_file(expect: [[&str;3];4], csv: CsvFile) {
        // ヘッダー部のテスト
        for (index, name) in csv.get_header().name.iter().enumerate() {
            assert_eq!(expect[0][index], name);
        }

        // データ部のテスト
        for (index1, name) in csv.get_header().name.iter().enumerate() {
            for index2 in 0..csv.get_body().len() {
                let value = csv.get_value(name.as_str(), index2).unwrap();

                assert_eq!(expect[index2 + 1][index1], value);
            }
        }
    }

    fn make_csv_row() -> CsvRow {
        let mut row = CsvRow::new();

        let data = CsvData::new("ヘッダー1", "いるかねこ");
        row.append(data);

        let data = CsvData::new("ヘッダー2", "海賊うさぎ");
        row.append(data);

        let data = CsvData::new("ヘッダー3", "やかまし");
        row.append(data);

        row
    }

    fn make_csv_row2() -> CsvRow {
        let mut row = CsvRow::new();

        let data = CsvData::new("ヘッダー1", "いぬねこ");
        row.append(data);

        let data = CsvData::new("ヘッダー2", "いぬ肝臓");
        row.append(data);

        let data = CsvData::new("ヘッダー3", "海賊姫");
        row.append(data);

        row
    }

    fn make_csv_row3() -> CsvRow {
        let mut row = CsvRow::new();

        let data = CsvData::new("ヘッダー1", "塩鯱");
        row.append(data);

        let data = CsvData::new("ヘッダー2", "塩うさぎ");
        row.append(data);

        let data = CsvData::new("ヘッダー3", "船長メイド");
        row.append(data);

        row
    }

    fn make_body() -> CsvBody {
        let mut body = CsvBody::new();
        body.append(make_csv_row());
        body.append(make_csv_row2());
        body.append(make_csv_row3());

        body
    }

    fn make_header() -> CsvHeader {
        let mut header = CsvHeader::new();
        header.append("ヘッダー1");
        header.append("ヘッダー2");
        header.append("ヘッダー3");

        header
    }

    fn make_file() -> CsvFile {
        CsvFile::new(make_header(), make_body())
    }
}
