use csv::Writer;
use serde::Serialize;

pub const PATH_DATA: &str = "../../.data";

pub fn csv<D: Serialize>(records: &[D]) -> anyhow::Result<String> {
    let mut writer = Writer::from_writer(Vec::new());
    records.iter().try_for_each(|record| writer.serialize(record))?;
    writer.flush()?;
    Ok(String::from_utf8(writer.into_inner()?)?)
}

#[macro_export]
macro_rules! file {
    ($name:expr, $extension:expr) => {
        format!("{}/{}.{}", $crate::commons::PATH_DATA, $name, $extension)
    };
}

#[macro_export]
macro_rules! data {
    ($name:expr, $extension:expr) => {
        std::fs::read(file!($name, $extension))
    };
}

#[macro_export]
macro_rules! snapshot {
    ($name:expr, $value:expr) => {
        insta::assert_snapshot!($name, csv($value).expect(format!("write {}.csv", $name).as_str()));
    };
}
