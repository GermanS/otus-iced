use std::{io::Write, net::TcpStream};

use iced::{
    Background, Border, Color, Font, Shadow, Theme,
    widget::{Button, Column, Text, button::Style, slider},
};
use otus_iced::{state::DeviceState, temperature::Temperature, termometer::Termometer};

pub fn main() -> iced::Result {
    iced::application("Термометер", ThermometerApp::update, ThermometerApp::view)
        .window_size(iced::Size::new(450f32, 225f32))
        .theme(|_| iced::Theme::GruvboxDark)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    TogglePower,
    SliderChanged(f32),
}

#[derive(Default)]
struct ThermometerApp {
    state: bool,
    temperature: f32,
}

impl ThermometerApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::TogglePower => {
                self.state = !self.state;

                if !self.state {
                    self.temperature = 0f32;
                }

                self.notify();
            }
            Message::SliderChanged(value) => {
                if self.state {
                    self.temperature = value;
                }
            }
        }
    }

    fn view(&self) -> Column<Message> {
        let roboto = Font::with_name("Roboto");

        let power_button = Button::new(
            Text::new(if self.state {
                "Включено"
            } else {
                "Выключено"
            })
            .font(roboto)
            .size(20),
        )
        .on_press(Message::TogglePower)
        .padding(12)
        .style(|t: &Theme, _| {
            let palette = t.extended_palette();

            match self.state {
                true => Style {
                    background: Some(Background::Color(palette.primary.base.color)),
                    text_color: Color::WHITE,
                    border: Border::default(),
                    shadow: Shadow::default(),
                },
                false => Style {
                    background: Some(Background::Color(palette.danger.base.color)),
                    text_color: Color::WHITE,
                    border: Border::default(),
                    shadow: Shadow::default(),
                },
            }
        });

        let thermometer_label = Text::new("Термометр").font(roboto).size(32);

        let temperature_slider =
            slider(1.0..=100.0, self.temperature, Message::SliderChanged).step(1.0);

        let temperature_display =
            Text::new(format!("Текущая температура: {:.1}", self.temperature))
                .font(roboto)
                .size(24);

        let content = Column::new()
            .spacing(10)
            .padding(20)
            .push(thermometer_label)
            .push(temperature_display)
            .push(temperature_slider)
            .push(power_button);

        content.into()
    }

    fn notify(&self) {
        let termo = Termometer::new(Temperature::new(self.temperature), DeviceState::new(self.state));

        let mut tcp_stream = TcpStream::connect("localhost:8080").expect("Unable to connect");

        tcp_stream.write_all(termo.to_string().as_bytes()).unwrap();
    }
}
