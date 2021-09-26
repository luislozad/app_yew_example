use yew::services::storage::StorageService;
use yew::services::IntervalService;

use js_sys::Array;
use wasm_bindgen::JsValue;
use std::time::Duration;

use web_sys::Element;

use yew::{
	Component, 
	ComponentLink,
	ShouldRender,
	Html,
	NodeRef,
};

use crate::core::{
	services, 
	controllers, 
	helpers,
	keys,
};

use services::{
	Msg, 
	State,
	Data,
	Timer,
	Score,
	UserValue,
};
use controllers::Get;
use helpers::*;

use keys::{
	TIMER,
	TIMER_LOAD,
	MICRO_SECONS,
	CLASS_PLAYER,
	CLASS_DROP,
	SCORE_MAX,
	LOADING_TOTAL,
};

pub struct App {
	pub link: ComponentLink<Self>,
	pub storage: StorageService,
	pub state: State,
	pub nodes: Vec<NodeRef>,
	pub view_category: bool,
	pub view_sub_category: bool,
	pub view_question: bool,
}

const TIME_LOAD_MICRO: f64 = ((MICRO_SECONS as f64) / 100.0) * (TIMER as f64);

impl Component for App {
	type Message = Msg;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
		Self {
			link,
			storage: load_storage(),
			state: State::default(),
			nodes: vec![NodeRef::default(), NodeRef::default(), NodeRef::default()],
			view_category: false,
			view_question: false,
			view_sub_category: false,
		}
	}

	fn update(&mut self, msg: Self::Message) -> ShouldRender {
		Get::on_update(self, msg);
		true
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		false
	}

	fn view(&self) -> Html {
		if self.view_category {

			Get::quiz_category(self)

		} else if self.view_question {

			Get::quiz_question(self)

		} else {

			Get::quiz_loading(self)

		}
	}
}

impl App {
	pub fn set_state(&mut self, data: Data) {
		self.state.loading = false;
		self.state.error = false;
		self.state.data = Some(data);

		self.state.user_value = self.get_fake_data();

		self.state.timer = Timer {
			secs: TIMER,
			micros: TIMER_LOAD,
			..Default::default()
		};

		self.state.score = Score {
			max: SCORE_MAX,
			..Default::default()
		};
	}

	pub fn set_class_player(&mut self) {
		if let Some(UserValue{ class_list, .. }) = &mut self.state.user_value {
			CLASS_PLAYER.iter().enumerate().for_each(
				|(i, val)| {
					class_list.set(i as u32, JsValue::from_str(val));
				}
			);
		}

	}

	pub fn insert_class_player(&mut self, num_item: i8) {
		if let Some(class) = self.nodes[1].cast::<Element>() {
			if let Some(element) = class.children().get_with_index(num_item as u32) {
				if let Some(UserValue{ class_list, .. }) = &self.state.user_value {
					element.class_list().add(class_list).unwrap();
				}
			}
		}		
	}

	pub fn set_category(&mut self, indx: usize) {
		if let Some(data) = &self.state.data {
			if let Some(UserValue{ category, .. }) = &mut self.state.user_value {
				*category = data.questions[indx].category.clone();
			}
		}
	}	

	pub fn set_question(&mut self, indx: usize) {
		if let Some(UserValue{ question, category, focus_index, focus_index_sub, question_total,  .. }) = &mut self.state.user_value {
			*question = category.sub_category[indx].question[*focus_index].clone();
			*question_total = category.sub_category[indx].question.len();
			*focus_index_sub = indx;
		}

		self.set_max_loading();
	}

	pub fn set_max_loading(&mut self) {
		if let Some(UserValue{ question_total, .. }) = self.state.user_value {
			self.state.score.max_loading = (LOADING_TOTAL / (question_total as f64)).floor() as i32;
		}
	}

	pub fn start_timer(&mut self) {
		self.state.timer.interval_micros = Some(
			Box::new(
				IntervalService::spawn(Duration::from_micros(TIME_LOAD_MICRO as u64), self.link.callback(|_| {
					Msg::MicroTimeout
				}))
			)
		);

		self.state.timer.interval_secs = Some(
			Box::new(
				IntervalService::spawn(Duration::from_secs(1), self.link.callback(|_| {
					Msg::Timeout
				}))
			)
		);
	}

	pub fn show_popus(&mut self) {
		if let Some(UserValue{ show_popus, .. }) = &mut self.state.user_value {
			*show_popus = true;
		}

		if let Some(class) = self.nodes[1].cast::<Element>() {
			for name in CLASS_DROP.iter() {
				for indx in 0..(class.children().length() as usize) {
					if let Some(element) = class.children().get_with_index(indx as u32) {
						element.class_list().remove_1(name).unwrap();
					}
				}
			}
		}	
	}

	pub fn hidden_popus(&mut self) {
		if let Some(UserValue{ show_popus, .. }) = &mut self.state.user_value {
			*show_popus = false;
		}

		self.state.timer.secs = TIMER;
		self.state.timer.micros = TIMER_LOAD;
	}

	pub fn get_score(&mut self, answer: i8) -> i32 {

		let answer_match = if let Some(UserValue{ question, .. }) = &self.state.user_value {
			Some(question.answer)
		} else {
			None
		};

		if answer == answer_match.unwrap() {
			self.state.score.max
		} else {
			0
		}
	}

	pub fn get_score_load(&mut self, score: i32) -> i32 {
		if score > 0 {
			self.state.score.max_loading
		} else {
			0
		}
	}

	pub fn get_fake_data(&mut self) -> Option<UserValue> {
		if let Some(data) = &self.state.data {
			Some(
				UserValue {
					question: data.questions[0].category.sub_category[0].question[0].clone(),
					category: data.questions[0].category.clone(),
					class_list: Array::new_with_length(3),
					focus_index: 0,
					focus_index_sub: 0,
					question_total: 0,
					show_popus: true,
				}
			)
		} else {
			None
		}
	}
}