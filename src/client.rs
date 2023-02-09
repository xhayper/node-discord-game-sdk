// use crate::user::DiscordUser;
use discord_sdk::user::User;
use discord_sdk::wheel::{UserState, Wheel};
use discord_sdk::{Discord, DiscordApp, Subscriptions};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(js_name = "Client")]
pub struct DiscordClient {
    pub client_id: String,
    internal_client: Discord,
    internal_user: Option<User>,
    internal_wheel: Wheel,
}

#[napi]
impl DiscordClient {
    #[napi(constructor)]
    pub fn new(client_id: String) -> Result<Self> {
        let client_id = match client_id.parse::<i64>() {
            Ok(id) => id,
            Err(err) => panic!("invalid client id: {}", err),
        };

        let (wheel, handler) = Wheel::new(Box::new(|err| {
            println!("error: {:?}", err);
        }));

        let client = Discord::new(
            DiscordApp::PlainId(client_id),
            Subscriptions::ACTIVITY,
            Box::new(handler),
        )
        .expect("unable to create discord client");

        Ok(DiscordClient {
            internal_client: client,
            client_id: client_id.to_string(),
            internal_user: None,
            internal_wheel: wheel,
        })
    }

    // #[napi(getter)]
    // pub fn get_user(&self) -> Result<Option<DiscordUser>> {
    //     if self.internal_user.is_none() {
    //         return Ok(None);
    //     }

    //     Ok(Some(DiscordUser::from_user(
    //         self.internal_user.as_ref().unwrap().clone(),
    //     )))
    // }

    #[napi]
    pub async unsafe fn connect(&mut self) {
        let mut user = self.internal_wheel.user();

        user.0.changed().await.unwrap();

        let user = match &*user.0.borrow() {
            UserState::Connected(user) => user.clone(),
            UserState::Disconnected(err) => panic!("failed to connect to Discord: {}", err),
        };

        self.internal_user = Some(user);
    }
}
