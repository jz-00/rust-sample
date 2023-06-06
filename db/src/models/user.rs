use super::prelude::*;

id64!(pub UserId);

#[derive(Debug)]
pub struct User {
    pub id: UserId,
    pub created: OffsetDateTime,
    pub uid: Option<String>,
    pub name: String,
    pub whereabouts: Option<String>,
    pub image: Option<ImageSummary>,
    pub settings: Settings,
    pub active: time::OffsetDateTime,

    // stats
    pub followers: i32,
    pub following: i32,
    pub contacts: i32,
    pub drinks: i32,
    pub tastings_organized: i32,
    pub tastings_attended: i32,
}

#[derive(Debug)]
pub struct UserPrivacy {
    pub user_id: UserId,
    pub drinks: Visibility,
    pub drink_places: Visibility,
    pub drink_reviews: Visibility,
    pub tastings: Visibility,
    pub acquisitions: Visibility,
    pub acquisition_dates: Visibility,
    pub acquisition_places: Visibility,
    pub acquisition_costs: Visibility,
}

pub(crate) mod json {
    use crate::models::enums::AppTheme;

    jsonb! {
        pub struct Settings {
            pub app: AppSettings,
        }
    }

    jsonb! {
        pub struct AppSettings {
            pub theme: AppTheme,
        }
    }
}

pub use json::*;
