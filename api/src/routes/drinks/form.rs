use std::collections::HashSet;

use axum::http::StatusCode;
use axum_extra::extract::{
    Multipart,
    multipart::{Field, MultipartError},
};

use crate::{
    domain::drinks::{Abv, DrinkImage, MAX_IMAGE_SIZE},
    routes::ApiError,
    services::drinks::NewDrink,
};

const MAX_TEXT_LENGTH: usize = 256;

pub(super) async fn parse(mut multipart: Multipart) -> Result<NewDrink, ApiError> {
    let mut image = None;
    let mut drink_type_id = None;
    let mut drink_size_id = None;
    let mut abv = None;
    let mut seen = HashSet::new();

    while let Some(field) = multipart.next_field().await.map_err(multipart_error)? {
        let name = field.name().unwrap_or_default().to_owned();
        if !seen.insert(name.clone()) {
            return Err(invalid("duplicate form field"));
        }

        match name.as_str() {
            "image" => image = Some(read_image(field).await?),
            "drinkTypeId" => drink_type_id = read_text(field).await?,
            "drinkSizeId" => drink_size_id = read_text(field).await?,
            "abv" => abv = parse_abv(read_text(field).await?)?,
            _ => return Err(invalid("unknown form field")),
        }
    }

    Ok(NewDrink {
        image: image.ok_or_else(|| invalid("image is required"))?,
        drink_type_id,
        drink_size_id,
        abv,
    })
}

async fn read_image(mut field: Field) -> Result<DrinkImage, ApiError> {
    let content_type = field.content_type().unwrap_or_default().to_owned();
    if !content_type.starts_with("image/") {
        return Err(invalid("only image files are allowed"));
    }

    let extension = field
        .file_name()
        .and_then(|name| name.rsplit_once('.'))
        .map(|(_, extension)| extension.to_owned())
        .unwrap_or_else(|| "jpg".into());

    let mut bytes = Vec::new();
    while let Some(chunk) = field.chunk().await.map_err(multipart_error)? {
        if bytes.len() + chunk.len() > MAX_IMAGE_SIZE {
            return Err(ApiError::Client {
                status: StatusCode::PAYLOAD_TOO_LARGE,
                code: "image_too_large",
                message: "image must be at most 10 MiB",
            });
        }
        bytes.extend_from_slice(&chunk);
    }

    DrinkImage::parse(bytes, content_type, extension).map_err(|_| invalid("invalid image"))
}

async fn read_text(field: Field) -> Result<Option<String>, ApiError> {
    let value = field.text().await.map_err(multipart_error)?;
    if value.len() > MAX_TEXT_LENGTH {
        return Err(invalid("form field is too long"));
    }
    Ok((!value.is_empty()).then_some(value))
}

fn parse_abv(value: Option<String>) -> Result<Option<Abv>, ApiError> {
    let Some(value) = value else {
        return Ok(None);
    };
    let value = value
        .parse()
        .map_err(|_| invalid("invalid alcohol percentage"))?;
    let abv = Abv::parse(value).map_err(|_| invalid("invalid alcohol percentage"))?;
    Ok(Some(abv))
}

fn invalid(message: &'static str) -> ApiError {
    ApiError::Client {
        status: StatusCode::BAD_REQUEST,
        code: "invalid_drink",
        message,
    }
}

fn multipart_error(error: MultipartError) -> ApiError {
    ApiError::Client {
        status: error.status(),
        code: "invalid_multipart",
        message: "invalid or oversized multipart body",
    }
}
