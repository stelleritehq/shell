use hyprland::data::{Windows, Workspaces};
use hyprland::dispatch::{dispatch_blocking, DispatchType};
use hyprland::event_listener::EventListener;
use hyprland::shared::{HyprData, HyprError};
use std::io::Result;

pub struct HyprlandIntegration {
    pub listener: EventListener,
}

impl HyprlandIntegration {
    // connect to the hyprland ipc socket
    pub fn new() -> Result<Self> {
        Ok(Self {
            listener: EventListener::new()
        })
    }

    // requests hyprland compositor, and returns the response
    pub fn send_request(&mut self, request: DispatchType) -> std::result::Result<(), HyprError> {
        dispatch_blocking(request)
    }

    // func to get all workspaces
    pub fn workspaces(&mut self) -> std::result::Result<Workspaces, HyprError> {
        Workspaces::get()
    }

    // func to get all the windows
    pub fn windows(&mut self) -> std::result::Result<Windows, HyprError> {
        Windows::get()
    }

    // func to listen event from the eventstream
    pub fn listen_events(&mut self) -> Result<()> {
        if let Err(e) = self.listener.start_listener() {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("error: [stlr/hyprland] - failed to subscribe to event stream: {:?}", e)
            ))
        } else {
            Ok(())
        }
    }
}
