use reqwest::Client;

pub fn authenticate(client: &Client, server_url: &str, login: &str, password: &str) -> Result<String> {
  let url = format!("{}/{}", server_url, "/api/users/login");
  client.post(&url)
        .body(json!({
          "login": login,
          "password": password
        }))
        .send()?;
}
