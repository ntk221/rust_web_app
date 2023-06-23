use std::collections::HashMap;
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

// Note the use of lifetime annotation 'a for all the member fields that are of reference types.
