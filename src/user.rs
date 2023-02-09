use discord_sdk::{user::User, Discord};
use napi_derive::napi;

#[napi(js_name = "User")]
pub struct DiscordUser {
    internal_user: Option<User>,
}

#[napi]
impl DiscordUser {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            internal_user: None,
        }
    }

    pub fn from_user(user: User) -> Self {
        Self {
            internal_user: Some(user),
        }
    }

    #[napi(getter)]
    pub fn get_username(&self) -> String {
        if self.internal_user.is_none() {
            return String::new();
        }

        self.internal_user.as_ref().unwrap().username.clone()
    }

    #[napi(getter)]
    pub fn get_discriminator(&self) -> String {
        if self.internal_user.is_none() {
            return String::new();
        }

        self.internal_user
            .as_ref()
            .unwrap()
            .discriminator
            .unwrap()
            .to_string()
    }

    #[napi(getter)]
    pub fn get_id(&self) -> String {
        if self.internal_user.is_none() {
            return String::new();
        }

        self.internal_user.as_ref().unwrap().id.0.to_string()
    }
}
