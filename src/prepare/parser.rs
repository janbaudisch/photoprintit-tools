use super::super::Platform;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct File {
    pub url: String,
    pub platform: Platform
}

impl fmt::Display for File {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.url)
    }
}

named!(url<&str, String>,
    map_res!(take_until!(";"), String::from_str)
);

named!(platform<&str, Platform>,
    map!(take_until!(";"), Platform::from)
);

named!(pub file<&str, File>,
    do_parse!(
        url:      url              >>
                  take!(1)         >>
        _x:       take_until!(";") >>
                  take!(1)         >>
        _kind:    take_until!(";") >>
                  take!(1)         >>
        platform: platform         >>
        (File { url, platform })
    )
);
