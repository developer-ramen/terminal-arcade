//! Functions for generating a list of controls shown at the welcome page.
//! In API signatures exposed here, WCL stands for "welcome controls list".

use ansi_to_tui::IntoText;
use crossterm::style::Attribute;
use ratatui::{
	layout::{
		Alignment,
		Constraint,
		Direction,
		Layout,
		Rect,
	},
	style::{
		Color,
		Style,
	},
	text::Text,
	widgets::{
		BorderType,
		Borders,
		Paragraph,
	},
	Frame,
};

use crate::{
	core::terminal::BackendType,
	ui::{
		components::ui_presets::{
			titled_ui_block,
			untitled_ui_block,
		},
		util::{
			ansi_parse_lines,
			stylize_raw,
		},
	},
};

/// Returns a list of lines in the controls list.
#[must_use]
pub fn wcl_lines() -> Vec<String> {
	let reset = Attribute::Reset;
	vec![
		format!(
			"{}: Choose a game to {}! ({}){reset}",
			stylize_raw("[1]"),
			stylize_raw("play"),
			stylize_raw("[P]"),
		),
		format!(
			"{}: View your {} ({})!{reset}",
			stylize_raw("[2]"),
			stylize_raw("configurations"),
			stylize_raw("[C]"),
		),
		format!(
			"{}: {}uit...{reset} ({} works globally!)",
			stylize_raw("[0]"),
			stylize_raw("[Q]"),
			stylize_raw("[Ctrl-C]"),
		),
	]
}

/// ANSI-parses the controls list into a list of [Text]s.
#[must_use]
pub fn wcl_texts() -> Vec<Text<'static>> {
	ansi_parse_lines(wcl_lines())
}

/// Returns a stylized controls list string.
#[must_use]
pub fn wcl_text() -> Text<'static> {
	wcl_lines().join("\n").into_text().unwrap()
}

/// Returns the layout for a controls list block.
#[must_use]
pub fn wcl_layout() -> Layout {
	Layout::default()
		.direction(Direction::Vertical)
		.vertical_margin(1)
		.horizontal_margin(0)
		.constraints(
			[
				Constraint::Max(3),
				Constraint::Max(3),
				Constraint::Max(3),
				Constraint::Max(0),
			]
			.as_ref(),
		)
}

/// Returns a controls list's individual control entry paragraphs.
#[must_use]
pub fn wcl_paragraphs(selected: Option<u64>) -> Vec<Paragraph<'static>> {
	wcl_texts()
		.into_iter()
		.enumerate()
		.map(|(index, text)| {
			let matches = selected.map_or(false, |selected_index| index as u64 == selected_index);
			Paragraph::new(text)
				.block(
					if matches {
						untitled_ui_block()
							.border_type(BorderType::Thick)
							.border_style(Style::default().fg(Color::LightRed))
					} else {
						untitled_ui_block()
					},
				)
				.alignment(Alignment::Center)
		})
		.collect()
}

/// Renders a controls list block.
pub fn render_wcl_block(size: Rect, frame: &mut Frame<'_, BackendType>, selected: Option<u64>) {
	frame.render_widget(titled_ui_block("Controls").borders(Borders::NONE), size);
	let chunks = wcl_layout().split(size);
	let widget_config = wcl_paragraphs(selected).into_iter().zip(chunks.iter());
	for (paragraph, chunk) in widget_config {
		frame.render_widget(paragraph, *chunk);
	}
}
