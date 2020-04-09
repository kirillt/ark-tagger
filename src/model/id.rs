use std::str::FromStr;
use derive_more::From;
use std::string::ToString;
use std::option::NoneError;
use std::num::ParseIntError;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Id {
    pub size: u64,
    pub hash: u32
}

impl ToString for Id {
    fn to_string(&self) -> String {
        format!("s{}_h{}", self.size, self.hash)
    }
}

#[derive(Debug, From)]
pub enum ParseError {
    NotEnoughTokens(NoneError),
    UnexpectedToken(ParseIntError)
}

impl FromStr for Id {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('_');

        let size = parts.next()?;
        let size = size.trim_start_matches('s').parse()?;

        let hash = parts.next()?;
        let hash = hash.trim_start_matches('h').parse()?;

        Ok(Id { size, hash })
    }
}