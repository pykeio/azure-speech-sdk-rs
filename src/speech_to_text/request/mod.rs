pub(crate) mod speech_config;
pub(crate) mod speech_context;

use chrono::{SecondsFormat, Utc};
use serde::{ Serialize};
use tokio_tungstenite::tungstenite::Message as TMessage;
use crate::speech_to_text::request::speech_config::SpeechConfig;
use crate::speech_to_text::request::speech_context::SpeechContext;

const CRLF: &str = "\r\n";

pub(crate) enum Request {
    SpeechConfig { session_id: String, data: SpeechConfig },
    SpeechContext { session_id: String, data: SpeechContext },
    StartAudio { session_id: String, content_type: String, data: Vec<u8> },
    Audio { session_id: String, data: Option<Vec<u8>> },
}

impl Into<TMessage> for Request {
    fn into(self) -> TMessage {
        match self {
            Request::SpeechConfig { session_id, data } => make_text_message("speech.config".to_string(), session_id, Some("application/json".to_string()), Some(data)),
            Request::SpeechContext { session_id, data } => make_text_message("speech.context".to_string(), session_id, Some("application/json".to_string()), Some(data)),
            Request::StartAudio { session_id, content_type, data } => make_binary_message("audio".to_string(), session_id, Some(content_type), Some(data)),
            Request::Audio { session_id, data } => make_binary_message("audio".to_string(), session_id, None, data)
        }
    }
}

fn make_text_message<T: Serialize>(path: String, session_id: String, content_type: Option<String>, data: Option<T>) -> TMessage {
    let mut headers = vec![("Path".to_string(), path), ("X-RequestId".to_string(), session_id)];

    headers.append(&mut vec![
        ("X-Timestamp".to_string(), Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)),
    ]);

    if let Some(content_type) = content_type {
        headers.append(&mut vec![("Content-Type".to_string(), content_type)]);
    }

    let headers = transform_headers_to_string(headers);

    if let Some(ref d) = data {
        TMessage::Text(format!("{}{CRLF}{}", headers, serde_json::to_string(d).unwrap()))
    } else {
        TMessage::Text(format!("{}{CRLF}", headers))
    }
}


fn transform_headers_to_string(map: Vec<(String, String)>) -> String {
    let mut header = String::new();
    for (content_type, value) in map {
        header.push_str(format!("{content_type}: {value}{CRLF}").as_str());
    }

    header
}

fn make_binary_payload(headers: String, data: Option<Vec<u8>>) -> Vec<u8> {
    let data_length = if let Some(ref d) = data {
        d.len()
    } else {
        0
    };

    let header_buffer: Vec<_> = headers.bytes().collect();
    let header_length = header_buffer.len();
    let mut payload = vec![0; 2 + header_length + data_length];
    payload[0] = ((header_length >> 8) & 0xff) as u8;
    payload[1] = (header_length & 0xff) as u8;
    payload[2..2 + header_length].copy_from_slice(&header_buffer);

    if let Some(ref d) = data {
        payload[2 + header_length..].copy_from_slice(d);
    }

    payload
}

fn make_binary_message(path: String, session_id: String, content_type: Option<String>, data: Option<Vec<u8>>) -> TMessage {
    let mut headers = vec![("Path".to_string(), path), ("X-RequestId".to_string(), session_id)];

    headers.append(&mut vec![
        ("X-Timestamp".to_string(), Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)),
    ]);

    if let Some(content_type) = content_type {
        headers.append(&mut vec![("Content-Type".to_string(), content_type)]);
    }

    let payload = make_binary_payload(transform_headers_to_string(headers), data);

    TMessage::Binary(payload)
}