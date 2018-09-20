use reqwest::{Client, Error};

pub mod auth_response;
pub mod fs_node;

use self::auth_response::AuthResponse;
use self::fs_node::FsNode;

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub login: String,
    pub password: String,
}

impl Auth {
    pub fn new<T: Into<String>>(login: T, password: T) -> Self {
        Auth {
            login: login.into(),
            password: password.into(),
        }
    }
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

    pub fn create(server_url: String, auth: &Auth) -> Result<CumulusApi, Error> {
        let client = Client::new();
        let auth = CumulusApi::authenticate(&client, &server_url, auth)?;
        Ok(CumulusApi::new(server_url, auth.token))
    }

    fn authenticate(client: &Client, server_url: &str, auth: &Auth) -> Result<AuthResponse, Error> {
        let url = format!("{}/{}", server_url, "api/users/login");
        let response = client.post(&url).json(auth).send()?.json()?;
        Ok(response)
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
}
