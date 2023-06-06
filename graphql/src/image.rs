use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct Thumbnail {
    pub url: String,
    pub width: i32,
    pub height: i32,
    pub aspect_ratio: f64,
}

#[derive(GraphQLObject)]
pub struct Image {
    pub url: String,
    pub width: i32,
    pub height: i32,
    pub aspect_ratio: f64,
    pub thumbnail: Thumbnail,
}
