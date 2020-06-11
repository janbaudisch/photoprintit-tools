#[derive(Debug, PartialEq)]
pub enum Platform {
    Any,
    Linux,
    Linux64,
    Mac,
    Windows,
    Windows64,
}

impl<'a> From<&'a str> for Platform {
    fn from(input: &str) -> Self {
        debug!("platform key: {}", input);
        match input {
            "a" => Platform::Any,
            "l" => Platform::Linux,
            "l64" => Platform::Linux64,
            "m" => Platform::Mac,
            "w" => Platform::Windows,
            "w64" => Platform::Windows64,
            _ => panic!("Unknown platform: {}", input),
        }
    }
}
