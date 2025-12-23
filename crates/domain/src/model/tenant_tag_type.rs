use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TenantTagType {
    #[default]
    Unknown,
    Entertainment,
    Education,
    Business,
    Other,
}

impl TenantTagType {
    pub fn is_valid(&self) -> bool {
        !matches!(self, TenantTagType::Unknown)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TenantTagType::Unknown => "unknown",
            TenantTagType::Entertainment => "entertainment",
            TenantTagType::Education => "education",
            TenantTagType::Business => "business",
            TenantTagType::Other => "other",
        }
    }
}

impl FromStr for TenantTagType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "entertainment" => Ok(TenantTagType::Entertainment),
            "education" => Ok(TenantTagType::Education),
            "business" => Ok(TenantTagType::Business),
            "other" => Ok(TenantTagType::Other),
            _ => Ok(TenantTagType::Unknown),
        }
    }
}

impl fmt::Display for TenantTagType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            "entertainment".parse::<TenantTagType>().unwrap(),
            TenantTagType::Entertainment
        );
        assert_eq!(
            "education".parse::<TenantTagType>().unwrap(),
            TenantTagType::Education
        );
        assert_eq!(
            "business".parse::<TenantTagType>().unwrap(),
            TenantTagType::Business
        );
        assert_eq!(
            "invalid".parse::<TenantTagType>().unwrap(),
            TenantTagType::Unknown
        );
    }

    #[test]
    fn test_is_valid() {
        assert!(!TenantTagType::Unknown.is_valid());
        assert!(TenantTagType::Entertainment.is_valid());
        assert!(TenantTagType::Education.is_valid());
    }
}
