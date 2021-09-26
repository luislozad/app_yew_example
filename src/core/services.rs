use serde::{Deserialize, Serialize};
use yew::Html;
// use std::collections::HashMap;
use yew::services::Task;
use js_sys::Array;
// use std::rc::Rc;

pub enum Msg {
	Promesa(Data),
	Not,
	NextItem,
	NextPlayer,
	UpdateItem,
	SaveItem(i8),
	Timeout,
	MicroTimeout,
	SubMenu(usize),
	Question(usize),
}

pub struct UserValue {
	pub question: QuestionItem,
	pub category: CategoryItem,
	pub class_list: Array,
	pub focus_index: usize,
	pub focus_index_sub: usize,
	pub question_total: usize,
	pub show_popus: bool,
}

#[derive(Default)]
pub struct Score {
	pub player: i32,
	pub system: i32,
	pub max: i32,
	pub max_loading: i32,
	pub system_loading: i32,
	pub player_loading: i32,
}

pub struct Question {
	pub popus: Html,
	pub header: Html,
	pub body: Html,
}

pub struct Category {
	pub menu: Html,
	pub sub_menu: Html,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
	pub questions: Vec<CategoryType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryType {
	pub category: CategoryItem,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryItem {
	pub name: String,
	pub icon: String,
	pub sub_category: Vec<SubCategory>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubCategory {
	pub name: String,
	pub icon: String,
	pub question: Vec<QuestionItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuestionItem {
	pub title: String,
	pub featured_image: String,
	pub answers: [String; 4],
	pub answer: i8,
}

impl Default for QuestionItem {
	fn default() -> Self {
		QuestionItem {
			title: "Este cangrejo real se encuentra en Sudamerica.".into(),
			featured_image: "https://static.inaturalist.org/photos/5804961/large.jpeg".into(),
			answers: ["Centrolla".into(), "Cangrejo Dungeness".into(), "Cangrejo corredor".into(), "Cangrejo de pinzas rojas".into()],
			answer: 2,
		}
	}
}

#[derive(Default)]
pub struct Timer {
	pub interval_secs: Option<Box<dyn Task>>,
	pub interval_micros: Option<Box<dyn Task>>,
	pub secs: i32,
	pub micros: i32,
}

pub struct State {
	pub loading: bool,
	pub error: bool,
	pub data: Option<Data>,
	pub timer: Timer,
	pub score: Score,
	pub user_value: Option<UserValue>,
}

impl Default for State {
	fn default() -> State {
		State {
			loading: true,
			error: false,
			data: None,
			timer: Timer::default(),
			score: Score::default(),
			user_value: None,
		}
	}
}