use crate::app::hop_net::Net;
use crate::app::hop_net::NetworkCommand;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::time::Duration;

pub fn get_message(
    channel: &Receiver<NetworkCommand>,
    is_stepping: bool,
) -> Option<NetworkCommand> {
    if is_stepping {
        // If the network is stepping, we do not block the thread waiting for new messages
        match channel.try_recv() {
            Ok(mess) => Some(mess),
            Err(err) => match err {
                mpsc::TryRecvError::Empty => Some(NetworkCommand::None),
                mpsc::TryRecvError::Disconnected => None,
            },
        }
    } else {
        // If the network is not stepping, letting the thread run makes no sense, so we wait
        match channel.recv() {
            Ok(mess) => Some(mess),
            Err(_) => None,
        }
    }
}

pub fn handle_message(
    net: &mut dyn Net<f64>,
    command: NetworkCommand,
    is_stepping: &mut bool,
    stepping_speed: &mut Duration,
) {
    match command {
        NetworkCommand::None => {}

        NetworkCommand::Learn(vec) => net.learn(&vec),

        NetworkCommand::Go => {
            *is_stepping = true;
        }

        NetworkCommand::Stop => {
            *is_stepping = false;
        }

        NetworkCommand::SetState(vec) => net.set_state(&vec),

        NetworkCommand::SetSpeed(speed) => {
            *stepping_speed = Duration::from_millis(1000 / speed);
        }

        _ => println!("An unimplemented command was recieved"),
    }
}
