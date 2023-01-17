/*
op codes cheat sheet:
0: gateway event
1: heartbeat sent
2: ready event (A load of info like guilds, user, settings, etc)
10: discord sent you heartbeat interval, hello
11: discord received your heartbeat

The gateway events are identified by string names
VC has its own op codes

btw people's email address is public through the api I think, weird
*/

use url::Url;
use tokio_tungstenite::tungstenite::{connect, Message, WebSocket};
use tokio_tungstenite::tungstenite::stream::MaybeTlsStream;
use std::net::TcpStream;
use std::time::Instant;
use serde_json::{self, Value};
use std::thread;
use std::sync::mpsc;

use crate::api::data::*;

pub fn start_thread(token: &String) -> mpsc::Receiver<GatewayResponse> {
    let (tx, rx) = mpsc::channel();

    let gateway_url = "wss://gateway.discord.gg/?v=9&encoding=json";
    let (mut socket, response) = connect(
        Url::parse(gateway_url).unwrap()
    ).expect("Can't connect");

    //Not sure if it's correct terminology
    let handshake = read_json_event(&mut socket).unwrap();
    let hb_interval = handshake["d"]["heartbeat_interval"].as_i64().unwrap();
    println!("Received Hbeat: {}", hb_interval);

    identify(&mut socket, token);

    //Can get a lot of data from it in order to 
    //not update much in network_thread
    let ready = read_json_event(&mut socket).expect("Couldn't get ready event");
    ready_event(&tx, ready);

    thread::spawn(move || {
        let mut timer = Instant::now();
        loop {
            let event = read_json_event(&mut socket);
            // dbg!(&event);
            match &event {
                Ok(v) => (),
                Err(v) => {println!("Gateway disconnected");
                           continue;},
            }
            
            let event = event.unwrap();

            let op_code = event["op"].as_i64().unwrap();
            // dbg!(op_code);
            if op_code == 1 {
                heartbeat(&mut socket);
            }

            //Should put all the events in a list or smthn
            if op_code == 0 {
                let event_name = event["t"].as_str().unwrap();
                match event_name {
                    "MESSAGE_CREATE" => {message_created(&tx, &event);},
                    "MESSAGE_REACTION_ADD" => (),
                    "MESSAGE_REACTION_REMOVE" => (),
                    "TYPING_START" => (),
                    "CHANNEL_CREATE" => (),
                    "GUILD_CREATE" => (),
                    "GUILD_DELETE" => (),
                    _ => ()
                }
            }
            
            //Heartbeat here
            //A thread would have to borrow the socket and it was a pain
            let elapsed = timer.elapsed().as_millis() as i64;
            if hb_interval <= elapsed {
                heartbeat(&mut socket);
                timer = Instant::now();
            }
        }
    });

    return rx;
}

//Each event has an attached sequence number
//Heartbeats need to include latest sequence number
//^^^ Didn't use it in python test and had no problems. Abandoned for now
fn heartbeat(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    let reply = Message::Text(r#"{
        "op": 1,
        "d": "null"
    }"#.into());

    socket.write_message(reply).expect("Hbeat failed");
}

fn identify(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>, token: &str) {
    //ugly as fuck
    let reply = format!("{{
        \"op\": 2,
        \"d\": {{
            \"token\": \"{}\",
            \"properties\": {{
                \"$os\": \"linux\",
                \"$browser\": \"chrome\",
                \"$device\": \"pc\"
            }}
        }}
    }}", token);

    let reply = Message::Text(reply.into());

    socket.write_message(reply).expect("Identification failed");
}

//Makes a Msg object and sends it back to ui thread
fn message_created(tx: &mpsc::Sender<GatewayResponse>, event: &Value) {
    let msg = Msg::from(&event["d"]);
    let gate_response = GatewayResponse::msg_create(msg);
    tx.send(gate_response).unwrap();
}

fn ready_event(tx: &mpsc::Sender<GatewayResponse>, event: Value) {
    let guilds = Guild::from_list(&event["d"]);
    let gate_response = GatewayResponse::ready(guilds);
    tx.send(gate_response).unwrap();
}

// use result instead
// Some weird shit with gateway disconnect idk
fn read_json_event(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) 
        -> Result<serde_json::Value, serde_json::Error> {
    let msg = socket.read_message();
    let msg = msg.expect("Error reading msg");
    let text_msg = msg.to_text().expect("No text, I think");
    let json_msg = serde_json::from_str(text_msg);

    match json_msg {
        Ok(v) => return Ok(v),
        Err(v) => return Err(v)
    }
}