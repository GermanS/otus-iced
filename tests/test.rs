#[cfg(test)]
mod socket_tests {
    use otus_iced::socket::Socket;
    use std::str::FromStr;

    #[test]
    fn positive_f32_in_string() {
        let message = "Socket 21.5W State: off";

        let socket = Socket::from_str(message);

        assert!(socket.is_ok(), "Looks like string has been parsed well");
        assert!(socket.unwrap().power().get() == 21.5, "Power is correct");
    }

    #[test]
    fn positive_u32_in_message() {
        let message = "Socket  1500W State: on";

        let result = Socket::from_str(message);

        assert!(result.is_ok(), "Looks like string has been parsed well");
        assert!(result.unwrap().power().get() == 1500.0, "Power is correct");
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

        assert!(termometer.state().get(), "... state is 'ON'");
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

        assert!(termometer.state().get(), "... state is 'ON'");
    }

    #[test]
    fn negative_missing_temperature() {
        let message = "Termometer xC";

        let termometer = Termometer::from_str(message);

        assert!(!termometer.is_ok(), "Got an error");
    }
}
