use super::prelude::*;

id64!(pub ImageId);

#[derive(Debug)]
pub struct Image {
    pub id: ImageId,
    pub created: OffsetDateTime,
    pub user_id: UserId,
    pub owner_id: Option<UserId>,
    pub url: String,
    pub width: i32,
    pub height: i32,
    pub aspect_ratio: f32,
    pub thumbnail_url: String,
    pub thumbnail_width: i32,
    pub thumbnail_height: i32,
    pub thumbnail_aspect_ratio: f32,
    pub colors: Option<ImageColors>,
}

pub(crate) mod json {
    use super::*;

    jsonb! {
        pub struct ImageColors {
            pub primary: String,
            pub secondary: String,
            pub accent: String,
        }
    }

    jsonb! {
        // minimal image data denormalized to other entities
        pub struct ImageSummary {
            pub url: String,
            pub width: i32,
            pub height: i32,
            pub thumbnail_url: String,
            pub thumbnail_width: i32,
            pub thumbnail_height: i32,
            pub colors: Option<ImageColors>,
        }
    }

    impl ImageSummary {
        pub fn new(image: Image) -> ImageSummary {
            let colors = if let Some(colors) = image.colors {
                Some(colors)
            } else {
                None
            };

            ImageSummary {
                url: image.url,
                width: image.width,
                height: image.height,
                thumbnail_url: image.thumbnail_url,
                thumbnail_width: image.thumbnail_width,
                thumbnail_height: image.thumbnail_height,
                colors,
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct ImageInput {
    pub user_id: Option<UserId>,
    pub owner_id: Option<UserId>,
    pub url: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub thumbnail_url: String,
    pub thumbnail_width: Option<i32>,
    pub thumbnail_height: Option<i32>,
    pub thumbnail_aspect_ratio: Option<f32>,
    pub colors: Option<ImageColors>,
}

pub use json::*;
// pub type OptionalImageSummary = Option<Json<ImageSummary>>;

// pub async fn create_image(db: &Pool, input: ImageInput) {}

// pub async fn update_image(db: &Pool, id: ImageId, input: ImageInput) {}

pub async fn get_image(db: &Pool, id: Option<ImageId>) -> Result<Option<Image>> {
    if let Some(id) = id {
        let image = query!(
            r#"
        SELECT *
        FROM images
        WHERE id = $1
        "#,
            id.0
        )
        .fetch_one(db)
        .await?;

        Ok(Some(Image {
            id: ImageId(image.id),
            created: image.created,
            user_id: UserId(image.user_id),
            owner_id: image.owner_id.map(UserId),
            url: image.url,
            width: image.width,
            height: image.height,
            aspect_ratio: image.aspect_ratio,
            thumbnail_url: image.thumbnail_url,
            thumbnail_width: image.thumbnail_width,
            thumbnail_height: image.thumbnail_height,
            thumbnail_aspect_ratio: image.thumbnail_aspect_ratio,
            colors: ImageColors::from_json(image.colors)?,
        }))
    } else {
        Ok(None)
    }
}

pub async fn get_image_summary(db: &Pool, id: Option<ImageId>) -> Result<Option<ImageSummary>> {
    if let Some(id) = id {
        let image = query!(
            r#"
            SELECT
                url, width, height,
                thumbnail_url, thumbnail_width, thumbnail_height,
                colors
            FROM images
            WHERE id = $1
            "#,
            id.0
        )
        .fetch_one(db)
        .await?;

        Ok(Some(ImageSummary {
            url: image.url,
            width: image.width,
            height: image.height,
            thumbnail_url: image.thumbnail_url,
            thumbnail_width: image.thumbnail_width,
            thumbnail_height: image.thumbnail_height,
            colors: ImageColors::from_json(image.colors)?,
        }))
    } else {
        Ok(None)
    }
}

// pub async fn delete_image(db: &Pool, id: ImageId) {}
