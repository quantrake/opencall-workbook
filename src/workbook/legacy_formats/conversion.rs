use crate::workbook::app::Workbook;
use crate::workbook::legacy_formats::v_b0002::WorkVersionB0002;
use crate::workbook::legacy_formats::v_b0003::WorkVersionB0003;

#[derive(Debug)]
enum Format {
    Unknown,

    // Format version 'B0002' - see struct `WorkVersionB0002`
    VersionB0002,

    // Format version 'B0003' - see struct `WorkVersionB0003`
    VersionB0003,
}

impl Workbook {
    pub fn convert_or_err(&mut self, e: Box<bincode::ErrorKind>) {
        // Note: Assuming that the encoded format version is represented
        // as "0.1.3" (or "B0001"),
        // we read first 5 bytes of the file_buffer to get the version string.
        let version_bytes = 5 * 8;
        let read_version_bytes = self.file_buffer.get(..version_bytes).unwrap();

        let version_read_result: Result<String, bincode::Error> =
            bincode::deserialize(read_version_bytes);

        let known_format: Format;
        let file_format = match version_read_result {
            Ok(v) => {
                known_format = match v.as_str() {
                    "B0002" => Format::VersionB0002,
                    "B0003" => Format::VersionB0003,
                    _ => Format::Unknown,
                };
                v
            }
            Err(_err) => {
                known_format = Format::Unknown;
                "read error".to_string()
            }
        };

        match known_format {
            Format::VersionB0002 => {
                if let Ok(work) = WorkVersionB0002::decode_bincode(&self.file_buffer) {
                    self.apply_format_b0002(work);
                } else {
                    let err = format!(
                        "Detected version {}: \
                    incompatible data format, \
                    probably an old workbook version was used for saving this \
                    project: {}",
                        file_format, e
                    );
                    rfd::MessageDialog::new()
                        .set_title("Parser Error")
                        .set_description(err)
                        .set_level(rfd::MessageLevel::Error)
                        .show();
                }
            }

            Format::VersionB0003 => {
                if let Ok(work) = WorkVersionB0003::decode_bincode(&self.file_buffer) {
                    self.apply_format_b0003(work);
                } else {
                    let err = format!(
                        "Detected version {}: \
                    incompatible data format, \
                    probably an old workbook version was used for saving this \
                    project: {}",
                        file_format, e
                    );
                    rfd::MessageDialog::new()
                        .set_title("Parser Error")
                        .set_description(err)
                        .set_level(rfd::MessageLevel::Error)
                        .show();
                }
            }

            _ => {
                let err = format!(
                    "Detected version {}: \
                incompatible data format, \
                probably an old workbook version was used for saving this \
                project: {}",
                    file_format, e
                );
                rfd::MessageDialog::new()
                    .set_title("Parser Error")
                    .set_description(err)
                    .set_level(rfd::MessageLevel::Error)
                    .show();
            }
        }
    }
}
