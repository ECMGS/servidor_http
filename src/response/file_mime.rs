
macro_rules! generate_mime_functions {
    ($($ext:expr => $mime:expr),*) => {
        const MIME_PAIRS: &[(&str, &str)] = &[
            $(($ext, $mime),)*
        ]; // constant to store MIME pairs

        /// Function to translate file extensions into MIMEs. If the extension doesn't exist, it will return None.
        ///
        /// # Arguments
        ///
        /// * `extension`: File extension in plain text, without period
        ///
        /// returns: Option<&str>: MIME as &str or None
        pub fn extension_to_mime(extension: &str) -> Option<&'static str> {
            MIME_PAIRS.iter()
                .find(|&&(ext, _)| ext == extension)
                .map(|&(_, mime)| mime)
        }
        /// Function that returns the first extension associated with a Mime, if the function doesn't find a solution, it will return a None.
        /// 
        /// # Arguments 
        /// 
        /// * `mime_type`: MIME in plain text.
        /// 
        /// returns: Option<&str>: extension as &str or None
        pub fn _mime_to_extension(mime_type: &str) -> Option<&'static str> {
            MIME_PAIRS.iter()
                .find(|&&(_, mime)| mime == mime_type)
                .map(|&(ext, _)| ext)
        }
    };
}

generate_mime_functions!(
    "aac" => "audio/aac",
    "abw" => "application/x-abiword",
    "arc" => "application/octet-stream",
    "avi" => "video/x-msvideo",
    "azw" => "application/vnd.amazon.ebook",
    "bin" => "application/octet-stream",
    "bz" => "application/x-bzip",
    "bz2" => "application/x-bzip2",
    "csh" => "application/x-csh",
    "css" => "text/css",
    "csv" => "text/csv",
    "doc" => "application/msword",
    "epub" => "application/epub+zip",
    "gif" => "image/gif",
    "ics" => "text/calendar",
    "jar" => "application/java-archive",
    "midi" => "audio/midi",
    "mid" => "audio/midi",
    "mpeg" => "video/mpeg",
    "mpkg" => "application/vnd.apple.installer+xml",
    "odp" => "application/vnd.oasis.opendocument.presentation",
    "ods" => "application/vnd.oasis.opendocument.spreadsheet",
    "odt" => "application/vnd.oasis.opendocument.text",
    "oga" => "audio/ogg",
    "ogv" => "video/ogg",
    "ogx" => "application/ogg",
    "pdf" => "application/pdf",
    "ppt" => "application/vnd.ms-powerpoint",
    "rar" => "application/x-rar-compressed",
    "rtf" => "application/rtf",
    "sh" => "application/x-sh",
    "tar" => "application/x-tar",
    "tif" => "image/tiff",
    "tiff" => "image/tiff",
    "ttf" => "font/ttf",
    "vsd" => "application/vnd.visio",
    "weba" => "audio/webm",
    "webm" => "video/webm",
    "webp" => "image/webp",
    "woff" => "font/woff",
    "woff2" => "font/woff2",
    "xhtml" => "application/xhtml+xml",
    "xls" => "application/vnd.ms-excel",
    "xml" => "application/xml",
    "xul" => "application/vnd.mozilla.xul+xml",
    "zip" => "application/zip",
    "3gp" => "video/3pgp",
    "3gp" => "audio/3gpp",
    "3g2" => "video/3gpp2",
    "3g2" => "audio/3gpp2",
    "7z" => "application/x-7z-compressed",
    "html" => "text/html",
    "js" => "text/javascript",
    "json" => "application/json",
    "png" => "image/png",
    "jpg" => "image/jpeg",
    "svg" => "image/svg+xml",
    "ico" => "image/x-icon",
    "webp" => "image/webp",
    "mp4" => "video/mp4",
    "webm" => "video/webm",
    "ogg" => "video/ogg",
    "mp3" => "audio/mpeg",
    "wav" => "audio/x-wav",
    "flac" => "audio/flac",
    "txt" => "text/plain",
    "jpeg" => "image/jpeg",
    "htm" => "text/html"
);

pub(crate) use generate_mime_functions;