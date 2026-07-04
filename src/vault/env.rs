use crate::constant::ENVS;
use crate::error::AppError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Env {
    Local,
    Dev,
    Prod,
}

impl Env {
    pub fn is_prod(self) -> bool {
        matches!(self, Env::Prod)
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Env::Local => "local",
            Env::Dev => "dev",
            Env::Prod => "prod",
        };
        write!(f, "{s}")
    }
}

impl FromStr for Env {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "local" => Ok(Env::Local),
            "dev" => Ok(Env::Dev),
            "prod" => Ok(Env::Prod),
            other => Err(AppError::InvalidEnv(format!("{other} (expected one of {})", ENVS.join(", ")))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyAddr {
    pub env: Env,
    pub name: String,
}

impl KeyAddr {
    pub fn parse(addr: &str) -> Result<Self, AppError> {
        let (env, name) = addr
            .split_once('/')
            .ok_or_else(|| AppError::Other(format!("expected env/name, got '{addr}'")))?;
        let env: Env = env.parse()?;
        validate_name(name)?;
        Ok(KeyAddr { env, name: name.to_string() })
    }

    pub fn stem(&self) -> String {
        format!("{}_{}", self.env, self.name)
    }
}

impl fmt::Display for KeyAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.env, self.name)
    }
}

pub fn validate_name(name: &str) -> Result<(), AppError> {
    let ok = !name.is_empty()
        && name.len() <= crate::constant::NAME_MAX_LEN
        && name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
    if ok {
        Ok(())
    } else {
        Err(AppError::InvalidName(name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_addr() {
        let addr = KeyAddr::parse("dev/deployer").unwrap();
        assert_eq!(addr.env, Env::Dev);
        assert_eq!(addr.name, "deployer");
        assert_eq!(addr.stem(), "dev_deployer");
    }

    #[test]
    fn rejects_bad_env() {
        assert!(KeyAddr::parse("staging/deployer").is_err());
    }

    #[test]
    fn rejects_bad_name() {
        assert!(KeyAddr::parse("dev/Deployer_1").is_err());
        assert!(KeyAddr::parse("dev/").is_err());
        assert!(KeyAddr::parse(&format!("dev/{}", "a".repeat(33))).is_err());
    }

    #[test]
    fn rejects_missing_slash() {
        assert!(KeyAddr::parse("deployer").is_err());
    }
}
