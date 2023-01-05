use super::method::Method ;
// Request struct, no body or headers.
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}