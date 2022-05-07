use std::{sync::{Arc, Mutex}, collections::HashMap};
use crate::wcif::{oauth::OAuth, json::parse};
use crate::wcif::*;
use warp::{Filter, hyper::Response};

use self::html::event_list_to_html;

mod html;

pub fn init(id: String) {
    //Url to approve the Oauth application
    let auth_url = "https://www.worldcubeassociation.org/oauth/authorize?client_id=TDg_ARkGANTJB_z0oeUWBVl66a1AYdYAxc-jPJIhSfY&redirect_uri=http%3A%2F%2Flocalhost%3A5000%2F&response_type=code&scope=public+manage_competitions";

    //Mutex for storing the authentification code for async reasons.
    let code: Arc<Mutex<Option<OAuth>>> = Arc::new(Mutex::new(None));
    let wcif: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    let local_wcif = wcif.clone();
    let local_id = id.clone();
    //Handling the get request from authentification. HTTP no s, super secure, everything is awesome. The API said that https is not required for localhost so it is fine.
    let root = warp::path::end()
        .and(warp::query::query())
        .map(move |s: HashMap<String, String>|{
            let code_clone = code.clone();
            let wcif_clone = wcif.clone();
            let id = id.clone();
            std::thread::spawn(move ||{
                s.iter().for_each(|(_,v)|{
                    let oauth = OAuth::get_auth(v.to_string());
                    let json = oauth.get_wcif(id.as_str());
                    let mut wcif_guard = wcif_clone.lock().unwrap();
                    *wcif_guard = json.clone();
                    let mut guard = code_clone.lock().unwrap();
                    *guard = Some(oauth);
                });
            });
            loop {
                std::thread::sleep(std::time::Duration::new(1,0) / 120);
                let wcif_guard = wcif.lock().unwrap();
                if (*wcif_guard).is_some() {
                    let json = (*wcif_guard).clone().unwrap();
                    let wcif = parse(json);
                    let body = format!("{}", event_list_to_html(&local_id.clone(), get_rounds(wcif)));
                    let response = Response::builder()
                        .header("content-type", "text/html")
                        .body(body);
                    return response;
                }
            }
        })
        .with(warp::cors().allow_any_origin());

    let round = warp::path!("round")
        .and(warp::query::query())
        .map(move |s: HashMap<String,String>|{
            let (eventid, round) = s.iter().fold(("", 0),|(e, r), (k, v)|{
                match k.as_str() {
                    "eventid" => (v, r),
                    "round" => (e, usize::from_str_radix(v, 10).unwrap()),
                    _ => panic!("Invalid query")
                }
            });
            let wcif_guard = local_wcif.lock().unwrap();
            let json = (*wcif_guard).clone().unwrap();
            let wcif = parse(json);
            let bytes = crate::pdf::run_from_wcif(wcif, eventid, round, 20);
            
            let response = Response::builder()
                .header("content-disposition", "attachment; filname=\"test.pdf\"")
                .body(bytes);
            return response;
        })
        .with(warp::cors().allow_any_origin());

    let routes = root
        .or(round);
    //Mutex for knowing when to force open authentification url. Opening before server is listening will break the server for some reason which i do not understand.
    let is_hosting = Arc::new(Mutex::new(false));
    let closure_is_hosting = is_hosting.clone();
    std::thread::spawn(move ||{
        let rt = tokio::runtime::Runtime::new().unwrap();
        let future = async {
            let mut guard = closure_is_hosting.lock().unwrap();
            *guard = true;
            drop(guard);
            warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;
        };
        rt.block_on(future);
    });

    //Checking whether code has been received and whether it is time to open authentification url at 120 tps.
    loop {
        std::thread::sleep(std::time::Duration::new(1,0) / 120);
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