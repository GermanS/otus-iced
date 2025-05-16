use iced::{
    Font, Length, Subscription,
    futures::{SinkExt, Stream, StreamExt, channel::mpsc},
    stream,
    widget::{Column, Row, Text},
};
use otus_iced::{socket::Socket, termometer::Termometer};

pub fn main() -> iced::Result {
    iced::application("Устройства", SmartDeviceApp::update, SmartDeviceApp::view)
        .window_size(iced::Size::new(900f32, 225f32))
        .theme(|_| iced::Theme::GruvboxDark)
        .subscription(SmartDeviceApp::subscription)
        .run()
}

#[derive(Debug)]
enum Message {
    TermometerOnline(Termometer),
    TermometerOffline,

    SocketOnline(Socket),
    SocketOffline,
}

#[derive(Default)]
struct SmartDeviceApp {
    termo_widget: TermoWidget,
    socket_widget: SocketWidget,
}

#[derive(Default)]
struct TermoWidget {
    state: bool,
    value: f32,
}

impl TermoWidget {
    fn status(&self) -> &str {
        match self.state {
            true => "Статуc: Online",
            _ => "Статуc: Offline",
        }
    }

    fn value(&self) -> String {
        match self.state {
            true => format!("Текущая мощность: {:.1}", self.value),
            _ => "N/A".into(),
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

    fn value(&self) -> String {
        match self.state {
            true => format!("Текущая температура: {:.1}", self.value),
            _ => "N/A".into(),
        }
    }
}

impl SmartDeviceApp {
    fn termometer_online(&mut self, t: Termometer) {
        self.termo_widget.state = true;
        self.termo_widget.value = t.temperature().get();
    }

    fn termometer_offline(&mut self) {
        self.termo_widget.state = false;
        self.termo_widget.value = 0.0;
    }

    fn socket_online(&mut self, s: Socket) {
        self.socket_widget.state = true;
        self.socket_widget.value = s.power().get();
    }

    fn socket_offline(&mut self) {
        self.socket_widget.state = false;
        self.socket_widget.value = 0.0;
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TermometerOnline(t) => self.termometer_online(t),
            Message::TermometerOffline => self.termometer_offline(),

            Message::SocketOnline(s) => self.socket_online(s),
            Message::SocketOffline => self.socket_offline(),
        }
    }

    fn view(&self) -> Row<Message> {
        let roboto = Font::with_name("Roboto");

        let socket_label = Text::new("Розетка").font(roboto).size(32);

        let socket_state = Text::new(self.socket_widget.status()).font(roboto).size(24);

        let socket_display = Text::new(self.socket_widget.value()).font(roboto).size(24);

        let socket_widget = Column::new()
            .spacing(12)
            .padding(20)
            .width(Length::Fill)
            .push(socket_label)
            .push(socket_state)
            .push(socket_display);

        let termo_label = Text::new("Термометр").font(roboto).size(32);

        let termo_state = Text::new(self.termo_widget.status()).font(roboto).size(24);

        let termo_display = Text::new(self.termo_widget.value()).font(roboto).size(24);

        let termo_widget = Column::new()
            .spacing(10)
            .padding(20)
            .width(Length::Fill)
            .push(termo_label)
            .push(termo_state)
            .push(termo_display);

        Row::new().push(socket_widget).push(termo_widget)
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(Self::some_worker)
    }

    fn some_worker() -> impl Stream<Item = Message> {
        stream::channel(32, |mut output| async move {
            let (_sender, mut receiver) = mpsc::channel(100);

            loop {
                let input = receiver.select_next_some().await;

                match input {
                    InputData::SocketIndicator(s) => {
                        let message = if s.state().get() {
                            Message::SocketOnline(s)
                        } else {
                            Message::SocketOffline
                        };

                        let _ = output.send(message).await;
                    }
                    InputData::TermoIndicator(t) => {
                        let message = if t.state().get() {
                            Message::TermometerOnline(t)
                        } else {
                            Message::TermometerOffline
                        };

                        let _ = output.send(message).await;
                    }
                }
            }
        })
    }
}

enum InputData {
    SocketIndicator(Socket),
    TermoIndicator(Termometer),
}
