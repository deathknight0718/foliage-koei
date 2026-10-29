pub const PATH_DATA: &str = "../../.data";

#[macro_export]
macro_rules! path {
    ($name:expr, $extension:expr) => {
        format!("{}/{}{}", $crate::commons::PATH_DATA, $name, $extension)
    };
}

#[macro_export]
macro_rules! bytes {
    ($name:expr, $extension:expr) => {
        std::fs::read(path!($name, $extension))
    };
}

#[macro_export]
macro_rules! snapshot {
    ($name:expr, $value:expr) => {{
        fn csv<D: serde::Serialize>(records: &[D]) -> anyhow::Result<String> {
            let mut writer = csv::Writer::from_writer(Vec::new());
            records.iter().try_for_each(|record| writer.serialize(record))?;
            writer.flush()?;
            Ok(String::from_utf8(writer.into_inner()?)?)
        }

        insta::assert_snapshot!($name, csv($value).expect(format!("write {}.csv", $name).as_str()));
    }};
}
