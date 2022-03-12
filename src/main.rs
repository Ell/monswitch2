#[macro_use]
extern crate lazy_static;

use iced::{Align, Application, Column, Container, Length, Settings, Text};
use monitor::Monitor;

mod monitor;

fn main() -> iced::Result {
    Monswitch::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Error {
    MonitorError,
}

#[derive(Debug)]
enum Monswitch {
    Loading,
    Loaded { monitors: Vec<Monitor> },
    HasError,
}

#[derive(Debug, Clone)]
enum Message {
    MonitorsLoaded(Result<Vec<Monitor>, Error>),
    ReloadMonitors,
}

impl Application for Monswitch {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Monswitch::Loading,
            iced::Command::perform(Monitor::get_monitors(), Message::MonitorsLoaded),
        )
    }

    fn title(&self) -> String {
        "monswitch".to_string()
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        match message {
            Message::MonitorsLoaded(Ok(monitors)) => {
                *self = Monswitch::Loaded { monitors };

                iced::Command::none()
            }
            Message::MonitorsLoaded(Err(_error)) => {
                *self = Monswitch::HasError;

                iced::Command::none()
            }
            Message::ReloadMonitors => match self {
                Monswitch::Loading => iced::Command::none(),
                _ => {
                    *self = Monswitch::Loading;

                    iced::Command::perform(Monitor::get_monitors(), Message::MonitorsLoaded)
                }
            },
        }
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let content = match self {
            Monswitch::Loading => Column::new()
                .width(Length::Shrink)
                .push(Text::new("Gathering Monitor Information").size(40)),
            Monswitch::Loaded { monitors } => {
                let mut column = Column::new().spacing(20).align_items(Align::Start);

                for monitor in monitors {
                    column = column.push(monitor.view());
                }

                column
            }
            Monswitch::HasError => Column::new()
                .width(Length::Shrink)
                .push(Text::new("Error Occured").size(40)),
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
