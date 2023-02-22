use std::{env, io};

use axum::{
    body::{Bytes, StreamBody},
    extract::{Multipart, Path, Query},
    BoxError,
};
use futures::{Stream, TryStreamExt};
use hyper::{Body, Request};
use mediatype::MediaType;
use serde::Deserialize;
use tokio::{fs::File, io::BufWriter};
use tokio_util::{
    codec::{BytesCodec, FramedRead},
    io::StreamReader,
};
use uuid::Uuid;

use crate::{
    converter::Converter,
    error::{Error, Result},
    helper::empty_string_as_none,
};

pub async fn upload_file(mut multipart: Multipart) -> Result<()> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = if let Some(file_name) = field.file_name() {
            file_name.to_owned()
        } else {
            continue;
        };

        stream_to_file(&file_name, field).await?;
    }

    Ok(())
}

async fn stream_to_file<S, E>(file_name: &str, stream: S) -> Result<()>
where
    S: Stream<Item = std::result::Result<Bytes, E>>,
    E: Into<BoxError>,
{
    // Convert the stream into an `AsyncRead`.
    let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
    let body_reader = StreamReader::new(body_with_io_error);
    futures::pin_mut!(body_reader);

    let storage_path = env::var("STORAGE_PATH").unwrap_or_else(|_| "data".to_owned());
    let file_id = Uuid::new_v4();
    let path = format!("./{storage_path}/{file_id}/original");

    if !std::path::Path::new(&path).exists() {
        tokio::fs::create_dir_all(&path).await.unwrap();
    }

    let file_path = std::path::Path::new(&path).join(file_name);
    let mut file = BufWriter::new(File::create(file_path).await.unwrap());

    tokio::io::copy(&mut body_reader, &mut file).await.unwrap();

    Ok(())
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct LoadFileParams {
    format: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    width: Option<u32>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    height: Option<u32>,
}

pub async fn load_file(
    Path(file_name): Path<String>,
    Query(query_params): Query<LoadFileParams>,
    _request: Request<Body>,
) -> Result<StreamBody<FramedRead<File, BytesCodec>>> {
    let storage_path = env::var("STORAGE_PATH").unwrap_or_else(|_| "data".to_owned());
    let file_parts: Vec<&str> = file_name.split('.').collect();

    let file_id: Uuid = if let Ok(id) = Uuid::parse_str(&file_name) {
        id
    } else if file_parts.len().eq(&2) {
        Uuid::parse_str(file_parts.first().unwrap()).unwrap()
    } else {
        return Err(Error::MissingFileId);
    };

    let mt = if let Some(format) = query_params.format.as_ref() {
        MediaType::parse(format).unwrap()
    } else if file_parts.len().eq(&2) {
        MediaType::parse(file_parts.get(1).unwrap()).unwrap()
    } else {
        return Err(Error::NotFound);
    };

    if Converter::is_mime_supported(mt.clone()) {
        return Err(Error::MimeNotSupported);
    }

    let path = if let (Some(width), Some(height), Some(format)) =
        (query_params.width, query_params.height, query_params.format)
    {
        format!("./{storage_path}/{file_id}/{width}x{height}/{file_id}.{format}")
    } else {
        format!("./{storage_path}/{file_id}/original/{file_id}.png")
    };

    let file = File::open(path).await.unwrap();
    let stream = FramedRead::new(file, BytesCodec::new());
    Ok(StreamBody::new(stream))
}
