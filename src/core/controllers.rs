use yew::Html;
// use yew::services::ConsoleService;

use web_sys::Element;

// use std::collections::HashMap;

use js_sys::Math;

use crate::core::{
	services,
	models,
	views,
	components,
	helpers,
};

use services::{
	Msg, 
	Question,
	Category,
	UserValue,
};
use models::App;
use views::{
	view_question, 
	view_category,
	view_loading_home,
};
use components::{
	component_header, 
	component_body,
	component_popus,
	component_menu,
	component_sub_menu,
};
use helpers::{
	wait_for_next,
	destroy_timer,
	load_api_data,
};

pub struct Get;

impl Get {
	pub fn quiz_question(app: &App) -> Html {
		let q = Question {
			popus: component_popus(app),
			header: component_header(app),
			body: component_body(app),
		};

		view_question(app, q)
	}

	pub fn quiz_category(app: &App) -> Html {
		let c = Category {
			menu: component_menu(app),
			sub_menu: component_sub_menu(app),
		};

		view_category(app, c)
	}

	pub fn quiz_loading(app: &App) -> Html {
		load_api_data(app);

		view_loading_home()
	}
}

// Events
impl Get {
	pub fn on_update(app: &mut App, msg: Msg) {
		match msg {
			Msg::Promesa(data) => {
				// Inicializa el estado de la aplicacion
				app.set_state(data);
				// Establece las clases que usara posteriormente el jugador (API JS)
				app.set_class_player();

				app.view_question = false;
				app.view_sub_category = false;
				app.view_category = true;
			},
			Msg::Not => {
				app.state.loading = false;
				app.state.error = true;
			},
			Msg::SaveItem(num_item) => {
				// Inserta las clases respectivas al jugador cuando este selecciono su respuesta
				app.insert_class_player(num_item);
				// 
				let score = app.get_score(num_item);
				app.state.score.player += score;
				// 
				app.state.score.player_loading += app.get_score_load(score);
				// Destruye el tiempo del primer cronometro
				destroy_timer(&mut app.state.timer.interval_micros);
				// Destruye el tiempo del segundo cronometro
				destroy_timer(&mut app.state.timer.interval_secs);
				// Espera 2 segundos antes que el sistema pueda elejir su propia respuesta
				wait_for_next(app, Msg::NextPlayer, None);
			},
			Msg::UpdateItem => {
				// Muestra las estadistica del jugador en un popus
				app.show_popus();
				// Incremente el indice para la siguiente pregunta
				if let Some(UserValue{ ref mut focus_index, question_total, focus_index_sub, .. }) = app.state.user_value {
					if (*focus_index + 1) < question_total {
						*focus_index += 1;
					}

					let index_sub = focus_index_sub;

					app.set_question(index_sub);
				}
				// Espera 2 segundos antes de mostrar la siguiente pregunta al jugador
				wait_for_next(app, Msg::NextItem, 2);
			},
			Msg::NextPlayer => {
				if let Some(class) = app.nodes[1].cast::<Element>() {
					let rand = Math::floor(Math::random() * (3.0 - 0.0) + 0.0);
					let indx = rand as u32;
					let add_class: &str = "bg-red-500";

					let score = app.get_score(indx as i8);
					app.state.score.system += score;
					app.state.score.system_loading += app.get_score_load(score);

					if let Some(element) = class.children().get_with_index(indx) {
						if element.class_list().contains("bg-green-500") {
							element.class_list().add_1("g-text-gray-50").unwrap();
							element.class_list().add_1("g-icon-selection-right").unwrap();
						} else {
							element.class_list().add_1(add_class).unwrap();
							element.class_list().add_1("g-text-gray-50").unwrap();
							element.class_list().add_1("g-icon-selection-right").unwrap();
						}
					}

					// Espera 2 segundos antes de mostrar las estadisticas popus
					wait_for_next(app, Msg::UpdateItem, 2);
				}
			},
			Msg::NextItem => {
				// Oculta el popus de las estadisticas
				app.hidden_popus();
				// Inicializa el cronometro de tiempo
				app.start_timer();
			},
			Msg::Timeout => {
				let pre_timer = app.state.timer.secs - 1;
				
				if pre_timer >= 0 {
					app.state.timer.secs -= 1;
				} else {
					destroy_timer(&mut app.state.timer.interval_secs);
				}

			},
			Msg::MicroTimeout => {

				let pre_timer = app.state.timer.micros - 1;

				if pre_timer >= 0 {
					app.state.timer.micros -= 1;
				} else {
					destroy_timer(&mut app.state.timer.interval_micros);
				}
			},
			Msg::SubMenu(i) => {
				app.set_category(i);
				app.view_question = false;
				app.view_category = true;
				app.view_sub_category = true;	
			},
			Msg::Question(i) => {
				// Guarada las preguntas que se usaran despues en la vista
				app.set_question(i);
				app.view_category = false;
				app.view_sub_category = false;
				app.view_question = true;
				// Espera 2 segundos antes de mostrar la siguiente pregunta al jugador
				wait_for_next(app, Msg::NextItem, 2);
			}
		}
	}	
}
