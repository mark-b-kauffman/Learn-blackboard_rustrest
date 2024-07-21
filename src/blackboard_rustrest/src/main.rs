pub mod config;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use std::process;
use std::sync::{Arc, Mutex};

// use tokio::time::{sleep, Duration};
// Moved all the urls into the config mod.
// const VERSION_URL: &str = "https://partner-test1.blackboard.com/learn/api/public/v1/system/version";
// const TOKEN_URL: &str = "https://partner-test1.blackboard.com/learn/api/public/v1/oauth2/token";

#[derive(Debug, Serialize, Deserialize)]
struct KeySecret {
    key: String,
    secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

#[derive(Debug, Clone)]
struct TokenManager {
    client: Client,
    key: String,
    secret: String,
    token: Arc<Mutex<Option<(String, u64)>>>,
    token_url: String,
    users_url: String,
}

// BEGIN Define User and Users Structs
#[derive(Debug, Serialize, Deserialize)]
struct Availability {
    available: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Name {
    given: String,
    family: String,
    suffix: Option<String>,
    preferredDisplayName: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Contact {
    email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    city: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Avatar {
    viewUrl: Option<String>,
    source: Option<String>,
}

// Had to make several fields optional with Option so the load from JSON to struct doesn't fail
// when the optional field is empty. Everything needs to be explict with Rust!
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    uuid: String,
    externalId: String,
    dataSourceId: String,
    userName: String,
    studentId: Option<String>,
    educationLevel: Option<String>,
    gender: Option<String>,
    created: String,
    modified: String,
    lastLogin: Option<String>,
    institutionRoleIds: Option<Vec<String>>,
    systemRoleIds: Option<Vec<String>>,
    availability: Option<Availability>,
    name: Option<Name>,
    contact: Option<Contact>,
    address: Option<Address>,
    avatar: Option<Avatar>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UsersResponse {
    results: Vec<User>,
}



// END Define User and Users Structs

impl TokenManager {
    async fn new(key: String, secret: String, token_url: String, users_url:String) -> Self {
        let client = Client::new();
        let token = Arc::new(Mutex::new(None));
        let manager = TokenManager {
            client,
            key,
            secret,
            token: token.clone(),
            token_url,
            users_url,
        };
        manager.update_token().await.unwrap();
        /* tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(3600)).await;
                manager.update_token().await.unwrap();
            }
        }); */
        manager
    }

    async fn update_token(&self) -> Result<(), Box<dyn std::error::Error>> {
        let response = self
            .client
            .post(&self.token_url)
            .basic_auth(&self.key, Some(&self.secret))
            .form(&[("grant_type", "client_credentials")])
            .send()
            .await?
            .json::<TokenResponse>()
            .await?;

        let mut token_guard = self.token.lock().unwrap();
        *token_guard = Some((response.access_token.clone(), response.expires_in));

        Ok(())
    }

    async fn get_token(&self) -> Result<String, Box<dyn std::error::Error>> {
        let token_guard = self.token.lock().unwrap();
        if let Some((token, expires_in)) = &*token_guard {
            if *expires_in > 0 {
                return Ok(token.clone());
            }
        }
        drop(token_guard);
        self.update_token().await?;
        let token_guard = self.token.lock().unwrap();
        Ok(token_guard.as_ref().unwrap().0.clone())
    }

    async fn get_users(&self, token: &str) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get(&self.users_url)
            .bearer_auth(token)
            .send()
            .await?
            .json::<UsersResponse>()
            .await?;
        Ok(response.results)
    }
}

#[tokio::main]
async fn main() {

    let config = config::new(); 

    let key: String = config.key;
    let secret:String = config.secret;
    let token_url: String = config.token_url;
    let users_url: String = config.users_url;

    let manager = TokenManager::new(key, secret, token_url, users_url).await;

    match manager.get_token().await {
        Ok(token) => { 
            println!("Access token: {}", token);
            match manager.clone().get_users(&token).await {
                Ok(users) => {
                    for user in users {
                        println!("User ID: {} User Name: {}", user.id, user.userName);
                        // Print other user fields as necessary
                    }
                }
                Err(err) => eprintln!("Error getting users: {}", err),
            }
        },
        Err(err) => {
            eprintln!("Error getting token: {}", err);
            process::exit(1);
        },
    }
}
