use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StaffRole {
    Unknown,
    Normal,
    Admin,
}

impl StaffRole {
    pub fn is_valid(&self) -> bool {
        !matches!(self, StaffRole::Unknown)
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, StaffRole::Admin)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            StaffRole::Unknown => "unknown",
            StaffRole::Normal => "normal",
            StaffRole::Admin => "admin",
        }
    }
}

impl FromStr for StaffRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(StaffRole::Normal),
            "admin" => Ok(StaffRole::Admin),
            _ => Ok(StaffRole::Unknown),
        }
    }
}

impl fmt::Display for StaffRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for StaffRole {
    fn default() -> Self {
        StaffRole::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!("normal".parse::<StaffRole>().unwrap(), StaffRole::Normal);
        assert_eq!("admin".parse::<StaffRole>().unwrap(), StaffRole::Admin);
        assert_eq!("invalid".parse::<StaffRole>().unwrap(), StaffRole::Unknown);
    }

    #[test]
    fn test_is_valid() {
        assert!(!StaffRole::Unknown.is_valid());
        assert!(StaffRole::Normal.is_valid());
        assert!(StaffRole::Admin.is_valid());
    }

    #[test]
    fn test_is_admin() {
        assert!(!StaffRole::Unknown.is_admin());
        assert!(!StaffRole::Normal.is_admin());
        assert!(StaffRole::Admin.is_admin());
    }

    #[test]
    fn test_display() {
        assert_eq!(StaffRole::Normal.to_string(), "normal");
        assert_eq!(StaffRole::Admin.to_string(), "admin");
    }
}
