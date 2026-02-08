#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use iced::widget::{Column, Container, Row, Space, Text};
use iced::{Element, Length, Subscription, Task};
use rcada_core::{tag::Tag, unit::Unit};
use serde::Deserialize;
use std::time::Duration;

const SERVER_URL: &str = "http://127.0.0.1:8080";
const POLLING_RATE: u64 = 200;
const HEALTHCHECK_RATE: u64 = 1000;

#[derive(Debug, Clone, Default)]
pub struct TagDisplay {
    pub name: String,
    pub value: String,
    pub unit: String,
    pub timestamp: String,
    pub data_type: String,
}

impl From<Tag> for TagDisplay {
    fn from(tag: Tag) -> Self {
        let value_str = match tag.value.value {
            rcada_core::value::Value::Float(v) => format!("{:.2}", v),
            rcada_core::value::Value::Integer(v) => v.to_string(),
            rcada_core::value::Value::Boolean(v) => v.to_string(),
            rcada_core::value::Value::String(v) => v,
        };

        let unit_suffix = match tag.meta.unit {
            Unit::None => "",
            Unit::Celsius => "°C",
            Unit::Percent => "%",
            Unit::Ampere => "A",
            Unit::Volt => "V",
            Unit::Degree => "°",
            Unit::Radian => "rad",
            Unit::Kelvin => "K",
            Unit::Metre => "m",
            Unit::Second => "s",
            Unit::Kilogram => "kg",
        };

        let timestamp_str = tag
            .value
            .timestamp
            .map(|t| t.format("%H:%M:%S").to_string())
            .unwrap_or_else(|| "--:----".to_string());

        Self {
            name: tag.name.to_string(),
            value: value_str,
            unit: unit_suffix.to_string(),
            timestamp: timestamp_str,
            data_type: format!("{:?}", tag.meta.data_type),
        }
    }
}

#[derive(Debug, Deserialize)]
struct TagsResponse {
    tags: Vec<Tag>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    HealthCheckServer,
    HealthCheckServerResult(bool),
    Refreshed(Vec<TagDisplay>),
}

#[derive(Debug, Clone, Default)]
struct RcadaClient {
    tags: Vec<TagDisplay>,
    server_url: String,
    server_online: bool,
}

impl RcadaClient {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                tags: Vec::new(),
                server_url: SERVER_URL.to_string(),
                server_online: false,
            },
            Task::none(),
        )
    }
    fn view(&self) -> Element<'_, Message> {
        let title = Text::new("RCADA Tag Viewer").size(28);

        let header = Row::new()
            .spacing(20)
            .push(Text::new("Name").size(14).width(Length::FillPortion(2)))
            .push(Text::new("Value").size(14).width(Length::FillPortion(1)))
            .push(Text::new("Unit").size(14).width(Length::FillPortion(1)))
            .push(Text::new("Time").size(14).width(Length::FillPortion(1)))
            .push(Text::new("Type").size(14).width(Length::FillPortion(1)));

        let rows = self.tags.clone().into_iter().map(|tag| {
            Row::new()
                .spacing(20)
                .push(Text::new(tag.name).size(14).width(Length::FillPortion(2)))
                .push(Text::new(tag.value).size(14).width(Length::FillPortion(1)))
                .push(Text::new(tag.unit).size(14).width(Length::FillPortion(1)))
                .push(
                    Text::new(tag.timestamp)
                        .size(14)
                        .width(Length::FillPortion(1)),
                )
                .push(
                    Text::new(tag.data_type)
                        .size(14)
                        .width(Length::FillPortion(1)),
                )
                .into()
        });

        let lines = Column::with_children(rows);

        let status_bar = Column::with_children([
            Text::new(self.server_url.clone()).into(),
            Text::new(if self.server_online {
                "Online"
            } else {
                "Offline"
            })
            .into(),
        ]);

        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(title)
            .push(header)
            .push(lines)
            .push(Space::new().height(Length::Fill))
            .push(status_bar);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Refresh => {
                if self.server_online {
                    Task::perform(RcadaClient::fetch_tags(), Message::Refreshed)
                } else {
                    Task::none()
                }
            },
            Message::Refreshed(tags) => {
                self.tags = tags;
                Task::none()
            },
            Message::HealthCheckServer => Task::perform(
                RcadaClient::health_check(),
                Message::HealthCheckServerResult,
            ),
            Message::HealthCheckServerResult(status) => {
                self.server_online = status;
                Task::none()
            },
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let poll_tags =
            iced::time::every(Duration::from_millis(POLLING_RATE)).map(|_| Message::Refresh);
        let health_check = iced::time::every(Duration::from_millis(HEALTHCHECK_RATE))
            .map(|_| Message::HealthCheckServer);

        Subscription::batch([poll_tags, health_check])
    }

    async fn health_check() -> bool {
        #[derive(Deserialize)]
        struct HealthCheckResponse {
            status: String,
        }

        let resp = reqwest::get(format!("{}/api/v1/health", SERVER_URL)).await;

        match resp {
            Ok(response) => match response.json::<HealthCheckResponse>().await {
                Ok(HealthCheckResponse {
                    status,
                }) => true,
                Err(_) => false,
            },
            Err(_) => false,
        }
    }

    async fn fetch_tags() -> Vec<TagDisplay> {
        let resp = reqwest::get(format!("{}/api/v1/tags", SERVER_URL)).await;

        match resp {
            Ok(response) => match response.json::<TagsResponse>().await {
                Ok(tags) => tags.tags.into_iter().map(Into::into).collect(),
                Err(e) => {
                    eprintln!("{e:?}");
                    Vec::new()
                },
            },
            Err(e) => {
                eprintln!("{e:?}");
                Vec::new()
            },
        }
    }
}

fn main() -> iced::Result {
    iced::application(RcadaClient::new, RcadaClient::update, RcadaClient::view)
        .subscription(RcadaClient::subscription)
        .run()
}
