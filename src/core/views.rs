use crate::core::{
	services, 
	models,
};

use yew::{
	html, 
	Html,
};

use services::{
	Question, 
	Category,
};
use models::App;

pub fn view_question(app: &App, Question{ header, body, popus }: Question) -> Html {
	html! {
		<div class="category min-h-screen lg:container lg:mx-auto lg:max-w-screen-lg sm:w-full">
			<div class="bg-black relative min-h-screen">
				// Component popus
				{ popus }
				<div class="flex items-end absolute w-4 h-full">
					<div class="loading-target w-4 bg-green-500 inset-0 transition-all" style=&format!("height: {}%;", app.state.score.player_loading)></div>
				</div>
				<div class="flex items-end absolute w-4 h-full g-left-end">
					<div class="loading-target w-4 bg-red-500 inset-0 transition-all" style=&format!("height: {}%;", app.state.score.system_loading)></div>
				</div>
				<div class="">
					<div class="header relative">
						// Component header
						{ header }
					</div>
					<div class="body lg:flex flex-wrap">
						// Component body
						{ body }
					</div>
				</div>			
			</div>
		</div>
	}
}

pub fn view_category(app: &App, Category{ menu, sub_menu }: Category) -> Html {
	let (display_menu, display_sub_menu) = if app.view_category && !app.view_sub_category {
		("", "hidden")
	} else {
		("hidden", "")
	};

	html! {
		<div class="category lg:container lg:mx-auto lg:max-w-screen-sm sm:w-full">
			// Menu
			<div class=("bg-yellow-400 flex flex-wrap justify-center lg:space-y-5 sm:space-y-7 sm:text-7xl font-bold", display_menu)>
				{ menu }
			</div>
			// SubMenu
			<div class=("bg-yellow-400 flex flex-wrap justify-center lg:space-y-5 sm:space-y-7 sm:text-7xl font-bold", display_sub_menu)>
				{ sub_menu }
			</div>
		</div>		
	}
}

pub fn view_loading_home() -> Html {
	html! {
		<div>
			<p>{ "Cargando..." }</p>
		</div>
	}
}