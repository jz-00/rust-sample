#[macro_use]
macro_rules! jsonb {
    ($vis:vis struct $name:ident $fields:tt) => {
        #[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone, PartialEq)]
        #[serde(default, rename_all = "camelCase")]
        $vis struct $name $fields

        impl crate::json::ToJson for $name {}

        impl $name {
            pub fn from_json(value: Option<serde_json::Value>) -> Result<Option<Self>, serde_json::Error> {
                if let Some(value) = value {
                    Ok(Some(serde_json::from_value(value)?))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

#[macro_use]
macro_rules! id64 {
    ($vis:vis $name:ident) => {
        #[serde(transparent)]
        #[derive(sqlx::Type, serde::Serialize, serde::Deserialize, Debug, PartialEq)]
        $vis struct $name(pub i64);

        impl std::ops::Deref for $name {
            type Target = i64;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    }
}

#[macro_use]
macro_rules! db_enum {
    (
        $( #[$attr:meta] )*
        $vis:vis enum $name:ident $fields:tt
    ) => {
        $( #[$attr] )*
        #[derive(
            sqlx::Type, strum_macros::EnumString, strum_macros::Display,
            strum_macros::AsRefStr, serde::Serialize, serde::Deserialize,
            Debug, Clone, PartialEq
        )]
        $vis enum $name $fields
    }
}
