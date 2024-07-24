use std::collections::HashMap;
use std::default;
use std::io::{Read, Write, Result};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code:&'a str,
    status_text:&'a str,
    headers:Option<HashMap<&'a str, &'a str>>,
    body:Option<String>
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse) -> String {
        let res1 = res.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res1.version(),
            &res1.status_code(),
            &res1.status_text(),
            &res1.headers(),
            &res.body.unwrap().len(),
            &res1.body(),
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>
    ) -> HttpResponse<'a> {
        let mut response = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        }
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-type", "text/html");
                Some(h)
            }
        };

        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Unknown".into(),
        };
        response.body = body;

        response

    }

    pub fn send_response(&self, stream:&mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string = String::from(res);
        let _ = write!(stream, "{}", response_string);

        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let map = self.headers.clone().unwrap();
        let mut hStr = String::from("");
        for (k, v) in map.iter() { 
            hStr = format!("{}{}: {}\r\n", hStr, k, v);
        };
        hStr
    }

    fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_200() {
        let response_real = HttpResponse::new("200", None, Some("test".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-type", "text/html");
                Some(h)
            },
            body: Some("test".into()),
        };
        assert_eq!(response_real, response_expected);
    }

    #[test]
    fn test_response_404() {
        let response_real = HttpResponse::new("404", None, Some("test".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-type", "text/html");
                Some(h)
            },
            body: Some("test".into()),
        };
        assert_eq!(response_real, response_expected);
    }

    #[test]
    fn test_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-type", "text/html");
                Some(h)
            },
            body: Some("test".into()),
        };
        let response_str: String = response_expected.into();
        let real_str = "HTTP/1.1 404 Not Found\r\nContent-type: text/html\r\nContent-Length: 4\r\n\r\ntest".to_string();
        assert_eq!(response_str, real_str);
    }

}