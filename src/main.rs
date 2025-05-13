use iced::{widget::{button::Style, slider, Button, Column, Text}, Background, Border, Color, Font, Shadow, Theme};

pub fn main() -> iced::Result {
    iced::application("Термометер", SocketApp::update, SocketApp::view)
        .window_size(iced::Size::new(450f32, 225f32))
        .theme(|_| iced::Theme::GruvboxDark)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    TermometerOnline,
    TermometerOffline,
    TemperatureChanged(f32),

    SocketOnline,
    SocketOffline,
    PowerChanged(f32),
}

#[derive(Default)]
struct SocketApp {
    power: TermoWidget,
    socket: SocketWidget,
}

#[derive(Default)]
struct TermoWidget {
    state: bool,
    value: f32,
}

#[derive(Default)]
struct SocketWidget {
    state: bool,
    value: f32,
}

impl SocketApp {

    fn update(&mut self, message: Message) {
        match message {
            Message::TermometerOnline => {},
            Message::TermometerOffline => {},
            Message::TemperatureChanged(value) => {},

            Message::SocketOnline => {},
            Message::SocketOffline => {},
            Message::PowerChanged(value) => {},
        }
    }

    fn view(&self) -> Column<Message> {
         let roboto = Font::with_name("Roboto");

        let power_button = Button::new(
            Text::new(if self.power_on { "Включено" } else { "Выключено" })
                .font(roboto)
                .size(20)

        )
        .on_press(Message::TogglePower)
        .padding(12)
        .style(|t: &Theme, _| {
            let palette = t.extended_palette();

            match self.power_on {
                true => Style {
                    background: Some(Background::Color(palette.primary.base.color)),
                    text_color: Color::WHITE,
                    border: Border::default(),
                    shadow: Shadow::default(),
                },
                false => Style {
                    background : Some(Background::Color(palette.danger.base.color)),
                    text_color: Color::WHITE,
                    border: Border::default(),
                    shadow: Shadow::default(),
                }
            }

        });

        let power_label = Text::new("Розетка")
            .font(roboto)
            .size(32);

        let power_slider = slider(1.0..=100.0, self.power, Message::SliderChanged)
            .step(1.0);

        let power_display = Text::new(format!("Текущая мощность: {:.1}", self.power))
            .font(roboto)
            .size(24);

        let content = Column::new()
            .spacing(10)
            .padding(20)
            .push(power_label)
            .push( power_display)
            .push(power_slider)
            .push(power_button);

        content.into()
    }
}