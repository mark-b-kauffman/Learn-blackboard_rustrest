// COPY THIS template TO config.rs. 
// Edit the values for your host, and application's key/secret.
// Delete this line and the two above in the config.rs file.
// Usage: pub mod config;
pub struct Config{
    pub hostname: String,
    pub key: String,
    pub secret: String,
    pub token_url: String,
    pub users_url: String,
}

// Define a function to initialize the Config struct
pub fn new() -> Config {
    Config {
        hostname: String::from("FQDN of your Blackboard Learn server."),
        key: String::from("<Your application key here.>"),
        secret: String::from("Your application secret here."),
        token_url: String::from("https://<FQDN of your Learn server>/learn/api/public/v1/oauth2/token"),
        users_url: String::from("https://<FQDN of your Learn server>/learn/api/public/v1/users"),
    }
}

