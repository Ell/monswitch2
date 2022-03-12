use anyhow::Result;
use iced::{Align, Column, Element, Image, Length, Row, Text};

use crate::{Error, Message};

lazy_static! {
    static ref DEFAULT_IMAGE_HANDLE: iced::image::Handle =
        iced::image::Handle::from_memory(include_bytes!("default.png").to_vec());
}

#[derive(Debug, Clone)]
pub struct Monitor {
    id: String,
    model_name: Option<String>,
}

impl Monitor {
    pub(crate) async fn get_monitors() -> Result<Vec<Monitor>, Error> {
        let monitors: Vec<_> = ddc_hi::Display::enumerate()
            .into_iter()
            .map(|display| {
                let model_name = display.info.model_name.clone();
                let id = display.info.id.clone();

                Monitor { id, model_name }
            })
            .collect();

        Ok(monitors)
    }

    pub(crate) fn view(&mut self) -> Element<Message> {
        let model_name = match &self.model_name {
            Some(model_name) => model_name.clone(),
            None => self.id.clone(),
        };

        Row::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(Monitor::fetch_image(&model_name))
            .push(
                Column::new()
                    .align_items(Align::Start)
                    .width(Length::Fill)
                    .spacing(5)
                    .push(Text::new(model_name).size(30).width(Length::Fill))
                    .push(Text::new(self.id.clone()).size(20).width(Length::Fill)),
            )
            .into()
    }

    fn get_display(&mut self) -> Option<&ddc_hi::Display> {
        None
    }

    fn fetch_image(_model_name: &str) -> Image {
        Image::new(DEFAULT_IMAGE_HANDLE.clone())
    }
}
