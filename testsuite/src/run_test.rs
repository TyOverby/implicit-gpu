use implicit::telemetry;
use super::Paths;
use std::path::PathBuf;
use super::formats;
use walkdir::{WalkDir};
use {flame, implicit, latin};

pub enum Error {
    NoExpectedFiles,
    CouldNotFind { file: String },
    UnexpectedFile { file: String },
    AabbMismatch {
        expected: String,
        actual: String
    },
    SvgMismatch {
        expected: String,
        actual: String
    },
    LineMismatch {
        expected: String,
        actual: String,
        message: String,
    },
    CMismatch {
        expected: String,
        actual: String,
    },
    NodesMismatch {
        expected: String,
        actual: String,
    },
    FieldMismatch {
        expected: String,
        actual: String,
        message: String,
    },
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            Error::NoExpectedFiles => writeln!(formatter, "  • No Expected files found")?,
            Error::CouldNotFind { ref file } => writeln!(formatter, "  • Could not find file {}", file)?,
            Error::SvgMismatch { ref expected, ref actual } => {
                writeln!(formatter, "  • svg files are not the same")?;
                writeln!(formatter, "    expected file : {}", expected)?;
                writeln!(formatter, "    actual file   : {}", actual)?;
            }
            Error::AabbMismatch { ref expected, ref actual } => {
                writeln!(formatter, "  • bounding box files are not the same")?;
                writeln!(formatter, "    expected file : {}", expected)?;
                writeln!(formatter, "    actual file   : {}", actual)?;
            }
            Error::CMismatch { ref expected, ref actual } => {
                writeln!(formatter, "  • c files are not the same")?;
                writeln!(formatter, "    expected file : {}", expected)?;
                writeln!(formatter, "    actual file   : {}", actual)?;
            }
            Error::NodesMismatch { ref expected, ref actual } => {
                writeln!(formatter, "  • nodes files are not the same")?;
                writeln!(formatter, "    expected file : {}", expected)?;
                writeln!(formatter, "    actual file   : {}", actual)?;
            }
            Error::UnexpectedFile { ref file} => writeln!(formatter, "  • unexpected file {}", file)?,
            Error::LineMismatch {
                ref expected,
                ref actual,
                ref message,
            } => {
                writeln!(formatter, "  • line files are not the same ({})", message)?;
                writeln!(formatter, "    expected file : {}", expected)?;
                writeln!(formatter, "    actual file   : {}", actual)?;
            }
            Error::FieldMismatch {
                ref expected,
                ref actual,
                ref message,
            } => {
                writeln!(formatter, "  • field files are not the same ({})", message)?;
                writeln!(formatter, "    expected file : {}", expected)?;
                writeln!(formatter, "    actual file   : {}", actual)?;
            }
        }

        Ok(())
    }
}

pub fn run_test(paths: &Paths) -> Result<(), Vec<Error>> {
    let _guard = flame::start_guard(format!("running {:?}", paths.json));

    let source = latin::file::read_string_utf8(&paths.json).unwrap();
    let scene = ::serde_json::from_str(&source).unwrap();

    let mut errors = vec![];

    let mut telemetry =
        telemetry::DumpTelemetry::new(paths.actual_dump.clone())
        .with_field_writer(|path, buffer| {
            latin::file::write(&path, formats::field::field_to_text(buffer)).unwrap();
        })
        .with_line_writer(|path, lines| {
            let lines = lines.iter().map(|&((x1, y1), (x2, y2))| formats::lines::Line(x1, y1, x2, y2));
            latin::file::write(&path, formats::lines::lines_to_text(lines)).unwrap();
        });

    ::std::fs::create_dir_all(&paths.actual_dump).unwrap();
    implicit::run_scene(&scene, &mut telemetry);

    if !paths.expected_dump.exists() {
        return Err(vec![Error::NoExpectedFiles])
    }
    let expected_paths =
        WalkDir::new(&paths.expected_dump)
            .into_iter()
            .filter_map(Result::ok)
            .map(|e| e.path().to_owned());

    for expected in expected_paths {
        let unique = expected.strip_prefix(&paths.parent).unwrap().to_owned();
        let unique: PathBuf = unique.components().skip(1).map(|c|c.as_os_str()).collect();
        let actual = paths.parent.join("actual").join(unique);

        // Ignore all files that start with "." (LOOKING AT YOU .DS_Store)
        if expected.file_name()
                   .and_then(|o| o.to_str())
                   .map(|p| p.starts_with("."))
                   .unwrap_or(false) {
            continue;
        }

        if !actual.exists() {
            errors.push(Error::CouldNotFind { file: actual.to_string_lossy().into_owned() });
            continue;
        }

        let extension = match expected.extension().and_then(|o| o.to_str()) {
            Some(ex) => ex,
            None => continue,
        };
        match extension {
            "png" => { /* png is for debugging only */ }
            "perf" => { /* perf data is for debugging only */ }
            "values" => {
                if let Err(e) = formats::field::compare(&expected, &actual) {
                    errors.push(Error::FieldMismatch {
                        expected: expected.to_string_lossy().into_owned(),
                        actual: actual.to_string_lossy().into_owned(),
                        message: e,
                    });
                }
            },
            "lines" => {
                if let Err(e) = formats::lines::compare(&expected, &actual) {
                    errors.push(Error::LineMismatch {
                        expected: expected.to_string_lossy().into_owned(),
                        actual: actual.to_string_lossy().into_owned(),
                        message: e,
                    });
                }
            }
            "c" => {
                if latin::file::read_string_utf8(&actual).unwrap() !=
                   latin::file::read_string_utf8(&expected).unwrap() {
                    errors.push(Error::CMismatch {
                        expected: expected.to_string_lossy().into_owned(),
                        actual: actual.to_string_lossy().into_owned(),
                    })
                }
            }
            "txt" => {
                if latin::file::read_string_utf8(&actual).unwrap() !=
                   latin::file::read_string_utf8(&expected).unwrap() {
                    errors.push(Error::NodesMismatch {
                        expected: expected.to_string_lossy().into_owned(),
                        actual: actual.to_string_lossy().into_owned(),
                    })
                }
            }
            "svg" => {
                if latin::file::read_string_utf8(&actual).unwrap() !=
                   latin::file::read_string_utf8(&expected).unwrap() {
                    errors.push(Error::SvgMismatch {
                        expected: expected.to_string_lossy().into_owned(),
                        actual: actual.to_string_lossy().into_owned(),
                    })
                }
            }
            "aabb" => {
                if latin::file::read_string_utf8(&actual).unwrap() !=
                   latin::file::read_string_utf8(&expected).unwrap() {
                    errors.push(Error::AabbMismatch {
                        expected: expected.to_string_lossy().into_owned(),
                        actual: actual.to_string_lossy().into_owned(),
                    })
                }
            }
            _ => errors.push(Error::UnexpectedFile{file: actual.to_str().unwrap().to_owned()}),
        }
    }

    if errors.len() == 0 {
        Ok(())
    } else {
        Err(errors)
    }
}
