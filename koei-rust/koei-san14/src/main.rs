use koei_san14::fixdata_exce::{FixDataExce, confirmed_prefix_sections, confirmed_tail_sections};
use std::{env, error::Error, fmt, fs, path::PathBuf};

const DEFAULT_OFFSET: usize = 0;
const DEFAULT_LENGTH: usize = 0x33;

#[derive(Debug)]
struct SliceError(String);

impl fmt::Display for SliceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for SliceError {}

struct SliceRequest {
    path: PathBuf,
    offset: usize,
    length: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    match arguments.first().map(String::as_str) {
        Some("slice") => run_slice(arguments),
        Some("fixdata-exce") => run_fixdata_exce(arguments),
        _ => Err(SliceError(usage()).into()),
    }
}

fn run_slice(arguments: Vec<String>) -> Result<(), Box<dyn Error>> {
    let request = parse_request(arguments)?;
    let bytes = fs::read(&request.path)?;
    let slice = read_slice(&bytes, request.offset, request.length)?;

    println!(
        "{}: offset 0x{:X}, length 0x{:X} ({} bytes)",
        request.path.display(),
        request.offset,
        slice.len(),
        slice.len()
    );
    print_hexdump(slice, request.offset);

    Ok(())
}

fn run_fixdata_exce(arguments: Vec<String>) -> Result<(), Box<dyn Error>> {
    if arguments.get(1).map(String::as_str) != Some("inspect") || arguments.len() != 3 {
        return Err(SliceError(usage()).into());
    }

    let path = PathBuf::from(&arguments[2]);
    let bytes = fs::read(&path)?;
    let document = FixDataExce::parse(&bytes)?;

    println!(
        "{}: header 0x{:X}, payload 0x{:X} bytes",
        path.display(),
        document.header_value,
        document.payload.len()
    );
    println!("confirmed Van Editor encoder prefix:");
    print_section_plan(confirmed_prefix_sections());
    println!("confirmed Van Editor encoder tail:");
    print_section_plan(confirmed_tail_sections());
    println!(
        "section offsets are intentionally not inferred until the on-disk reader is recovered."
    );

    Ok(())
}

fn print_section_plan(sections: &[koei_san14::fixdata_exce::EncoderSection]) {
    for section in sections {
        let identifier = section
            .identifier
            .map(|value| value.to_string())
            .unwrap_or_else(|| "leading".to_owned());
        let record_size = section
            .memory_record_size
            .map(|size| format!("0x{size:X}"))
            .unwrap_or_else(|| "mode-dependent".to_owned());

        println!(
            "  id={identifier}, count=0x{:X} ({}), memory_record_size={record_size}",
            section.element_count, section.element_count
        );
    }
}

fn parse_request(arguments: impl IntoIterator<Item = String>) -> Result<SliceRequest, SliceError> {
    let arguments = arguments.into_iter().collect::<Vec<_>>();
    if arguments.first().map(String::as_str) != Some("slice") {
        return Err(SliceError(usage()));
    }

    let Some(path) = arguments.get(1) else {
        return Err(SliceError(usage()));
    };

    if arguments.len() > 4 {
        return Err(SliceError(usage()));
    }

    let offset = arguments
        .get(2)
        .map(|value| parse_integer("offset", value))
        .transpose()?
        .unwrap_or(DEFAULT_OFFSET);
    let length = arguments
        .get(3)
        .map(|value| parse_integer("length", value))
        .transpose()?
        .unwrap_or(DEFAULT_LENGTH);

    Ok(SliceRequest {
        path: PathBuf::from(path),
        offset,
        length,
    })
}

fn parse_integer(name: &str, value: &str) -> Result<usize, SliceError> {
    let (radix, digits) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .map_or((10, value), |digits| (16, digits));

    usize::from_str_radix(digits, radix).map_err(|_| {
        SliceError(format!(
            "{name} must be a decimal or hexadecimal integer: {value}"
        ))
    })
}

fn read_slice(bytes: &[u8], offset: usize, length: usize) -> Result<&[u8], SliceError> {
    let end = offset
        .checked_add(length)
        .ok_or_else(|| SliceError("offset plus length overflows".to_owned()))?;

    bytes.get(offset..end).ok_or_else(|| {
        SliceError(format!(
            "requested range 0x{offset:X}..0x{end:X} exceeds file length 0x{:X}",
            bytes.len()
        ))
    })
}

fn print_hexdump(bytes: &[u8], base_offset: usize) {
    for (line_index, line) in bytes.chunks(16).enumerate() {
        print!("{:08X}: ", base_offset + line_index * 16);
        for byte in line {
            print!("{byte:02X} ");
        }
        println!();
    }
}

fn usage() -> String {
    "Usage:\n\
     koei-san14 slice <path> [offset] [length]\n\
     koei-san14 fixdata-exce inspect <path>\n\
     \n\
     Slice offsets and lengths accept decimal or 0x-prefixed hexadecimal values.\n\
     Slice defaults: offset 0x0, length 0x33."
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn parses_default_slice_request() {
        let request = parse_request(["slice", "fixdata.s14"].map(str::to_owned)).unwrap();

        assert_eq!(request.path, Path::new("fixdata.s14"));
        assert_eq!(request.offset, 0);
        assert_eq!(request.length, 0x33);
    }

    #[test]
    fn parses_hexadecimal_slice_request() {
        let request =
            parse_request(["slice", "fixdata.s14", "0x14", "0x33"].map(str::to_owned)).unwrap();

        assert_eq!(request.offset, 0x14);
        assert_eq!(request.length, 0x33);
    }

    #[test]
    fn rejects_out_of_bounds_slices() {
        let error = read_slice(&[0; 4], 3, 2).unwrap_err();

        assert_eq!(
            error.to_string(),
            "requested range 0x3..0x5 exceeds file length 0x4"
        );
    }
}
