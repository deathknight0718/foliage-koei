#[macro_export]
macro_rules! path {
    ($name:expr, $extension:expr) => {
        format!("../../.data/{}{}", $name, $extension)
    };
}

#[macro_export]
macro_rules! bytes {
    ($name:expr, $extension:expr) => {
        std::fs::read(path!($name, $extension))
    };
}

pub fn snapshot<T: serde::Serialize>(records: &[T]) -> anyhow::Result<String> {
    let mut writer = csv::Writer::from_writer(Vec::new());
    records.iter().try_for_each(|record| writer.serialize(record))?;
    writer.flush()?;
    Ok(String::from_utf8(writer.into_inner()?)?)
}
