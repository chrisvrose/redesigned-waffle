#[derive(Clone, Debug)]
pub struct AppConfigVariables {
    pub database_url: String,
    pub salt: String,
    pub jwt_secret: String,

    // server stats
    pub address:&'static str,
    pub port: u16
}

impl AppConfigVariables{
    const DEFAULT_ADDRESS: &str = "127.0.0.1";
    const DEFAULT_PORT: u16 = 8080;
    pub fn from_env()->Self{
        let database_url = std::env::var("DATABASE_URL").expect("No Database URL");
        let salt = std::env::var("SALTEDSECRET").expect("No Salted secret");
        let jwt_secret = std::env::var("JWTSECRET").expect("No JWT secret");
        Self {
            address: Self::DEFAULT_ADDRESS,
            port: Self::DEFAULT_PORT,
            database_url,
            salt,
            jwt_secret
        }
    }
}
