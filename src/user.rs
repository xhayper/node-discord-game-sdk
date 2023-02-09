// use discord_sdk::user::User;
// use napi::bindgen_prelude::Reference;
// use napi_derive::napi;

// #[napi(js_name = "User")]
// pub struct DiscordUser {
//     internal_user: User,
// }

// #[napi]
// impl DiscordUser {
//     pub fn with_user(user: User) -> Self {
//         Self {
//             internal_user: user,
//         }
//     }

//     #[napi(getter)]
//     pub fn get_id(&self) -> String {
//         self.internal_user.id.0.to_string()
//     }
// }
