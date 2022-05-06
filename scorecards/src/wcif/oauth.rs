use std::{collections::HashMap, sync::{Arc, Mutex}};

use reqwest::Client;
use serde::Deserialize;
use warp::Filter;

#[derive(Deserialize)]
struct AuthResponse {
    access_token: String,
    refresh_token: String
}

pub struct OAuth {
    access_token: String,
    refresh_token: String,
    client: Client
}

impl OAuth {
    #[tokio::main]
    pub async fn get_auth() -> Self {
        //Url to approve the Oauth application
        let auth_url = "https://www.worldcubeassociation.org/oauth/authorize?client_id=TDg_ARkGANTJB_z0oeUWBVl66a1AYdYAxc-jPJIhSfY&redirect_uri=http%3A%2F%2Flocalhost%3A5000%2F&response_type=code&scope=public+manage_competitions";
     
        //Get authorization code
        let code = get_auth_code(auth_url);

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
    pub async fn get_wcif(self: &Self, id: &str) -> String {
        let get_url = format!("https://www.worldcubeassociation.org/api/v0/competitions/{}/wcif/public", id);
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
            return String::new();
        }

        response
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

pub fn get_auth_code(auth_url: &str) -> String {
    //Mutex for storing the authentification code for async reasons.
    let code: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let code2 = code.clone();

    //Handling the get request from authentification. HTTP no s, super secure, everything is awesome. The API said that https is not required for localhost so it is fine.
    let root = warp::path::end()
        .and(warp::query::query())
        .map(move |s: HashMap<String, String>|{
            let mut guard = code.lock().unwrap();
            s.iter().for_each(|(_,v)|{
                *guard = v.clone();
            });
            format!("Authentification received")
        })
        .with(warp::cors().allow_any_origin());

    //Mutex for knowing when to force open authentification url. Opening before server is listening will break the server for some reason which i do not understand.
    let is_hosting = Arc::new(Mutex::new(false));
    let closure_is_hosting = is_hosting.clone();
    std::thread::spawn(move ||{
        let rt = tokio::runtime::Runtime::new().unwrap();
        let future = async {
            let mut guard = closure_is_hosting.lock().unwrap();
            *guard = true;
            drop(guard);
            warp::serve(root).run(([127, 0, 0, 1], 5000)).await;
        };
        rt.block_on(future);
    });

    //Checking whether code has been received and whether it is time to open authentification url at 120 tps.
    loop {
        std::thread::sleep(std::time::Duration::new(1,0) / 120);
        let guard = code2.lock().unwrap();
        if *guard != "".to_string() {
            return (*guard).clone();
        }
        let mut guard = is_hosting.lock().unwrap();
        if *guard {
            *guard = false;

            //Try opening in browser. In case of fail write the url to the terminal
            match open::that(auth_url) {
                Err(_) => {
                    println!("Please open the following website and follow the instructions:");
                    println!("{}", auth_url);
                }
                Ok(_) => ()
            }
        }
    }
}