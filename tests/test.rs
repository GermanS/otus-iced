#[cfg(test)]
mod socket_tests {
    use otus_iced::socket::Socket;
    use std::str::FromStr;

    #[test]
    fn positive_f32_in_string() {
        let message = "Socket 21.5W State: off";

        let result = Socket::from_str(message);
        assert!(result.is_ok(), "Looks like string has been parsed well");

        let socket = result.unwrap();

        assert!(socket.power().get() == 21.5, "Power is correct");
        assert!(!socket.state().get(), "... state is 'off'");
    }

    #[test]
    fn positive_u32_in_message() {
        let message = "Socket  1500W State: on";

        let result = Socket::from_str(message);
        assert!(result.is_ok(), "Looks like string has been parsed well");

        let socket = result.unwrap();

        assert!(socket.power().get() == 1500.0, "Power is correct");
        assert!(socket.state().get(), "... state is 'on'");
    }

    #[test]
    fn negative_missing_temperature() {
        let message = "Socket -x- W";

        let result = Socket::from_str(message);

        assert!(!result.is_ok(), "Got an error");
    }
}

#[cfg(test)]
mod termometer_test {
    use otus_iced::termometer::Termometer;
    use std::str::FromStr;

    #[test]
    fn positive_f32_in_string() {
        let message = "Termometer 21.5C State: on";

        let result = Termometer::from_str(message);

        assert!(result.is_ok(), "Looks like string has been parsed well");

        let termometer = result.unwrap();

        assert!(
            termometer.temperature().get() == 21.5,
            "Temperature is correct"
        );

        assert!(termometer.state().get(), "... state is 'on'");
    }

    #[test]
    fn positive_u32_in_message() {
        let message = "Termometer 21C State: on";

        let result = Termometer::from_str(message);
        assert!(result.is_ok(), "Looks like string has been parsed well");

        let termometer = result.unwrap();

        assert!(
            termometer.temperature().get() == 21.0,
            "Temperature is correct"
        );

        assert!(termometer.state().get(), "... state is 'on'");
    }

    #[test]
    fn negative_missing_temperature() {
        let message = "Termometer xC";

        let termometer = Termometer::from_str(message);

        assert!(!termometer.is_ok(), "Got an error");
    }
}
