// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Context, Result, anyhow};
use iced::{
    Element,
    Length::Fill,
    widget::{button, column, container, row, text, text_editor},
};
use reqwest::blocking::Client;

const URL_BEST: &str =
    "https://raw.githubusercontent.com/ngosang/trackerslist/master/trackers_best.txt";
const URL_ALL: &str =
    "https://raw.githubusercontent.com/ngosang/trackerslist/master/trackers_all.txt";
const URL_BEST_IP: &str =
    "https://raw.githubusercontent.com/ngosang/trackerslist/master/trackers_best_ip.txt";
const URL_ALL_IP: &str =
    "https://raw.githubusercontent.com/ngosang/trackerslist/master/trackers_all_ip.txt";

#[derive(Debug, Clone)]
enum List {
    Best,
    All,
    BestIp,
    AllIp,

    Action(text_editor::Action),
}

#[derive(Default)]
struct State {
    content: text_editor::Content,
    error: Option<String>,
}

impl State {
    fn handle_list_press(&mut self, list_url: &str) {
        match get_trackers(list_url) {
            Ok(text) => {
                self.content = text_editor::Content::with_text(&text);
                self.error = None;
            }
            Err(e) => {
                self.content = text_editor::Content::with_text("");
                self.error = Some(e.to_string());
            }
        }
    }

    fn perform_action_if(&mut self, action: text_editor::Action) {
        match action {
            text_editor::Action::SelectAll
            | text_editor::Action::Scroll { lines: _ }
            | text_editor::Action::Click(_) => self.content.perform(action),
            _ => (),
        }
    }
}

fn main() -> iced::Result {
    iced::run("TrackersGUI", update, view)
}

fn update(state: &mut State, list: List) {
    match list {
        List::Best => state.handle_list_press(URL_BEST),
        List::All => state.handle_list_press(URL_ALL),
        List::BestIp => state.handle_list_press(URL_BEST_IP),
        List::AllIp => state.handle_list_press(URL_ALL_IP),

        List::Action(a) => state.perform_action_if(a),
    }
}

fn view(state: &State) -> Element<List> {
    let content_widget = text_editor(&state.content)
        .placeholder("the trackers go here...")
        .on_action(List::Action)
        .height(Fill);
    let display_area = if let Some(error) = &state.error {
        column![
            content_widget,
            text(format!("Error: {error}")).color(iced::Color::new(1.0, 0.0, 0.0, 1.0))
        ]
        .spacing(10)
    } else {
        column![content_widget].spacing(10)
    };

    container(
        column![
            display_area,
            row![
                button("Best").on_press(List::Best).width(Fill).padding(10),
                button("All").on_press(List::All).width(Fill).padding(10),
                button("Best IP")
                    .on_press(List::BestIp)
                    .width(Fill)
                    .padding(10),
                button("All IP")
                    .on_press(List::AllIp)
                    .width(Fill)
                    .padding(10),
            ]
            .spacing(10)
            .width(Fill)
        ]
        .spacing(10),
    )
    .padding(10)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}

fn get_trackers(url: &str) -> Result<String> {
    let client = Client::new();
    let resp = client.get(url).send().context("Failed to get trackers")?;

    if !resp.status().is_success() {
        return Err(anyhow!("Failed to get trackers from `{url}`"));
    }

    let text = resp.text().context("Failed to get trackers text")?;

    if text.trim().is_empty() {
        return Err(anyhow!("Tracker list at `{url}` is empty"));
    }

    Ok(text)
}
