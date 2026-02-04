use std::path::PathBuf;

use url::Url;

use crate::pipeline::Feedback;

pub fn texture_path_from_url(uri: &Url, feedback: &Feedback) -> Option<PathBuf> {
    if let Ok(path) = uri.to_file_path() {
        return Some(path);
    }

    let raw = uri.as_str();
    let mut s = if let Some(rest) = raw.strip_prefix("file://") {
        rest.to_string()
    } else {
        raw.to_string()
    };

    if s.starts_with('/') && s.get(2..3) == Some(":") {
        s.remove(0);
    }

    if s.is_empty() {
        feedback.warn(format!("Texture URI is empty, skipping: {uri}"));
        return None;
    }

    if !s.contains(".zip/") && !s.contains(".zip\\") {
        feedback.warn(format!("Texture URI is not a file path, skipping: {uri}"));
        return None;
    }

    Some(PathBuf::from(s))
}
