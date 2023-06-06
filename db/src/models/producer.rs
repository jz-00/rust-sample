use super::prelude::*;

id64!(pub ProducerId);

#[derive(Debug)]
pub struct Producer {
    pub id: ProducerId,
    pub created: OffsetDateTime,
    pub name: String,
    pub image: Option<ImageSummary>,
    pub places: Vec<Place>,
    pub wines: Vec<Wine>,
}

#[derive(Debug)]
pub struct ProducerSummary {
    pub id: ProducerId,
    pub name: String,
    pub image: Option<ImageSummary>,
}

pub struct ProducerInput {
    pub name: String,
    pub image_id: Option<ImageId>,
    // place data...
}

// pub async fn create_producer(db: &Pool, input: ProducerInput) {}

// pub async fn update_producer(db: &Pool, id: ProducerId, input: ProducerInput) {}

// pub async fn get_producer(db: &Pool, id: Option<ProducerId>) -> Result<Option<Producer>> {
//     Ok(None)
// }

pub async fn get_producer_summary(
    db: &Pool,
    id: Option<ProducerId>,
) -> Result<Option<ProducerSummary>> {
    if let Some(id) = id {
        let producer = query!(
            r#"
            SELECT
                id, name, image
            FROM producers
            WHERE id = $1
            "#,
            id.0
        )
        .fetch_one(db)
        .await?;

        Ok(Some(ProducerSummary {
            id: ProducerId(producer.id),
            name: producer.name,
            image: ImageSummary::from_json(producer.image)?,
        }))
    } else {
        Ok(None)
    }
}

// pub async fn delete_producer(db: &Pool, id: ProducerId) {}
