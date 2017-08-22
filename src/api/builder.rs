//! API `Client` builder.

use std::str::FromStr;
use std::string::ToString;

use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::Client as HttpClient;

use hyper::header::{
    Headers,
    Connection, UserAgent,
    AcceptLanguage, qitem,
    Authorization, Bearer
};
use hyper::LanguageTag;

use super::prelude::*;

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
        
        Builder::set_lang(&mut headers, Lang::EN);
        
        headers
    }
    
    fn set_lang(headers: &mut Headers, lang: Lang) {
        let lang_tag = LanguageTag::from_str(&lang.to_string()).unwrap();
        headers.set(AcceptLanguage(vec![qitem(lang_tag)]));
    }
    
    /// All of the endpoints which are local aware accept a language parameter.
    ///
    /// Add the `Accept-Language` HTTP header with the value of `<language>`.
    pub fn lang(&mut self, lang: Lang) -> &mut Self {
        Builder::set_lang(&mut self.headers, lang);
        
        self
    }
    
    /// All of the endpoints which fetch account data require the use of an API key.
    ///
    /// Add the `Authorization` HTTP header to your request with the value `Bearer <API key>`.
    pub fn api_key(&mut self, api_key: String) -> &mut Self {
        self.headers.set(Authorization(
            Bearer {
                token: api_key
            }
        ));
        
        self
    }
    
    /// All of the endpoints support paging. The default page size is 50, the maximum page size is
    /// currently 200.
    ///
    /// Add the `X-Page-Size` HTTP header with the value of `<page_size>`.
    pub fn page_size(&mut self, page_size: u8) -> &mut Self {
        let page_size = format!("{}", match page_size > 200 {
            true => page_size,
            false => 200
        });
        self.headers.set_raw("X-Page-Size", vec![page_size.into_bytes()]);
        
        self
    }
    
    /// Specify the total number of generated pages.
    ///
    /// Add the `X-Page-Total` HTTP header with the value of `<page_total>`.
    pub fn page_total(&mut self, page_total: u32) -> &mut Self {
        let page_total = format!("{}", page_total);
        self.headers.set_raw("X-Page-Total", vec![page_total.into_bytes()]);
        
        self
    }
    
    /// Specify the number of resources on the current page (lower or equal to the page size).
    ///
    /// Add the `X-Result-Count` HTTP header with the value of `<result_count>`.
    pub fn result_count(&mut self, result_count: u32) -> &mut Self {
        let result_count = format!("{}", result_count);
        self.headers.set_raw("X-Result-Count", vec![result_count.into_bytes()]);
        
        self
    }
    
    /// Specify the total number of resources.
    ///
    /// Add the `X-Result-Total` HTTP header with the value of `<result_total>`.
    pub fn result_total(&mut self, result_total: u32) -> &mut Self {
        let result_total = format!("{}", result_total);
        self.headers.set_raw("X-Result-Total", vec![result_total.into_bytes()]);
        
        self
    }
}

impl Into<Client> for Builder {
    
    /// Consumes this `Builder` instance in order to create a fully-configured API `Client`.
    fn into(self) -> Client {
        Client::new(self.version, self.client, self.headers)
    }
}