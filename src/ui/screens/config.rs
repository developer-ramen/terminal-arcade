//! The screen for viewing and modifying the configuration for Terminal Arcade.

use crossterm::event::{
	Event,
	KeyCode,
	KeyModifiers,
};
use ratatui::{
	layout::{
		Alignment,
		Constraint,
		Direction,
		Layout,
	},
	widgets::{
		Borders,
		Paragraph,
	},
	Frame,
};

use crate::ui::{
	components::{
		presets::{
			titled_ui_block,
			untitled_ui_block,
		},
		under_construction::render_under_construction_block,
	},
	screens::{
		ScreenKind,
		ScreenState,
	},
	Screen,
};

/// See the [module](self) documentation for more information.
#[derive(Default, Clone)]
pub struct ConfigScreen;

impl Screen for ConfigScreen {
	fn initial_state(&self) -> ScreenState {
		ScreenState::new("Under construction!", ScreenKind::Normal, None)
	}

	fn handle_event(&mut self, _event: &Event, _state: &mut ScreenState) -> anyhow::Result<()> {
		Ok(())
	}

	fn render_ui(&self, frame: &mut Frame<'_>, _state: &ScreenState) {
		render_under_construction_block(frame);
	}
}
