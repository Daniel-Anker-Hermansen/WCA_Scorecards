use std::collections::HashMap;

use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct AuthResponse {
    access_token: String,
    refresh_token: String
}

#[derive(Debug)]
pub struct OAuth {
    access_token: String,
    refresh_token: String,
    client: Client
}

impl OAuth {
    #[tokio::main]
    pub async fn get_auth(code: String) -> Self {
        let client = reqwest::Client::new();

        let mut params = HashMap::new();

        params.insert("grant_type", "authorization_code");
        params.insert("client_id", "TDg_ARkGANTJB_z0oeUWBVl66a1AYdYAxc-jPJIhSfY");
        params.insert("client_secret", "h0jIi8YkSzJo6U0JRQk-vli21yJ58kuz7_p9-AUyat0");
        params.insert("redirect_uri", "http://localhost:5000/");
        params.insert("code", &code.trim());

        //Request token
        let response = client
            .post("https://www.worldcubeassociation.org/oauth/token")
            .form(&params)
            .send()
            .await.unwrap()
            .text()
            .await.unwrap();

        let auth_response:AuthResponse = serde_json::from_str(&response).unwrap();

        Self {
            access_token: auth_response.access_token,
            refresh_token: auth_response.refresh_token,
            client
        }
    }

    #[tokio::main]
    pub async fn get_wcif(self: &Self, id: &str) -> Option<String> {
        let get_url = format!("https://www.worldcubeassociation.org/api/v0/competitions/{}/wcif", id);
        //Request wcif
        let response = self.client
            .get(&get_url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .send()
            .await.unwrap()
            .text()
            .await.unwrap();
    
        //If response is short, an error has occured and the authentification need to be refreshed
        if response.len() <= 1000 {
            return None;
        }

        Some(response)
    }

    #[tokio::main]
    pub async fn refresh_token(self: &Self) -> Result<(),String> {
        //Correctness is not confirmed
        //I cannot find the github repo for the API again

        let mut params = HashMap::new();

        params.insert("grant_type", "refresh_token");
        params.insert("refresh_token", &self.refresh_token);
        //params.insert("client_secret", "h0jIi8YkSzJo6U0JRQk-vli21yJ58kuz7_p9-AUyat0");
        //params.insert("redirect_uri", "http://localhost:5000/");
        //params.insert("code", &code.trim());

        let response = self.client
            .post("https://www.worldcubeassociation.org/oauth/token")
            .form(&params)
            .send()
            .await.unwrap()
            .text()
            .await.unwrap();

        println!("{}", response);

        Ok(())
    }
}