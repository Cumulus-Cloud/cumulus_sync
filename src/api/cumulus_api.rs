use std::fs::File;
use reqwest::{Client, Error};

use super::fs_node::FsNode;

#[derive(Debug, Serialize, Deserialize)]
struct AuthResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Auth {
    login: String,
    password: String,
}

pub struct UnautenticatedApi {
    client: Client,
    server_url: String,
}

impl UnautenticatedApi {
    pub fn authenticate<S: Into<String>>(self, login: S, password: S) -> Result<CumulusApi, Error> {
        let url = format!("{}/{}", self.server_url, "api/users/login");
        let auth = Auth {
            login: login.into(),
            password: password.into(),
        };
        let response: AuthResponse = self.client.post(&url).json(&auth).send()?.json()?;
        Ok(CumulusApi {
            client: self.client,
            server_url: self.server_url,
            token: response.token,
        })
    }
}

pub struct CumulusApi {
    client: Client,
    server_url: String,
    token: String,
}

impl CumulusApi {
    pub fn new<S: Into<String>>(client: Client, server_url: S) -> UnautenticatedApi {
        UnautenticatedApi {
            client,
            server_url: server_url.into(),
        }
    }

    pub fn fs_node(&self, path: &str) -> Result<FsNode, Error> {
        let url = format!("{}/api/fs{}", self.server_url, path);
        let mut response = self
            .client
            .get(&url)
            .header("authorization", self.token.clone())
            .send()?;
        println!("response {:?}", response.status());
        let jsonr = response.json()?;
        Ok(jsonr)
    }

    pub fn upload(&self, path: &str, file: File) -> Result<FsNode, Error> {
        let url = format!("{}/api/upload{}", self.server_url, path);
        let mut response = self
            .client
            .post(&url)
            .body(file)
            .header("authorization", self.token.clone())
            .send()?;
        info!("upload {:?} {:?}", response.status(), url);
        let jsonr = response.json()?;
        Ok(jsonr)
    }
}
