use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaItems {
    pub media_items: Vec<MediaItem>,
    pub next_page_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaItem {
    pub id: String,
    pub product_url: String,
    pub base_url: String,
    pub mime_type: String,
    pub media_metadata: MediaMetadata,
    pub filename: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaMetadata {
    pub creation_time: String,
    pub width: String,
    pub height: String,
    pub photo: Photo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
    pub camera_make: String,
    pub camera_model: String,
    pub focal_length: i64,
    #[serde(rename = "apertureFNumber")]
    pub aperture_fnumber: i64,
    pub iso_equivalent: i64,
    pub exposure_time: String,
}
