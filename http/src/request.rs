use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct  HttpRequest {
    pub method: Method,
    pub version:Version,
    pub resource:Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<&str> for Method {
    fn from(value: &str) -> Method {
        match value {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        } 
    }
}

impl From<&str> for Version {
    fn from(value: &str) -> Version {
    
        match value {
            "HTTP/1.1" => Version::V1_1,
            // "2.0" => Method::V2_0,
            _ => Version::Uninitialized,
        } 
    }
}

impl From<String> for HttpRequest {
    fn from(req: String) -> HttpRequest {

        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path(String::from(""));
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = String::from("");
    
        for line in req.lines() { 
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
                // 空行
            } else {
                parsed_msg_body = line.to_string();
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body,
        }
    }
}

fn process_header_line(s: &str) -> (String, String) {
    let mut headers = s.split(":");
    let mut key = "".to_string();
    let mut value = "".to_string();
    if let Some(k) = headers.next() { 
        key = k.to_string();
    };
    if let Some(v) = headers.next() { 
        value = v.to_string();
    };

    (key, value)
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut contents = s.split_whitespace();
    let method = contents.next().unwrap();
    let resource = contents.next().unwrap();
    let version = contents.next().unwrap();

    (
        method.into(), 
        Resource::Path(resource.to_string()),
        version.into()
    )

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method() {
        // Method 实现了 from ，所以可以 into 可以转化为 Method
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s = "GET /test HTTP/1.1\r\nHost: localhost:8989\r\nUser-Agent: chrom/1.1\r\nAccept: */*\r\n\r\n".to_string();
        let mut test_headers = HashMap::new();
        test_headers.insert("Host".into(), " localhost".into());
        test_headers.insert("Accept".into(), " */*".into());
        test_headers.insert("User-Agent".into(), " chrom/1.1".into());

        let req: HttpRequest = s.into();

        assert_eq!(req.method, Method::Get);
        assert_eq!(req.version, Version::V1_1);
        assert_eq!(req.resource, Resource::Path(String::from("/test")));
        assert_eq!(req.headers, test_headers);

    }

}