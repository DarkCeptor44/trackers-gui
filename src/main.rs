// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{
    Element,
    Length::Fill,
    Theme,
    task::Task,
    widget::{Button, button, column, container, row, text, text_editor},
};
use reqwest::Client;

const URL_BEST: &str =
    "https://raw.githubusercontent.com/ngosang/trackerslist/master/trackers_best.txt";
const URL_ALL: &str =
    "https://raw.githubusercontent.com/ngosang/trackerslist/master/trackers_all.txt";
const URL_BEST_IP: &str =
    "https://raw.githubusercontent.com/ngosang/trackerslist/master/trackers_best_ip.txt";
const URL_ALL_IP: &str =
    "https://raw.githubusercontent.com/ngosang/trackerslist/master/trackers_all_ip.txt";

#[derive(Debug, Default)]
struct State {
    content: text_editor::Content,
    error: Option<String>,
    is_fetching: bool,
}

impl State {
    fn button<'f>(&self, text: &'f str, msg: List) -> Button<'f, List> {
        button(text)
            .on_press_maybe(if self.is_fetching { None } else { Some(msg) })
            .width(Fill)
            .padding(10)
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

#[derive(Debug, Clone)]
enum List {
    Best,
    All,
    BestIp,
    AllIp,

    Action(text_editor::Action),
    Fetched(Result<String, String>),
}

fn main() -> iced::Result {
    iced::application("Trackers", update, view)
        .theme(|_| Theme::TokyoNight)
        .run()
}

fn update(state: &mut State, list: List) -> Task<List> {
    match list {
        List::Best => {
            state.is_fetching = true;
            state.error = None;
            Task::perform(get_trackers(URL_BEST), List::Fetched)
        }
        List::All => {
            state.is_fetching = true;
            state.error = None;
            Task::perform(get_trackers(URL_ALL), List::Fetched)
        }
        List::BestIp => {
            state.is_fetching = true;
            state.error = None;
            Task::perform(get_trackers(URL_BEST_IP), List::Fetched)
        }
        List::AllIp => {
            state.is_fetching = true;
            state.error = None;
            Task::perform(get_trackers(URL_ALL_IP), List::Fetched)
        }
        List::Action(a) => {
            state.perform_action_if(a);
            Task::none()
        }
        List::Fetched(result) => {
            state.is_fetching = false;
            match result {
                Ok(text) => {
                    state.content = text_editor::Content::with_text(&text);
                    state.error = None;
                }
                Err(e) => {
                    state.content = text_editor::Content::with_text("");
                    state.error = Some(e);
                }
            }
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<List> {
    let content_widget = text_editor(&state.content)
        .placeholder(if state.is_fetching {
            "Fetching..."
        } else {
            "the trackers go here..."
        })
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
            text("Trackers:"),
            display_area,
            row![
                state.button("Best", List::Best),
                state.button("All", List::All),
                state.button("Best IP", List::BestIp),
                state.button("All IP", List::AllIp),
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

async fn get_trackers(url: &str) -> Result<String, String> {
    let client = Client::new();
    let resp = client
        .get(url)
        .send()
        .await
        .map_err(|_| format!("Failed to get trackers from `{url}`"))?;

    if !resp.status().is_success() {
        return Err(format!("Failed to get trackers from `{url}`"));
    }

    let text = resp
        .text()
        .await
        .map_err(|_| "Failed to get trackers text")?;

    if text.trim().is_empty() {
        return Err(format!("Tracker list at `{url}` is empty"));
    }

    Ok(text)
}
