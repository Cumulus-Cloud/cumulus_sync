use reqwest::header::Authorization;
use reqwest::Client;
use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::prelude::*;

use errors::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
  token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
  pub login: String,
  pub password: String
}

impl Auth {
  pub fn new(login: &str, password: &str) -> Self {
    Auth {
      login: login.to_owned(),
      password: password.to_owned()
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum FsNode {
    FILE {
      id: Uuid,
      path: String,
      name: String,
      hidden: bool,
      creation: NaiveDateTime,
      modification: NaiveDateTime,
      owner: Uuid,
      size: i64,
      humanReadableSize: String,
      hash: String,
      mimeType: String,
      hasThumbnail: bool,
    },
    DIRECTORY {
      id: Uuid,
      path: String,
      name: String,
      hidden: bool,
      creation: NaiveDateTime,
      modification: NaiveDateTime,
      owner: Uuid,
      content: Vec<FsNode>,
    },
}

pub struct CumulusApi {
  client: Client,
  server_url: String,
  token: String,
}

impl CumulusApi {
  pub fn new(server_url: String, token: String) -> Self {
    CumulusApi {
      client: Client::new(),
      server_url,
      token,
    }
  }

  pub fn create(server_url: String, auth: &Auth) -> Result<CumulusApi> {
    let client = Client::new();
    let auth = CumulusApi::authenticate(&client, &server_url, auth)?;
    Ok(CumulusApi::new(server_url, auth.token))
  }

  fn authenticate(client: &Client, server_url: &str, auth: &Auth) -> Result<AuthResponse> {
    let url = format!("{}/{}", server_url, "api/users/login");
    let response = client.post(&url).json(auth).send()?.json()?;
    Ok(response)
  }

  pub fn fs_node(&self, path: &str) -> Result<FsNode> {
    let url = format!("{}/api/fs{}", self.server_url, path);
    let mut response = self.client.get(&url).header(Authorization(self.token.clone())).send()?;
    println!("response {:?}", response.status());
    let jsonr = response.json()?;
    Ok(jsonr)
  }
}
