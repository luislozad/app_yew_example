use std::time::Duration;
use async_std::task;

use crate::core::{
	services, 
	models
};

use services::Msg;
use models::App;
use yew::services::storage::{Area, StorageService};
use wasm_bindgen::JsValue;
use yewtil::future::LinkFuture;



pub fn load_storage() -> StorageService {
	let storage = StorageService::new(Area::Local).expect("storage desactivado por el usuario");
	storage
}

pub async fn load_data() -> Result<services::Data, JsValue> {
	let res = reqwest::Client::new()
        .get("https://raw.githubusercontent.com/luislozad/api/main/data.json")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?;

    let text = res.text().await?;

    Ok(serde_json::from_str(&text).unwrap())
}

pub fn load_api_data(app: &App) {
	if app.state.loading {
		app.link.send_future(async {
			match load_data().await {
				Ok(data) => {
					Msg::Promesa(data)
				},
				Err(_err) => Msg::Not,
			}
		});
	}
}

pub fn wait_for_next<N: Into<Option<u64>> + 'static>(app: &App, r: Msg, t: N) {
	app.link.send_future(
		async {
			task::sleep(Duration::from_secs(t.into().unwrap_or(2))).await;
			r
		}
	);
}

pub fn destroy_timer<T: ?Sized>(v: &mut Option<Box<T>>) {
	*v = None;
}