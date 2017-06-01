//! API client.

use std::str::FromStr;
use std::string::ToString;
use std::io::Read;

use serde_json;
use serde::de::DeserializeOwned;

use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::Client as HttpClient;

use hyper::status::StatusCode;
use hyper::header::Headers;
use hyper::header::{Connection, UserAgent, AcceptLanguage, qitem};
use hyper::LanguageTag;

use super::requester::Requester;
use super::{Version, Lang};

pub struct Client {
    version: Version,
    
    client: HttpClient,
    headers: Headers
}

impl Client {
    
    pub fn new(version: Version) -> Self {
        Client {
            client: Client::default_client(),
            headers: Client::headers(),
            
            version: version
        }
    }
    
    fn default_client() -> HttpClient {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = HttpClient::with_connector(connector);
        
        client
    }
    
    fn headers() -> Headers {
        let mut headers = Headers::new();
        headers.set(Connection::close());
        headers.set(UserAgent(format!("{}/{}", ::PACKAGE, ::VERSION)));
        
        let lang_tag = LanguageTag::from_str(&Lang::EN.to_string()).unwrap();
        headers.set(AcceptLanguage(vec![qitem(lang_tag)]));
        
        headers
    }
}

impl Requester for Client {
    fn request<T>(&self, version: Version, endpoint: &str) -> ::Result<T> where T: DeserializeOwned {
        let url = format!("{}/{}/{}", ::API_URL, version, endpoint);
        let mut response = self.client
                               .get(&url)
                               .headers(self.headers.clone())
                               .send()?;
        
        let mut body = String::new();
        response.read_to_string(&mut body)?;
    
        match response.status {
            StatusCode::Conflict |
            StatusCode::BadRequest |
            StatusCode::UnprocessableEntity |
            StatusCode::Unauthorized |
            StatusCode::NotFound |
            StatusCode::Forbidden => Err(::Error::from(body)),
            _ => Ok(serde_json::from_str::<T>(&body)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use api::*;
    use api::requester::Requester;
    
    #[test]
    fn build() {
        let client: Client = Client::new(Version::V2);
        let build = client.request::<Build>(Version::V2, "build");
        
        assert!(build.is_ok());
        assert!(*build.unwrap() > 0);
    }
}