
macro_rules! generate_mime_functions {
    ($($ext:expr => $mime:expr),*;$($ext_one_way:expr => $mime_one_way:expr),*) => {
        pub fn extension_to_mime(extension: &str) -> Option<&'static str> {
            match extension {
                $($ext => Some($mime),)*
                _ => None,
            }
        }

        pub fn mime_to_extension(mime_type: &str) -> Option<&'static str> {// TODO Continuar con el mime one way
            match mime_type {
                $($mime => Some($ext),)*
                _ => None,
            }
        }
    };
}

generate_mime_functions!(
    "aac" => "audio/aac",
    "html" => "text/html",
    "css" => "text/css",
    "js" => "text/javascript",
    "json" => "application/json",
    "png" => "image/png",
    "jpg" => "image/jpeg",
    "gif" => "image/gif",
    "svg" => "image/svg+xml",
    "ico" => "image/x-icon",
    "webp" => "image/webp",
    "mp4" => "video/mp4",
    "webm" => "video/webm",
    "ogg" => "video/ogg",
    "mp3" => "audio/mpeg",
    "wav" => "audio/wav",
    "flac" => "audio/flac",
    "txt" => "text/plain";
    "jpeg" => "image/jpeg"
);

pub(crate) use generate_mime_functions;