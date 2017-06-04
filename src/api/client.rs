//! API client.

use hyper::Client as HttpClient;
use hyper::header::Headers;

use super::Version;
use super::requester::Requester;

pub struct Client {
    version: Version,
    
    client: HttpClient,
    headers: Headers
}

impl Client {
    
    pub fn new(version: Version, client: HttpClient, headers: Headers) -> Self {
        Client {
            version: version,
            
            client: client,
            headers: headers
        }
    }
}

impl Requester for Client {
    fn get_client(&self) -> &HttpClient {
        &self.client
    }
    
    fn get_headers(&self) -> Headers {
        self.headers.clone()
    }
    
    fn get_version(&self) -> Version {
        self.version.clone()
    }
}