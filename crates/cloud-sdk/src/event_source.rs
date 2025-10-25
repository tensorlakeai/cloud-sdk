//! # SSE Decoder for Server-Sent Events
//!
//! This module provides a decoder for Server-Sent Events (SSE) that extracts JSON data
//! from "data: {json}" lines and deserializes them.

use bytes::{Buf, BytesMut};
use pin_project_lite::pin_project;
use serde::de::DeserializeOwned;
use std::io;
use tokio_util::codec::Decoder;

/// Error type for SSE decoding.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pin_project! {
    #[derive(Debug)]
    pub struct SseDecoder<T> {
        ty: std::marker::PhantomData<T>,
    }
}

impl<T> SseDecoder<T> {
    #[inline]
    pub fn new() -> SseDecoder<T> {
        SseDecoder {
            ty: std::marker::PhantomData,
        }
    }
}

impl<T> Decoder for SseDecoder<T>
where
    T: DeserializeOwned,
{
    type Item = T;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let nl_index = src.iter().position(|b| *b == b'\n');

        if !src.is_empty() {
            if let Some(pos) = nl_index {
                let remainder = src.split_off(pos + 1);
                let line = &src[..src.len() - 1]; // Remove the newline

                if line.starts_with(b"data: ") {
                    let json_slice = &line[6..]; // Skip "data: "
                    match serde_json::from_slice(json_slice) {
                        Ok(json) => {
                            src.unsplit(remainder);
                            src.advance(pos + 1);
                            Ok(Some(json))
                        }
                        Err(e) if e.is_eof() => {
                            // Unescaped newline inside JSON, put back and wait for more data
                            src.truncate(src.len() - 1); // Remove the newline
                            src.unsplit(remainder);
                            Ok(None)
                        }
                        Err(e) => Err(e.into()),
                    }
                } else {
                    // Not a data line, skip
                    src.unsplit(remainder);
                    src.advance(pos + 1);
                    Ok(None)
                }
            } else {
                // No newline, check if it's a data line
                if src.starts_with(b"data: ") {
                    let json_slice = &src[6..];
                    match serde_json::from_slice(json_slice) {
                        Ok(json) => {
                            src.clear();
                            Ok(Some(json))
                        }
                        Err(e) if e.is_eof() => Ok(None),
                        Err(e) => Err(e.into()),
                    }
                } else {
                    // Not a data line, clear or wait
                    Ok(None)
                }
            }
        } else {
            Ok(None)
        }
    }
}
