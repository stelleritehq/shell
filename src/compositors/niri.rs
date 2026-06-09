use niri_ipc::{Event, Request, Response};
use niri_ipc::socket::Socket;
use std::io::Result;

pub struct NiriIntegration {
    socket: Socket,
}

impl NiriIntegration {
    // connect to the niri ipc socket
    pub fn new() -> Result<Self> {
        let socket = Socket::connect()?;
        Ok(Self { socket })
    }

    // requests niri compositor, and returns the response
    pub fn send_request(&mut self, request: Request) -> Result<std::result::Result<Response, String>> {
        self.socket.send(request)
    }

    // func to get all workspaces
    pub fn workspaces(&mut self) -> Result<std::result::Result<Response, String>> {
        self.send_request(Request::Workspaces)
    }

    // func to get all the windows
    pub fn windows(&mut self) -> Result<std::result::Result<Response, String>> {
        self.send_request(Request::Windows)
    }

    // func to listen event from the eventstream
    pub fn listen_events<F>(&mut self, mut callback: F) -> Result<()>
    where
        F: FnMut(Event),
    {
        let reply = self.send_request(Request::EventStream)?;

        if matches!(reply, Ok(Response::Handled)) {
            let mut read_event = self.socket.read_events();

            while let Ok(event) = read_event() {
                callback(event);
            }
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("error: [stlr/niri] - failed to subscribe to event stream: {:?}", reply)
            ))
        }
    }
}
