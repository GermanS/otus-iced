use std::{hash::Hash, sync::Arc};

use iced::{
    Font, Length, Subscription, Task,
    advanced::subscription::{EventStream, Hasher, Recipe, from_recipe},
    futures::{
        SinkExt, StreamExt,
        channel::mpsc::{self, Receiver},
        lock::Mutex,
        stream::BoxStream,
    },
    widget::{self, Column, Row, Text},
};
use otus_iced::{socket::Socket, termometer::Termometer};

use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

pub fn main() -> iced::Result {
    iced::application("Устройства", SmartDeviceApp::update, SmartDeviceApp::view)
        .window_size(iced::Size::new(900f32, 225f32))
        .theme(|_| iced::Theme::GruvboxDark)
        .subscription(SmartDeviceApp::subscription)
        .run_with(SmartDeviceApp::new)
}

#[derive(Debug)]
enum Message {
    TermometerOnline(Termometer),
    TermometerOffline,

    SocketOnline(Socket),
    SocketOffline,

    ServerStarted,
}

//#[derive(Default)]
struct SmartDeviceApp {
    termo_widget: TermoWidget,
    socket_widget: SocketWidget,

    net_event_receiver: Arc<Mutex<mpsc::Receiver<SensorData>>>,
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
            true => format!("Текущая температура: {:.1} C", self.value),
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
            true => format!("Текущая мощность: {:.1} Вт", self.value),
            _ => "N/A".into(),
        }
    }
}

enum SensorData {
    SocketIndicator(Socket),
    TermoIndicator(Termometer),
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

    fn new() -> (Self, Task<Message>) {
        let (net_event_sender, net_event_receiver) = mpsc::channel::<SensorData>(32);

        (
            Self {
                termo_widget: TermoWidget::default(),
                socket_widget: SocketWidget::default(),
                net_event_receiver: Arc::new(Mutex::new(net_event_receiver)),
            },
            Task::batch([
                Task::perform(device_server(net_event_sender), |_| Message::ServerStarted),
                widget::focus_next(),
            ]),
        )
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TermometerOnline(t) => self.termometer_online(t),
            Message::TermometerOffline => self.termometer_offline(),

            Message::SocketOnline(s) => self.socket_online(s),
            Message::SocketOffline => self.socket_offline(),
            Message::ServerStarted => {}
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
        let netwrk = from_recipe(NetStream(self.net_event_receiver.clone()));

        Subscription::batch([netwrk])

        //Subscription::run(worker)
    }
}

async fn device_server(net_event_sender: mpsc::Sender<SensorData>) {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    tokio::spawn(async move {
        loop {
            let (tcp, _) = listener.accept().await.unwrap();

            let mut tx_clone = net_event_sender.clone();

            tokio::spawn(async move {
                match handle_connection(tcp).await {
                    Some(SensorData::SocketIndicator(s)) => {
                        let _ = tx_clone.send(SensorData::SocketIndicator(s)).await;
                    }
                    Some(SensorData::TermoIndicator(t)) => {
                        let _ = tx_clone.send(SensorData::TermoIndicator(t)).await;
                    }
                    None => {
                        print!("Nothing is happend");
                    }
                }
            });
        }
    });
}

async fn handle_connection(mut socket: TcpStream) -> Option<SensorData> {
    let mut buf = [0; 128];

    let n = socket.read(&mut buf).await.unwrap();
    let recieved = String::from_utf8_lossy(&buf[..n]);

    if let Ok(t) = recieved.parse::<Termometer>() {
        return Some(SensorData::TermoIndicator(t));
    }

    if let Ok(s) = recieved.parse::<Socket>() {
        return Some(SensorData::SocketIndicator(s));
    }

    None
}

struct NetStream(Arc<Mutex<Receiver<SensorData>>>);

impl Recipe for NetStream {
    type Output = Message;

    fn hash(&self, state: &mut Hasher) {
        std::any::TypeId::of::<Self>().hash(state)
    }

    fn stream(self: Box<Self>, _: EventStream) -> BoxStream<'static, Self::Output> {
        Box::pin(async_stream::stream! {
            let mut receiver = self.0.lock().await;

            while let Some(event) = receiver.next().await {
                match event {
                    SensorData::SocketIndicator(s) => {
                        if s.state().get() {
                            yield Message::SocketOnline(s)
                        } else {
                            yield Message::SocketOffline
                        };
                    }
                    SensorData::TermoIndicator(t) => {
                        if t.state().get() {
                            yield Message::TermometerOnline(t)
                        } else {
                            yield Message::TermometerOffline
                        };
                    }
                }
            }
        })
    }
}
