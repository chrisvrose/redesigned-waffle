use argon2::Config;

pub fn get_config<'a>()->Config<'a>{
    return Config::default();
}