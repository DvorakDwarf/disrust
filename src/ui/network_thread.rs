//NOT NEEDED. COULD MAKE USE AGAIN IF I WANTED TO SPEED UP API REQUESTS
//BY MAKING THEM THROUGH A SEPERATE THREAD (WRAPPER IS BLOCKING)
//replaced by gateway

use std::sync::mpsc;
use std::{thread, time};

use crate::api::data::*;
use crate::api::wrapper;

//MAKE STRUCT FOR A NETWORK RESPONSE EVENTUALLY
pub fn start_thread(conn: Connection) -> mpsc::Receiver<Vec<Channel>> {
    let (tx, rx) = mpsc::channel();
    let servers = wrapper::guilds(&conn);

    thread::spawn(move || {
        loop {
            // let channels = wrapper::channels(&conn, &servers[0]);
            // tx.send(channels).unwrap();
            // //TEMPORARY
            // thread::sleep(time::Duration::from_secs(9999));

            // let server = Guild {
            //     id: "990321252671561788".to_string(),
            //     name: "Whatever".to_string(),
            //     channels: Vec::new()
            // };

            let channels = wrapper::channels(&conn, &servers[1]);
            tx.send(channels).unwrap();
            thread::sleep(time::Duration::from_secs(9999));
        }
    });

    return rx;
}