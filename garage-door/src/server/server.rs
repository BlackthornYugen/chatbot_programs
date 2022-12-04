use std::time::Duration;

use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use tokio::time;

// Data structure that holds the state of the garage doors.
struct GarageDoors {
    doors: Vec<GarageDoor>,
}

// Data structure that represents a single garage door.
struct GarageDoor {
    id: u64,
    status: DoorStatus,
    timeout: Option<Duration>,
}

// Enumeration that represents the possible statuses of a garage door.
enum DoorStatus {
    Open,
    Closed,
}

impl GarageDoors {
    fn new() -> Self {
        // Initialize the garage doors data structure with some example doors.
        GarageDoors {
            doors: vec![
                GarageDoor { id: 1, status: DoorStatus::Closed, timeout: None },
                GarageDoor { id: 2, status: DoorStatus::Closed, timeout: None },
            ],
        }
    }

    fn get_door(&self, door_id: u64) -> Option<&GarageDoor> {
        // Look up the door with the specified ID.
        self.doors.iter().find(|door| door.id == door_id)
    }

    fn open_door(&mut self, door_id: u64, timeout: Option<Duration>) {
        // Look up the door with the specified ID.
        if let Some(door) = self.doors.iter_mut().find(|door| door.id == door_id) {
            // Update the door's status and timeout.
            door.status = DoorStatus::Open;
            door.timeout = timeout;
        }
    }

    fn close_door(&mut self, door_id: u64) {
        // Look up the door with the specified ID.
        if let Some(door) = self.doors.iter_mut().find(|door| door.id == door_id) {
            // Update the door's status and timeout.
            door.status = DoorStatus::Closed;
            door.timeout = None;
        }
    }

    fn check_timeouts(&mut self) {
        // Check for doors that have timed out and close them.
        for door in self.doors.iter_mut() {
            if let Some(timeout) = door.timeout {
                // Calculate the remaining time for the timeout.
                let remaining_time = timeout.checked_sub(Duration::from_secs(1));

                // If the timeout has elapsed, close the door.
                if remaining_time.map_or(true, |duration| duration.as_secs() == 0) {
                    door.status = DoorStatus::Closed;
                    door.timeout = None;
                } else {
                    door.timeout = remaining_time;
                }
            }
        }
    }
}

// HTTP handler for the GET /api/garage/status/{doorId} endpoint.
async fn get_status(doors: Data<GarageDoors>, door_id: web::Path
