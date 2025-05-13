use iced::{
    widget::{row, Column, Row, Text}, Font, Length
};

pub fn main() -> iced::Result {
    iced::application("Устройства", SmartDeviceApp::update, SmartDeviceApp::view)
        .window_size(iced::Size::new(900f32, 225f32))
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
struct SmartDeviceApp {
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

impl SmartDeviceApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::TermometerOnline => {}
            Message::TermometerOffline => {}
            Message::TemperatureChanged(value) => {}

            Message::SocketOnline => {}
            Message::SocketOffline => {}
            Message::PowerChanged(value) => {}
        }
    }

    fn view(&self) -> Row<Message> {
        let roboto = Font::with_name("Roboto");

        let socket_label = Text::new("Розетка").font(roboto).size(32);

        let socket_state = Text::new("Статуc: Offine").font(roboto).size(24);

        let socket_display = Text::new(format!("Текущая мощность: {:.1}", 0))
            .font(roboto)
            .size(24);

        let socket_widget = Column::new()
            .spacing(12)
            .padding(20)
            .width(Length::Fill)
            .push(socket_label)
            .push(socket_state)
            .push(socket_display);

        let termo_label = Text::new("Термометр").font(roboto).size(32);

        let termo_state = Text::new("Статуc: Offine").font(roboto).size(24);

        let termo_display = Text::new(format!("Текущая температура: {:.1}", 0))
            .font(roboto)
            .size(24);

        let termo_widget = Column::new()
            .spacing(10)
            .padding(20)
            .width(Length::Fill)
            .push(termo_label)
            .push(termo_state)
            .push(termo_display);

        let packed = Row::new()
            .push(socket_widget)
            .push(termo_widget);

        packed
    }
}
