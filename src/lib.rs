use dotenv::dotenv;
use failure::Error;
use lazy_static::*;
use log::*;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_xml_rs::*;
use std::env;
use xml::*;

pub mod xml;

lazy_static! {
    static ref HEADER_SOAP: HeaderMap = {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/soap+xml;charset=UTF-8"),
        );
        headers
    };
}

#[derive(Debug)]
pub struct Api {
    client: reqwest::Client,
    url: String,
    username: String,
    security_code_1: String,
    security_code_2: String,
    session_id: Option<String>,
}

impl Api {
    fn new(url: String, username: String, security_code_1: String, security_code_2: String) -> Api {
        Api {
            client: reqwest::Client::new(),
            url,
            username,
            security_code_1,
            security_code_2,
            session_id: None,
        }
    }

    /// Get the setting from a .env file.
    /// - URL
    /// - USERNAME
    /// - SECURITY_CODE_1
    /// - SECURITY_CODE_2
    pub fn new_from_env() -> Result<Api, Error> {
        dotenv().ok();

        if let (Ok(url), Ok(username), Ok(security_code_1), Ok(security_code_2)) = (
            env::var("URL"),
            env::var("USERNAME"),
            env::var("SECURITY_CODE_1"),
            env::var("SECURITY_CODE_2"),
        ) {
            Ok(Api::new(url, username, security_code_1, security_code_2))
        } else {
            Err(failure::format_err!("missing some field in .env"))
        }
    }

    fn update_session_id(&mut self) -> Result<String, Error> {
        let open_session = OpenSession {
            username: Some(self.username.clone()),
            security_code_1: Some(self.security_code_1.clone()),
            security_code_2: Some(self.security_code_2.clone()),
        };

        let session_id = self
            .open_session(open_session)?
            .open_session_result
            .unwrap()
            .session_id
            .unwrap()
            .trim_matches(|c| c == '{' || c == '}')
            .to_string();

        self.session_id = Some(session_id.clone());
        return Ok(session_id);
    }

    pub fn session_id(&mut self) -> Result<String, Error> {
        if let Some(session_id) = &self.session_id {
            Ok(session_id.clone())
        } else {
            self.update_session_id()
        }
    }

    fn add_soap<S: Serialize>(value: &S, wrap: String) -> Result<String, Error> {
        Ok(format!(r#"<soap:Envelope xmlns:soap="http://www.w3.org/2003/05/soap-envelope" xmlns="http://www.e-boekhouden.nl/soap">
<soap:Header/>
<soap:Body><{}>{}</{}></soap:Body>
</soap:Envelope>"#, wrap, to_string(value).unwrap(), wrap))
    }

    fn exchange<'de, I: Serialize, O: Deserialize<'de>, S: Into<String>>(
        &self,
        name: S,
        input: &I,
    ) -> Result<O, Error> {
        let mut res = self
            .client
            .post(&self.url)
            .headers(HEADER_SOAP.clone())
            .body(Self::add_soap(input, name.into())?)
            .send()?
            .text()?;

        if let (Some(l), Some(b)) = (res.find("<soap:Body>"), res.find("</soap:Body>")) {
            let content: String = res.drain((l + 11)..b).collect();;
            println!("content: {}", content);
            return from_str(&content).map_err(|e| failure::format_err!("{:?}", e));
        }
        failure::bail!("Missing body");
    }

    pub fn open_session(
        &mut self,
        open_session: OpenSession,
    ) -> Result<OpenSessionResponse, Error> {
        self.exchange("OpenSession", &open_session)
    }

    pub fn get_relaties(
        &mut self,
        filter: Option<CRelatieFilter>,
    ) -> Result<GetRelatiesResponse, Error> {
        let get_relaties = GetRelaties {
            session_id: Some(self.session_id()?.clone()),
            security_code_2: Some(self.security_code_2.clone()),
            c_filter: filter,
        };

        self.exchange("GetRelaties", &get_relaties)
    }

    pub fn update_relaties(&mut self, relatie: CRelatie) -> Result<UpdateRelatieResponse, Error> {
        let update_relatie = UpdateRelatie {
            session_id: Some(self.session_id()?.clone()),
            security_code_2: Some(self.security_code_2.clone()),
            o_rel: Some(relatie),
        };

        self.exchange("UpdateRelatie", &update_relatie)
    }
}
