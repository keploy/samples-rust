use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct NoteData {
    pub note: NoteResponse,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct NoteResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub published: bool,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct NoteListResponse {
    pub status: String,
    pub results: usize,
    pub notes: Vec<NoteResponse>,
}

#[derive(Serialize, Debug)]
pub struct SingleNoteResponse {
    pub status: String,
    pub data: NoteData,
}