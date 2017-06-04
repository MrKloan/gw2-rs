//! API `Client` builder.

use std::str::FromStr;
use std::string::ToString;

use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::Client as HttpClient;

use hyper::header::Headers;
use hyper::header::{Connection, UserAgent, AcceptLanguage, qitem};
use hyper::LanguageTag;

use super::{Client, Version, Lang};

pub struct Builder {
    version: Version,
    
    client: HttpClient,
    headers: Headers
}

impl Builder {
    
    pub fn new(version: Version) -> Self {
        Builder {
            version: version,
            
            client: Builder::default_http_client(),
            headers: Builder::default_headers()
        }
    }
    
    fn default_http_client() -> HttpClient {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        
        HttpClient::with_connector(connector)
    }
    
    fn default_headers() -> Headers {
        let mut headers = Headers::new();
        headers.set(Connection::close());
        headers.set(UserAgent(format!("{}/{}", ::PACKAGE, ::VERSION)));
        
        let lang_tag = LanguageTag::from_str(&Lang::EN.to_string()).unwrap();
        headers.set(AcceptLanguage(vec![qitem(lang_tag)]));
        
        headers
    }
    
    pub fn lang(&mut self, lang: Lang) -> &mut Self {
        let lang_tag = LanguageTag::from_str(&lang.to_string()).unwrap();
        self.headers.set(AcceptLanguage(vec![qitem(lang_tag)]));
        
        self
    }
}

impl Into<Client> for Builder {
    fn into(self) -> Client {
        Client::new(self.version, self.client, self.headers)
    }
}