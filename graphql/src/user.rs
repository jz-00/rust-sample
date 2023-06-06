use juniper::{GraphQLObject, GraphQLScalarValue, ID};

use super::image::Image;

#[derive(GraphQLScalarValue, Clone)]
pub struct UserId(pub ID);

#[derive(GraphQLObject, Default)]
pub struct UserStats {
    pub follower_count: i32,
    pub following_count: i32,
    pub contact_count: i32,
    pub drink_count: i32,
    pub review_count: i32,
    pub tasting_count: i32,
    pub library_wine_count: i32,
}

/// User info, public connections, and metadata.
#[derive(GraphQLObject)]
pub struct UserProfile {
    pub id: UserId,
    pub name: String,
    pub image: Option<Image>,
    pub whereabouts: Option<String>,
    pub followers: Vec<UserProfile>,
    pub following: Vec<UserProfile>,
    pub stats: UserStats,
}

impl UserProfile {
    pub fn new(id: UserId, name: &str) -> UserProfile {
        UserProfile {
            id,
            name: name.to_owned(),
            image: None,
            whereabouts: None,
            followers: vec![],
            following: vec![],
            stats: UserStats::default(),
        }
    }
}

/// User profile, settings, connections, and metadata.
#[derive(GraphQLObject)]
pub struct UserAccount {
    pub id: UserId,
    pub profile: UserProfile,
    // pub libraries: Vec<Library>,
    // pub organizations: Vec<Organization>,
    // pub legal: UserAgreements,
    // pub settings: UserSettings,
    // pub contacts: Contacts,
}
