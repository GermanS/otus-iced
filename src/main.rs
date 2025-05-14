use iced::{
    Font, Length,
    widget::{Column, Row, Text},
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
    termometer: TermoWidget,
    socket: SocketWidget,
}

#[derive(Default)]
struct TermoWidget {
    state: bool,
    value: f32,
}

impl TermoWidget {
    fn status(&self) -> &str {
        match self.state {
            true => "Online",
            _ => "Offline",
        }
    }
}

#[derive(Default)]
struct SocketWidget {
    state: bool,
    value: f32,
}

impl SocketWidget {
    fn status(&self) -> &str {
        match self.state {
            true => "Online",
            _ => "Offline",
        }
    }
}

impl SmartDeviceApp {
    fn termometer_online(&mut self) {
        self.termometer.state = true;
    }

    fn termometer_offline(&mut self) {
        self.termometer.state = false;
    }

    fn temperature(&mut self, value: f32) {
        self.termometer.value = value;
    }

    fn socket_online(&mut self) {
        self.socket.state = true;
    }

    fn socket_offline(&mut self) {
        self.socket.state = false;
    }

    fn power(&mut self, value: f32) {
        self.socket.value = value;
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TermometerOnline => self.termometer_online(),
            Message::TermometerOffline => self.termometer_offline(),
            Message::TemperatureChanged(value) => self.temperature(value),

            Message::SocketOnline => self.socket_online(),
            Message::SocketOffline => self.socket_offline(),
            Message::PowerChanged(value) => self.power(value),
        }
    }

    fn view(&self) -> Row<Message> {
        let roboto = Font::with_name("Roboto");

        let socket_label = Text::new("Розетка").font(roboto).size(32);

        let socket_state = Text::new(format!("Статуc: {}", self.socket.status()))
            .font(roboto)
            .size(24);

        let socket_display = Text::new(format!("Текущая мощность: {:.2}", self.socket.value))
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

        let termo_state = Text::new(format!("Статуc: {}", self.termometer.status()))
            .font(roboto)
            .size(24);

        let termo_display = Text::new(format!("Текущая температура: {:.1}", self.termometer.value))
            .font(roboto)
            .size(24);

        let termo_widget = Column::new()
            .spacing(10)
            .padding(20)
            .width(Length::Fill)
            .push(termo_label)
            .push(termo_state)
            .push(termo_display);

        let packed = Row::new().push(socket_widget).push(termo_widget);

        packed
    }
}
