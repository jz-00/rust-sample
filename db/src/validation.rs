use std::result::Result;

use super::error::DbError;

pub fn validate_str<T: AsRef<str>>(name: &'static str, value: T) -> Result<String, DbError> {
    let value = value.as_ref().trim().to_owned();
    if !value.is_empty() {
        Ok(value)
    } else {
        Err(DbError::InvalidValue { name, value })
    }
}

pub fn validate_required_str<T: AsRef<str>>(
    name: &'static str,
    value: Option<T>,
) -> Result<String, DbError> {
    if let Some(value) = value {
        validate_str(name, value)
    } else {
        Err(DbError::RequiredArgument(name))
    }
}

pub fn validate_optional_str<T: AsRef<str>>(
    name: &'static str,
    value: Option<T>,
) -> Result<Option<String>, DbError> {
    if let Some(value) = value {
        Ok(Some(validate_str(name, value)?))
    } else {
        Ok(None)
    }
}

pub fn validate_str_vec<T: AsRef<str>, V: AsRef<[T]>>(
    name: &'static str,
    value: V,
) -> Result<Box<[String]>, DbError> {
    let value = value.as_ref();
    let mut validated = Vec::with_capacity(value.len());
    for v in value {
        validated.push(validate_str(name, v)?);
    }
    Ok(validated.into_boxed_slice())
}

pub fn validate_required_str_vec<T: AsRef<str>, V: AsRef<[T]>>(
    name: &'static str,
    value: Option<V>,
) -> Result<Box<[String]>, DbError> {
    if let Some(value) = value {
        validate_str_vec(name, value)
    } else {
        Err(DbError::RequiredArgument(name))
    }
}

pub fn validate_optional_str_vec<T: AsRef<str>, V: AsRef<[T]>>(
    name: &'static str,
    value: Option<V>,
) -> Result<Option<Box<[String]>>, DbError> {
    if let Some(value) = value {
        Ok(Some(validate_str_vec(name, value.as_ref())?))
    } else {
        Ok(None)
    }
}

pub fn validate_required<T>(name: &'static str, value: Option<T>) -> Result<T, DbError> {
    value.ok_or(DbError::RequiredArgument(name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_str() -> Result<(), DbError> {
        assert_eq!(validate_str("test", "pass")?, "pass");
        assert_eq!(validate_str("test", " pass ")?, "pass");
        assert!(validate_str("test", "").is_err());
        Ok(())
    }

    #[test]
    fn test_validate_required_str() -> Result<(), DbError> {
        assert_eq!(validate_required_str("test", Some("pass"))?, "pass");
        assert_eq!(validate_required_str("test", Some(" pass "))?, "pass");
        assert!(validate_required_str("test", None::<&str>).is_err());
        Ok(())
    }

    #[test]
    fn test_validate_optional_str() -> Result<(), DbError> {
        assert_eq!(
            validate_optional_str("test", Some("pass"))?,
            Some("pass".to_owned())
        );
        assert_eq!(
            validate_optional_str("test", Some(" pass "))?,
            Some("pass".to_owned())
        );
        assert!(validate_optional_str("test", None::<&str>).is_ok());
        Ok(())
    }

    #[test]
    fn test_validate_str_vec() -> Result<(), DbError> {
        assert_eq!(*validate_str_vec("test", ["pass"])?, ["pass".to_owned()]);
        assert_eq!(*validate_str_vec("test", [" pass "])?, ["pass".to_owned()]);
        assert_eq!(
            *validate_str_vec("test", Vec::<String>::new())?,
            Vec::<String>::new()[..]
        );
        assert!(validate_str_vec("test", [""]).is_err());
        Ok(())
    }

    #[test]
    fn test_validate_required_str_vec() -> Result<(), DbError> {
        assert_eq!(
            *validate_required_str_vec("test", Some(["pass"]))?,
            ["pass".to_owned()]
        );
        assert_eq!(
            *validate_required_str_vec("test", Some([" pass "]))?,
            ["pass".to_owned()]
        );
        assert_eq!(
            *validate_required_str_vec("test", Some(Vec::<String>::new()))?,
            Vec::<String>::new()[..]
        );
        assert!(validate_required_str_vec("test", None::<Vec<&str>>).is_err());
        assert!(validate_required_str_vec("test", Some([""])).is_err());
        Ok(())
    }

    #[test]
    fn test_validate_optional_str_vec() -> Result<(), DbError> {
        assert_eq!(
            validate_optional_str_vec("test", Some(["pass"]))?,
            Some(vec!["pass".to_owned()].into_boxed_slice())
        );
        assert_eq!(
            validate_optional_str_vec("test", Some([" pass "]))?,
            Some(vec!["pass".to_owned()].into_boxed_slice())
        );
        assert_eq!(
            validate_optional_str_vec("test", Some(Vec::<String>::new()))?,
            Some(Vec::<String>::new().into_boxed_slice())
        );
        assert!(validate_optional_str_vec("test", None::<Vec<&str>>).is_ok());
        assert!(validate_optional_str_vec("test", Some([""])).is_err());
        Ok(())
    }

    #[test]
    fn test_validate_required() -> Result<(), DbError> {
        assert!(validate_required("test", Some(())).is_ok());
        assert!(validate_required::<()>("test", None).is_err());
        Ok(())
    }
}
