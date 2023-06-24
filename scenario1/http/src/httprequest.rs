use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

/*
trait From<&str> {
    fn from(s: &str) -> Method;
}
*/

/*
    From<&str>
    This trait enables conversion of incoming string slice into HttpRequest data structure
*/
impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub resource: Resource,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        // 1. 受信 HTTP リクエストの各行を読み取ります。各行はCRLFで区切られます。
        // 2. 各行を以下の様に評価する
        /*
           - その行がリクエスト行の場合(すべてのリクエスト行がHTTPキーワードとバージョン番号を含むので、リクエスト行かどうかをチェックするためにHTTPキーワードを探している)、その行からメソッド、パス、HTTPバージョンを抽出する。
           - その行がヘッダー行(区切り文字 ':' で識別される)であれば、ヘッダー項目のキーと値を抽出し、リクエストのヘッダーリストに追加する。HTTPリクエストには複数のヘッダー行があり得ることに注意してください。物事を単純にするために、keyとvalueは印字可能なASCII文字(10進数で33から126の間の値を持つ文字で、コロンを除く)で構成されていなければならないと仮定しよう。
           - 空行(empty line)の場合は、区切り行として扱う。この場合、何もする必要はない。
           - メッセージ本文がある場合は、スキャンしてStringとして格納する。
        */

        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";
        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.is_empty() {
            } else {
                parsed_msg_body = line;
            }
        }
        HttpRequest {
            method: (parsed_method),
            resource: (parsed_resource),
            version: (parsed_version),
            headers: (parsed_headers),
            msg_body: (parsed_msg_body.to_string()),
        }
    }
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
    // Parse the request line into individual chunks split by whitespaces.
    let mut words = s.split_whitespace();
    // Extract the HTTP method from first part of the request line
    let method = words.next().unwrap();
    // Extract the resource (URI/URL) from second part of the request line
    let resource = words.next().unwrap();
    // Extract the HTTP version from third part of the request line
    let version = words.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}
fn process_header_line(s: &str) -> (String, String) {
    let mut iter = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = iter.next() {
        key = k.to_string();
    }
    if let Some(v) = iter.next() {
        value = v.to_string();
    }
    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_from_str() {
        assert_eq!(Method::from("GET"), Method::Get);
        assert_eq!(Method::from("POST"), Method::Post);
        assert_eq!(Method::from("PUT"), Method::Uninitialized);
    }

    #[test]
    fn test_version_from_str() {
        assert_eq!(Version::from("HTTP/1.1"), Version::V1_1);
        assert_eq!(Version::from("HTTP/2.0"), Version::V2_0);
        assert_eq!(Version::from("HTTP/3.0"), Version::Uninitialized);
    }

    #[test]
    fn test_httprequest() {
        let s = String::from("GET / HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");
        let request = HttpRequest::from(s.as_str());
        assert_eq!(request.method, Method::Get);
        assert_eq!(request.resource, Resource::Path(String::from("/")));
        assert_eq!(request.version, Version::V1_1);
        let mut headers_expected = HashMap::new();
        headers_expected.insert(String::from("Host"), String::from(" localhost"));
        headers_expected.insert(String::from("User-Agent"), String::from(" curl/7.64.1"));
        headers_expected.insert(String::from("Accept"), String::from(" */*"));
        assert_eq!(request.headers, headers_expected);
    }
}
