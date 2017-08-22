//! API `Requester` trait.

use std::io::Read;

use serde_json;
use serde::de::DeserializeOwned;

use hyper::Client as HttpClient;

use hyper::status::StatusCode;
use hyper::header::Headers;
use super::version::Version;

/// Generic trait used to query the API and return an instance of a `Deserialize`able struct.
pub trait Requester {
    
    fn request<T>(&self, endpoint: &str) -> ::Result<T> where T: DeserializeOwned {
        let url = format!("{}/{}/{}", ::api::URL, self.get_version(), endpoint);
        let mut response = self.get_client()
            .get(&url)
            .headers(self.get_headers())
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
    
    fn get_client(&self) -> &HttpClient;
    fn get_headers(&self) -> Headers;
    fn get_version(&self) -> Version;
}