use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
struct AuthResponse {
    access_token: String
}

#[tokio::main]
pub async fn get_wcif(id: &str) -> String {
    //Url to aprove the Oauth application
    let auth_url = "https://www.worldcubeassociation.org/oauth/authorize?client_id=6kju4KoHbrSCJBHnN_rS63l4Mk0gs0pv5XIMRijw0qI&redirect_uri=urn%3Aietf%3Awg%3Aoauth%3A2.0%3Aoob&response_type=code&scope=public+manage_competitions";

    //Try opening in browser. In case of fail write the url to the terminal
    match open::that(auth_url) {
        Err(_) => {
            println!("Please open the following website and follow the instructions:");
            println!("{}", auth_url);
        }
        Ok(_) => ()
    }

    //Get authorization code
    println!("Please enter the authorization code:");
    let mut code = String::new();
    std::io::stdin().read_line(&mut code).unwrap();

    let client = reqwest::Client::new();

    let mut params = HashMap::new();

    params.insert("grant_type", "authorization_code");
    params.insert("client_id", "6kju4KoHbrSCJBHnN_rS63l4Mk0gs0pv5XIMRijw0qI");
    params.insert("client_secret", "aNrc-pa-sPE_d4eH0zItVoEHfv7hoGy26ZnzrKpxd-g");
    params.insert("redirect_uri", "urn:ietf:wg:oauth:2.0:oob");
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

    let get_url = format!("https://www.worldcubeassociation.org/api/v0/competitions/{}/wcif", id);

    //Request wcif
    let response = client
        .get("https://www.worldcubeassociation.org/api/v0/competitions/dastrupsleepover2022/wcif")
        .header("Authorization", format!("Bearer {}", auth_response.access_token))
        .send()
        .await.unwrap()
        .text()
        .await.unwrap();

    response
}
