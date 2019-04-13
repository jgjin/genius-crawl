use std::{
    collections::{
        BTreeMap as Map,
    },
};

use serde::{
    Deserialize,
    Serialize,
};
use serde_json::{
    Value,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseMetadata {
    status: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseHits<Hit> {
    pub hits: Vec<Hit>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct APIResponse<Hit> {
    meta: ResponseMetadata,
    pub response: ResponseHits<Hit>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtistMetadata {
    api_path: String,
    header_image_url: String,
    id: i32,
    image_url: String,
    is_meme_verified: bool,
    is_verified: bool,
    pub name: String,
    url: String,
    iq: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stats {
    hot: bool,
    pageviews: Option<i32>,
    unreviewed_annotations: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResult {
    annotation_count: i32,
    pub api_path: String,
    full_title: String,
    header_image_thumbnail_url: String,
    header_image_url: String,
    id: i32,
    lyrics_owner_id: i32,
    lyrics_state: String,
    path: String,
    pub primary_artist: ArtistMetadata,
    pyongs_count: Option<i32>,
    song_art_image_thumbnail_url: String,
    stats: Stats,
    pub title: String,
    title_with_featured: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchHit {
    pub result: SearchResult,
    #[serde(flatten)]
    other: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtistTrack {
    #[serde(rename = "origin_artist_name")]
    pub artist: String,
    #[serde(rename = "track_name")]
    pub track: String,
    #[serde(flatten)]
    other: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchCsv {
    artist_name: String,
    track_name: String,
    api_path: Option<String>,
}

impl SearchCsv {
    pub fn from(
        artist_track: ArtistTrack,
        api_path: Option<String>,
    ) -> Self {
        Self {
            artist_name: artist_track.artist,
            track_name: artist_track.track,
            api_path: api_path,
        }
    }
}
