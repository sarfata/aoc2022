#[derive(Debug, PartialEq, Eq)]
pub enum ListingOutput {
    File { name: String, size: usize },
    Directory { name: String },
}

#[derive(Debug, PartialEq, Eq)]
pub enum InputLine {
    CdRoot,
    CdFolder { s: String },
    LsCmd,
    ListingOutput(ListingOutput),
}

// IMPROVEMENTS:
// Avoid copying the input data into strings... - keep using reference and use lifetimes.

impl TryFrom<&str> for InputLine {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let fields: Vec<&str> = value.split_whitespace().collect();
        let parts = (fields.get(0), fields.get(1), fields.get(2));

        match parts {
            (Some(&"$"), Some(&"ls"), _) => Ok(InputLine::LsCmd),
            (Some(&"$"), Some(&"cd"), Some(&"/")) => Ok(InputLine::CdRoot),
            (Some(&"$"), Some(&"cd"), Some(path)) => Ok(InputLine::CdFolder {
                s: path.to_string(),
            }),
            (Some(&"dir"), Some(name), _) => {
                Ok(InputLine::ListingOutput(ListingOutput::Directory {
                    name: name.to_string(),
                }))
            }
            (Some(size), Some(name), _) => Ok(InputLine::ListingOutput(ListingOutput::File {
                name: name.to_string(),
                size: size.parse().map_err(|_| "Unable to parse size")?,
            })),

            _ => Err("Invalid input line."),
        }
    }
}

#[test]
fn parse_ls() {
    match "$ ls".try_into() as Result<InputLine, _> {
        Ok(x) => assert!(x == InputLine::LsCmd),
        Err(e) => assert!(false, "{e:?}"),
    }
}
#[test]
fn convert_listing() {
    let lines: Vec<InputLine> = vec!["$ cd /", "$ ls", "dir a", "111 b.txt"]
        .iter()
        .cloned()
        .filter_map(|l| l.try_into().ok())
        .collect();

    println!("{lines:?}");

    assert!(lines.len() == 4)
}
