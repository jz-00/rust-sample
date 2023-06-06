use super::prelude::*;

id64!(pub PlaceId);

#[derive(Debug)]
pub struct Place {
    pub id: PlaceId,
    pub created: OffsetDateTime,
    pub name: String,
    pub image: Option<ImageSummary>,
    pub street: String,
    pub city: String,
    pub country: Country,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    pub url: Option<String>,
    // TODO
    // location GEOMETRY(POINT, 4326),
    pub notes: Option<String>,
    pub type_tags: Vec<String>,
}
