// use yew::services::ConsoleService;

use crate::core::{
	models,
	services,
};

use models::App;
use services::{
	State, 
	Msg,
	Data,
	CategoryType,
	SubCategory,
	UserValue,
	Score,
};

use yew::{html, Html};

const CLASS_BODY: &str = "flex cursor-pointer items-center justify-center mb-6 lg:mb-4 g-w-mid g-w-mid-dk h-52 lg:h-24 px-12 py-12 g-p-14 bg-white text-gray-800 text-5xl lg:text-2xl leading-tight text-center rounded-xl font-bold break-words";

pub fn component_header(app: &App) -> Html {
	let Score{ player, system, .. } = &app.state.score;

	html! {
		<>
		<div class="header__chrono absolute inset-0 w-full h-3 bg-blue-300 bg-opacity-60">
			<div class="loading-main h-3 w-full bg-blue-500 transition-all ease-linear" style=&format!("width: {}%", app.state.timer.micros)></div>
		</div>
		<div class="header__players flex justify-between items-end h-36 lg:h-24 pt-3 px-20">
			<div class="header__left pt-5 h-full flex-shrink-0">
				<div class="profile flex">
					<div class="profile__avatar w-28 h-28 lg:w-14 lg:h-14">
						<img src={ r#"https://avatarfiles.alphacoders.com/106/106923.jpg"# } class="rounded-full border-green-500 border-8 lg:border-4"/>
					</div>
					<div class="profile__info ml-4 lg:ml-3 text-gray-50">
						<div class="profile__name text-3xl lg:text-sm font-medium">
							<span>{ "LuisDev" }</span>
						</div>
						<div class="profile__point text-6xl lg:text-3xl font-bold text-green-500 w-full">
							<span>{ &format!("{}", player) }</span>
						</div>
					</div>
				</div>
			</div>
			<div class="header__middle h-full text-center">
				<div class="time flex h-full flex-wrap pt-6">
					<div class="time__title text-blue-300 uppercase text-xl lg:text-xs w-full">
						<span>{ "Tiempo restante" }</span>
					</div>
					<div class="time__now relative -top-1.5 font-bold text-blue-500 text-6xl lg:text-3xl w-full">
						<span>{ &format!("{}", app.state.timer.secs) }</span>
					</div>
				</div>
			</div>
			<div class="header__righ pt-5 h-full flex-shrink-0">
				<div class="profile flex">
					<div class="profile__info mr-4 lg:mr-3 text-right text-gray-50">
						<div class="profile__name text-3xl lg:text-sm font-medium">
							<span>{ "Jorge Arturo" }</span>
						</div>
						<div class="profile__point text-6xl lg:text-3xl font-bold text-red-500 w-full">
							<span>{ &format!("{}", system) }</span>
						</div>
					</div>
					<div class="profile__avatar w-28 h-28 lg:w-14 lg:h-14">
						<img src={ r#"https://cdn.cp.adobe.io/content/2/rendition/d4eeaea6-cc0b-420e-8553-3b4fb4b64c6f/artwork/f4d6dc7a-44f2-4b43-8086-3689f8cd4518/version/0/format/jpg/dimension/width/size/300"# } class="rounded-full border-red-500 border-8 lg:border-4"/>
					</div>
				</div>
			</div>
		</div>
		</>
	}
}

pub fn component_body(app: &App) -> Html {
	let UserValue{ question, .. } = app.state.user_value.as_ref().unwrap();

	html! {
		<>
		<div class="title text-gray-50 md:text-8xl lg:text-6xl px-20 text-center mb-12 mt-20 lg:mt-10">
			<span>{ &question.title }</span>
		</div>
		<div class="image px-20 lg:pr-4 lg:pl-20 mb-3 lg:w-4/6">
			<img src={ &question.featured_image } class="rounded-2xl"/>
		</div>
		<div class="answers sm:px-20 lg:pl-0 lg:pr-20 lg:w-2/6">
			<ul class="sm:flex flex-wrap justify-between lg:flex-none" ref=app.nodes[1].clone()>
				{ for question.answers.iter().enumerate().map( |(i, val)| component_answer(app, val, i as i8) ) }
			</ul>
		</div>
		</>	
	}
}

fn component_answer(app: &App, answer: &str, indx: i8) -> Html {
	html! {
		<li onclick=app.link.callback( move |_| Msg::SaveItem(indx) ) class=CLASS_BODY>{ answer }</li>
	}
}

pub fn component_popus(app: &App) -> Html {
	let UserValue{ question_total, focus_index, show_popus, category, focus_index_sub, .. } = app.state.user_value.as_ref().unwrap();

	let display_popus = if *show_popus {
		""
	} else {
		"hidden"
	};

	html! {
		<div class=("round-status bg-black z-50 absolute w-full h-full", display_popus) ref=app.nodes[0].clone()>
			<header class="header absolute w-full">
			<div class="header__chrono absolute inset-0 w-full h-3 bg-blue-300 bg-opacity-60">
				<div class="loading-main h-3 w-full bg-blue-500" style=&format!("width: {}%;", app.state.timer.micros)></div>
			</div>
			<div class="header__players flex justify-between items-end h-36 lg:h-24 pt-3 px-20">
				<div class="header__left pt-5 h-full flex-shrink-0">
					<div class="profile flex">
						<div class="profile__avatar w-28 h-28 lg:w-14 lg:h-14">
							<img src="https://avatarfiles.alphacoders.com/106/106923.jpg" class="rounded-full border-green-500 border-8 lg:border-4"/>
						</div>
						<div class="profile__info ml-4 lg:ml-3 text-gray-50">
							<div class="profile__name text-3xl lg:text-sm font-medium">
								// <!-- <span>Maria Juana</span> -->
							</div>
							<div class="profile__point text-6xl lg:text-3xl font-bold text-green-500 w-full">
								// <!-- <span>94</span> -->
							</div>
						</div>
					</div>
				</div>
				<div class="header__middle h-full text-center">
					<div class="time flex h-full flex-wrap pt-6">
						<div class="time__title text-blue-300 uppercase text-xl lg:text-xs w-full">
							// <!-- <span>Tiempo restante</span> -->
						</div>
						<div class="time__now relative -top-1.5 font-bold text-blue-500 text-6xl lg:text-3xl w-full">
							// <!-- <span>9</span> -->
						</div>
					</div>
				</div>
				<div class="header__righ pt-5 h-full flex-shrink-0">
					<div class="profile flex">
						<div class="profile__info mr-4 lg:mr-3 text-right text-gray-50">
							<div class="profile__name text-3xl lg:text-sm font-medium">
								// <!-- <span>Jorge Arturo</span> -->
							</div>
							<div class="profile__point text-6xl lg:text-3xl font-bold text-red-500 w-full">
								// <!-- <span>112</span> -->
							</div>
						</div>
						<div class="profile__avatar w-28 h-28 lg:w-14 lg:h-14">
							<img src="https://cdn.cp.adobe.io/content/2/rendition/d4eeaea6-cc0b-420e-8553-3b4fb4b64c6f/artwork/f4d6dc7a-44f2-4b43-8086-3689f8cd4518/version/0/format/jpg/dimension/width/size/300" class="rounded-full border-red-500 border-8 lg:border-4" />
						</div>
					</div>
				</div>
			</div>
			</header>
			<round class="absolute flex items-center justify-center w-full h-full text-gray-50">
				<div class="round-body flex justify-center flex-wrap">
					<div class="category-image sm:w-96 sm:h-96 lg:w-52 lg:h-52 mb-6">
						<img src="https://preview.redd.it/cx6vk8ezc6761.png?width=960&crop=smart&auto=webp&s=50e8eaa3de0a0c48a174b4e751a8efd605aa0598" class="h-full" />
					</div>
					<div class="category-title sm:text-6xl lg:text-5xl text-center w-full text-gray-200">
						<span>{ &format!("{}", category.sub_category[*focus_index_sub].name) }</span>
					</div>
					<div class="category-subtitle sm:text-3xl lg:text-2xl uppercase font-bold text-center w-full sm:mt-3 lg:mt-2 text-gray-500 ">
						<span>{ &format!("{}", category.name) }</span>
					</div>
					<div class="round-num sm:text-9xl lg:text-8xl uppercase font-bold text-center w-full mt-12">
						<span>{ &format!("Round {}", focus_index + 1) }</span>
					</div>
					<div class="item-total sm:text-5xl lg:text-4xl text-gray-500 font-medium text-center w-full sm:mt-10 lg:mt-5">
						<span>{ &format!("{} of {}", focus_index + 1, question_total) }</span>
					</div>
				</div>
			</round>
		</div>
	}
}

pub fn component_menu(app: &App) -> Html {
	let State{ data, .. } = &app.state;

	let questions = if let Some(Data{ questions, .. }) = data {
		Some(questions)	
	} else {
		None
	};

	html! { for questions.unwrap().iter().enumerate().map( |item| component_menu_item(app, item) ) }

}

fn component_menu_item(app: &App, (i, val): (usize, &CategoryType)) -> Html {
	html! {
		// <!-- Button -->
		<div class="flex w-11/12 bg-white rounded-xl p-7 flex-shrink-0 cursor-pointer" onclick=app.link.callback( move |_| Msg::SubMenu(i) )>
			// <!-- Category icon -->
			<span class="relative w-24 flex-shrink-0">
				<div class="absolute left-0 bottom-0 w-16">
					{ &val.category.icon }
				</div>
			</span>
			// <!-- Category placeholder -->
			<span>
				{ &val.category.name }
			</span> 
			// <!-- Category icon access -->
			<span class="relative w-full">
				<div class="absolute right-0 w-16">
					<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
					  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
					</svg>
				</div>
			</span>
		</div>			
	}
}

pub fn component_sub_menu(app: &App) -> Html {
	let State{ user_value, .. } = &app.state;

	let category = if let Some(UserValue{ category, .. }) = user_value {
		Some(category)
	} else {
		None
	};

	html! {
		{ for category.unwrap().sub_category.iter().enumerate().map( |item| component_sub_menu_item(app, item) ) }
	}
}

fn component_sub_menu_item(app: &App, (i, val): (usize, &SubCategory)) -> Html {
	html! {
		// <!-- Button -->
		<div class="flex w-11/12 bg-white rounded-xl p-7 flex-shrink-0 cursor-pointer" onclick=app.link.callback(move |_| Msg::Question(i))>
			// <!-- Category icon -->
			<span class="relative w-24 flex-shrink-0">
				<div class="absolute left-0 bottom-0 w-16">
					{ &val.icon }
				</div>
			</span>
			// <!-- Category placeholder -->
			<span>
				{ &val.name }
			</span> 
			// <!-- Category icon access -->
			<span class="relative w-full">
				<div class="absolute right-0 w-16">
					<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
					  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
					</svg>
				</div>
			</span>
		</div>		
	}	
}