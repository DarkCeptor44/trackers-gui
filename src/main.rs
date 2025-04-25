// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{anyhow, Context, Result};
use iced::{
    widget::{button, column, container, row, text_editor},
    Element,
    Length::Fill,
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

#[derive(Debug, Clone, Copy)]
enum List {
    Best,
    All,
    BestIp,
    AllIp,
}

#[derive(Default)]
struct State {
    content: text_editor::Content,
}

fn main() -> iced::Result {
    iced::run("TrackersGUI", update, view)
}

fn update(state: &mut State, list: List) {
    let text = match list {
        List::Best => get_trackers(URL_BEST).unwrap(),
        List::All => get_trackers(URL_ALL).unwrap(),
        List::BestIp => get_trackers(URL_BEST_IP).unwrap(),
        List::AllIp => get_trackers(URL_ALL_IP).unwrap(),
    };

    state.content = text_editor::Content::with_text(&text);
}

fn view(state: &State) -> Element<List> {
    container(
        column![
            text_editor(&state.content)
                .placeholder("the trackers go here...")
                .height(Fill),
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
