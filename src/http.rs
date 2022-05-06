use reqwest::{blocking, Url};
pub use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::borrow::Cow;

pub trait Query {
    /// Making HTTP request to this endpoint.
    fn endpoint(&self) -> Cow<'static, str>;

    /// Optional. Array of key value pairs.
    fn params(&self) -> QueryParams {
        QueryParams::default()
    }

    /// Optional. Generally a `&'static str` serialized by `serde_json`.
    fn body(&self) -> Result<Option<&'static str>, &'static str> {
        Ok(None)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct QueryParams<'a> {
    pub params: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

pub trait Strategy {
    /** Type of data being returned. */
    type Type;
    /** HTTP method. */
    fn method(&self) -> Method;
    /** Execute the `Strategy`. */
    fn execute(&self) -> Self::Type;
}

#[derive(Debug)]
pub struct Reqwest {
    client: blocking::Client,
    req_builder: Option<blocking::RequestBuilder>,
}

impl Default for Reqwest {
    fn default() -> Self {
        Self {
            client: blocking::Client::new(),
            req_builder: None,
        }
    }
}

impl Reqwest {
    /** Constructor that will parse `endpoint` as `Url` and run `init` for us. */
    pub fn new(method: Method, endpoint: &str) -> Self {
        let mut reqwest = Self {
            client: blocking::Client::new(),
            req_builder: None,
        };
        reqwest.init(method, endpoint);
        reqwest
    }

    /** Initialise a `RequestBuilder` for given `method` & `url`. */
    pub fn init(&mut self, method: Method, endpoint: &str) -> &mut Self {
        self.req_builder =
            Some(self.client.request(method, Self::parse_endpoint(endpoint)));
        self
    }

    /** Add an auth header + bearer `token` to this `Request`. */
    pub fn add_auth(&mut self, auth_token: &str) -> &mut Self {
        self.req_builder = Some(self.get_req_builder().bearer_auth(auth_token));
        self
    }

    /** Add a query, e.g. `?id=123&foo=abc`; `&[("id","123"),("foo", "abc")]` or a struct. */
    pub fn add_query<T: Serialize>(&mut self, query: &T) -> &mut Self {
        self.req_builder = Some(self.get_req_builder().query(query));
        self
    }

    /** Add JSON body to this `Request`. */
    pub fn add_body<T: Serialize>(&mut self, model: T) -> &mut Self {
        let json =
            to_string(&model).expect("failed to serialize model to JSON");
        self.req_builder = Some(self.get_req_builder().body(json));
        self
    }

    /** Send `Request` and return `Response`. */
    pub fn send(&mut self) -> blocking::Response {
        self.get_req_builder()
            .send()
            .expect("failed to make `reqwest`")
    }

    /** Send `Request` then parse and serialize `Response` to T. */
    pub fn data<T: for<'de> Deserialize<'de>>(
        &mut self,
    ) -> Result<T, StatusCode> {
        let resp = self.send();
        let status = resp.status();

        if !status.is_success() {
            return Err(resp.status());
        }

        Ok(resp.json::<T>().expect("failed to serialize `Response`"))
    }

    /** Helper fn to parse `endpoint` from `&str` to `Url`. */
    fn parse_endpoint(endpoint: &str) -> Url {
        Url::parse(endpoint).expect("failed to parse `endpoint`")
    }

    /** Helper fn to take `req_builder` from `Option` container to address borrower concerns. */
    fn get_req_builder(&mut self) -> blocking::RequestBuilder {
        self.req_builder
            .take()
            .expect("uninitialised; no `req_builder` set")
    }
}
