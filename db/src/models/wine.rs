use super::prelude::*;

id64!(pub WineId);

#[derive(Debug, PartialEq)]
pub struct Wine {
    pub id: WineId,
    pub created: OffsetDateTime,
    pub name: String,
    pub image: Option<ImageSummary>,
    pub color: WineColor,
    pub styles: Vec<String>,
    pub producer_id: Option<ProducerId>,
    pub producer_name: Option<String>,
    pub country: Country,
    pub regions: Vec<String>,
    pub classification_type: Option<String>,
    pub classification_name: Option<String>,
    pub grapes: Vec<String>,
    pub grape_count: i16,
    pub abv: Option<f32>,
    pub producer_notes: Option<String>,
    pub vintages: Vec<WineVintage>,
}

id64!(pub WineVintageId);

#[derive(Debug, PartialEq)]
pub struct WineVintage {
    pub id: WineVintageId,
    pub created: time::OffsetDateTime,
    pub wine_id: WineId,
    pub vintage: i16,
    pub image: Option<ImageSummary>,
    pub grapes: Vec<String>,
    pub grape_count: i16,
    pub abv: Option<f32>,
    pub producer_notes: Option<String>,
}

#[derive(Debug, Default)]
pub struct WineInput {
    pub name: String,
    pub image_id: Option<ImageId>,
    pub color: Option<WineColor>,
    pub styles: Option<Vec<String>>,
    pub producer_id: Option<ProducerId>,
    pub country: Option<Country>,
    pub regions: Option<Vec<String>>,
    pub classification_type: Option<String>,
    pub classification_name: Option<String>,
    pub grapes: Option<Vec<String>>,
    pub abv: Option<f32>,
    pub producer_notes: Option<String>,
    pub vintage: Option<i16>,
}

pub async fn create_wine(db: &Pool, input: WineInput) -> Result<Wine> {
    // unpack arguments and check for required

    let name = validate_str("name", input.name)?;
    let color = validate_required("color", input.color)?;
    let styles = validate_optional_str_vec("styles", input.styles)?;
    let country = validate_required("country", input.country)?;
    let regions = validate_optional_str_vec("regions", input.regions)?;

    // both classification type and nane are required if one is given
    let classification_type =
        validate_optional_str("classification_type", input.classification_type)?;
    let classification_name =
        validate_optional_str("classification_name", input.classification_name)?;

    if classification_type.is_some() && classification_name.is_none() {
        anyhow!(DbError::DependentArgument {
            name: "classification_type",
            depends_on: "classification_name",
        });
    } else if classification_type.is_none() && classification_name.is_some() {
        anyhow!(DbError::DependentArgument {
            name: "classification_name",
            depends_on: "classification_type",
        });
    }

    let grapes = validate_optional_str_vec("grapes", input.grapes)?;
    let grape_count = grapes.as_deref().unwrap_or_default().len() as i16;
    let producer_notes = validate_optional_str("produer_notes", input.producer_notes)?;
    let image = get_image_summary(db, input.image_id).await?;
    let producer = get_producer_summary(db, input.producer_id).await?;

    let mut tx = db.begin().await?;

    let wine = query!(
        r#"
        INSERT INTO wines (
            name, image, color, styles, producer_id,
            country, regions, classification_type, classification_name,
            grapes, grape_count, abv, producer_notes
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        RETURNING *
        "#,
        name,
        image.to_json(),
        color.to_string(),
        styles.as_deref(),
        producer.as_ref().map(|p| p.id.0),
        country.as_ref(),
        regions.as_deref(),
        classification_type,
        classification_name,
        grapes.as_deref(),
        grape_count,
        input.abv,
        producer_notes,
    )
    .fetch_one(&mut tx)
    .await?;

    // create WineVintage
    let wine_vintage = if let Some(vintage) = input.vintage {
        let vintage = query!(
            r#"
        INSERT INTO wine_vintages (
            wine_id, vintage, vintage_image, vintage_grapes,
            vintage_grape_count, vintage_abv, vintage_producer_notes
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#,
            wine.id,
            vintage,
            image.to_json(),
            grapes.as_deref(),
            grape_count,
            input.abv,
            producer_notes,
        )
        .fetch_one(&mut tx)
        .await?;

        Some(WineVintage {
            id: WineVintageId(vintage.id),
            created: vintage.created,
            wine_id: WineId(vintage.wine_id),
            vintage: vintage.vintage,
            image: ImageSummary::from_json(vintage.vintage_image)?,
            grapes: vintage.vintage_grapes.unwrap_or_default(),
            grape_count: vintage.vintage_grape_count,
            abv: vintage.vintage_abv,
            producer_notes: vintage.vintage_producer_notes,
        })
    } else {
        None
    };

    tx.commit().await?;

    // return wine + vintage
    Ok(Wine {
        id: WineId(wine.id),
        created: wine.created,
        name: wine.name,
        image: ImageSummary::from_json(wine.image)?,
        color: WineColor::from_str(&wine.color)?,
        styles: wine.styles.unwrap_or_default(),
        producer_id: wine.producer_id.map(ProducerId),
        producer_name: producer.map(|p| p.name),
        country: Country::from_str(&wine.country)?,
        regions: wine.regions.unwrap_or_default(),
        classification_type: wine.classification_type,
        classification_name: wine.classification_name,
        grapes: wine.grapes.unwrap_or_default(),
        grape_count: wine.grape_count,
        abv: wine.abv,
        producer_notes: wine.producer_notes,
        vintages: wine_vintage.map(|vintage| vec![vintage]).unwrap_or(vec![]),
    })
}

// pub async fn update_wine(db: &Pool, id: WineId, input: WineInput) {}

// pub async fn get_wine(db: &Pool, id: WineId) {}

// pub async fn get_wine_summary(db: &Pool, id: WineId) {}

// pub async fn delete_wine(db: &Pool, id: WineId) {}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::pool::create_default_pool;

    #[tokio::test]
    pub async fn test_create_wine() -> anyhow::Result<()> {
        let now = OffsetDateTime::now_utc();

        let wine = create_wine(
            &create_default_pool().await,
            WineInput {
                name: "A Wine".to_owned(),
                color: Some(WineColor::Red),
                country: Some(Country::IT),
                abv: Some(13.5),
                vintage: Some(2000),
                ..WineInput::default()
            },
        )
        .await?;

        let wine_vintage = &wine.vintages[0];

        assert!(wine.id.0 > 0);
        assert!(wine.created >= now);
        assert_eq!(wine.name, "A Wine");
        assert_eq!(wine.color, WineColor::Red);
        assert_eq!(wine.country, Country::IT);
        assert_eq!(wine.abv, Some(13.5));
        assert!(wine_vintage.id.0 > 0);
        assert!(wine_vintage.created >= now);
        assert_eq!(wine_vintage.wine_id, wine.id);
        assert_eq!(wine_vintage.image, wine.image);
        assert_eq!(wine_vintage.grapes, wine.grapes);
        assert_eq!(wine_vintage.grape_count, wine.grape_count);
        assert_eq!(wine_vintage.abv, wine.abv);
        assert_eq!(wine_vintage.producer_notes, wine.producer_notes);
        Ok(())
    }
}
