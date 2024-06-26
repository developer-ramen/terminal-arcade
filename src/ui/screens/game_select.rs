//! A game-selection screen.
//! Users can scroll through the list with arrows to look for a game they want,
//! search a game by its name, or pick a game at random.

use std::cmp::{
	max,
	min,
};

use crossterm::event::{
	Event,
	KeyCode,
	KeyModifiers,
};
use rand::Rng;
use ratatui::{
	layout::{
		Alignment,
		Constraint,
		Direction,
		Layout,
		Rect,
	},
	widgets::Paragraph,
	Frame,
};
use strum::IntoEnumIterator;

use crate::{
	games::{
		Game,
		Games,
	},
	ui::{
		components::{
			game_select::{
				search_bottom_bar::render_search_bottom_bar,
				search_section::render_search_section,
			},
			presets::{
				titled_ui_block,
				untitled_ui_block,
			},
		},
		screens::{
			ScreenKind,
			ScreenState,
		},
		widgets::scrollable_list::ScrollableList,
		Screen,
	},
};

/// Turns a character uppercase.
/// Take care not to use this function beyond normal characters with known
/// uppercase forms like those found in ASCII. If an uppercase character is not
/// found, the lowercase character is returned instead.
fn uppercase_char(c: char) -> char {
	c.to_uppercase().to_string().chars().next().unwrap_or(c)
}

/// The struct for the game selection screen.
#[derive(Clone)]
pub struct GameSearchScreen {
	/// Search term, inputted by the user.
	search_term: Option<String>,

	/// Search results.
	search_results: Vec<Games>,

	/// Scrollable list widget for display.
	game_results_list: ScrollableList<Games>,

	/// Time spent to search and filter the results, in seconds.
	time_to_search_secs: f64,
}

impl Default for GameSearchScreen {
	fn default() -> Self {
		let all_games: Vec<_> = Games::iter().collect();
		Self {
			search_term: None,
			search_results: all_games.clone(),
			game_results_list: ScrollableList::new(
				all_games.into_iter().map(|game| game.data().metadata.get_list_entry()).collect(),
				Some(5),
				3,
				Direction::Vertical,
				Alignment::Center,
				Some((1, 3)),
				None,
			),
			time_to_search_secs: 0.0,
		}
	}
}

impl Screen for GameSearchScreen {
	fn initial_state(&self) -> ScreenState {
		ScreenState::new("Search for a game!", ScreenKind::Normal, None)
	}

	fn handle_event(&mut self, event: &Event, state: &mut ScreenState) -> anyhow::Result<()> {
		if let Event::Key(key) = event {
			match key.code {
				KeyCode::Char('r') if key.modifiers == KeyModifiers::CONTROL => {
					self.game_results_list.scroll_to_random();
				},
				KeyCode::Char('d') if key.modifiers == KeyModifiers::CONTROL => {
					self.clear_search_term();
				},
				KeyCode::Backspace => self.pop_one_character(),
				KeyCode::Char(character)
					if [KeyModifiers::SHIFT, KeyModifiers::NONE].contains(&key.modifiers) =>
				{
					self.add_character_to_term(character, key.modifiers);
				},
				KeyCode::Up => {
					self.game_results_list.scroll_forward();
				},
				KeyCode::Down => {
					self.game_results_list.scroll_backward();
				},
				KeyCode::Left => self.decrease_searches_shown(),
				KeyCode::Right => self.increase_searches_shown(),
				KeyCode::Enter if self.game_results_list.get_selected().is_some() => {
					self.select_game(state);
				},
				_ => {},
			}
		}
		Ok(())
	}

	fn render_ui(&self, frame: &mut Frame<'_>, _state: &ScreenState) {
		let size = frame.size();
		let chunks = Self::game_selection_layout(size).split(size);
		render_search_section(frame, chunks[0], self.search_term.as_deref());
		self.game_results_list.render(frame, chunks[1]);
		render_search_bottom_bar(
			frame,
			chunks[2],
			self.search_results.len(),
			self.time_to_search_secs,
			max(self.game_results_list.get_display_count().unwrap(), 5),
		);
	}
}

impl GameSearchScreen {
	/// Returns the layout for the game selection screen.
	#[must_use]
	fn game_selection_layout(size: Rect) -> Layout {
		let search_section_height = 3;
		let used_ui_height = search_section_height + 3 + 2;
		let search_results_height =
			if used_ui_height >= size.height { 10 } else { size.height - used_ui_height };

		let constraints = vec![
			Constraint::Max(search_section_height), // Search bar/section
			Constraint::Max(search_results_height), // Search results
			Constraint::Max(3),                     // Search bottom info row
			Constraint::Max(0),                     /* Prevents elements from taking all
			                                         * remaining space. */
		];
		Layout::default()
			.direction(Direction::Vertical)
			.vertical_margin(1)
			.horizontal_margin(2)
			.constraints(constraints)
	}

	/// Selects a game.
	fn select_game(&mut self, state: &mut ScreenState) {
		if let Some(selection) = self.game_results_list.get_selected() {
			state.screen_created = selection.1.data.data().created_screen.take();
		}
	}

	/// Updates the search results.
	fn update_search_results(&mut self) {
		let timer = std::time::Instant::now();
		self.search_results = Games::get_by_search_term(&self.search_term);
		self.update_results_list();
		self.time_to_search_secs = timer.elapsed().as_secs_f64();
	}

	/// Updates the [`Self::game_results_list`] property from the
	/// [`Self::search_results`] property.
	fn update_results_list(&mut self) {
		self.game_results_list.update_items(
			self.search_results.iter().map(|game| game.data().metadata.get_list_entry()).collect(),
		);
	}

	/// Adds the character to the search term object, capping out at 256
	/// characters.
	fn add_character_to_term(&mut self, character: char, modifier: KeyModifiers) {
		let character =
			if modifier == KeyModifiers::SHIFT { uppercase_char(character) } else { character };
		match self.search_term {
			None => self.search_term = Some(character.to_string()),
			Some(ref mut term) if term.len() < 100 => term.push(character),
			Some(_) => panic!("Logic went flying all around the plane of existence"),
		}
		self.update_search_results();
	}

	/// Clears the search term.
	fn clear_search_term(&mut self) {
		self.search_term = None;
		self.update_search_results();
	}

	/// Pops one character from the search term, or does nothing if the term is
	/// empty.
	fn pop_one_character(&mut self) {
		if let Some(ref mut term) = self.search_term {
			term.pop();
			if term.is_empty() {
				self.search_term = None;
			}
		}
		self.update_search_results();
	}

	/// Increases the number of shown searches, capping out at 10.
	fn increase_searches_shown(&mut self) {
		let count = self.game_results_list.get_display_count().unwrap();
		if count < min(10, self.search_results.len()) {
			self.game_results_list.set_display_count(count + 1);
		}
	}

	/// Decreases the number of shown searches, capping out at 5.
	fn decrease_searches_shown(&mut self) {
		let count = self.game_results_list.get_display_count().unwrap();
		if count > min(5, self.search_results.len()) {
			self.game_results_list.set_display_count(count - 1);
		}
	}
}
