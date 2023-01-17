//MIGHT WANT TO SEPERATE PRIVATE AND PUBLIC FOR CLARITY AND NEATNESS
//Make everything return results

use crate::{api::data::*, ui::channels::App};
use std::string::String;
use reqwest;

fn get_length(list: &serde_json::Value) -> usize {
    let the_length = list.as_array();
    match the_length {
        Some(v) => (),
        None => {panic!("TRIED TO GET LENGTH OF AN EMPTY RESPONSE")}
    }
    return the_length.unwrap().len();
}

//Should be removed
//Would be much better to figure out deserialization in structs
fn get(list: &serde_json::Value, index: usize, key: &str) -> anyhow::Result<String, &'static str> {
    //BIG BRAIN DOWN HERE

    let mut value = list[index].get(key);
    match value {
        Some(v) => (),
        None => {value = list.get(key);},
    }

    if value == None {
        return Err("bruh");
    }

    let value = value.unwrap();
    if value.is_number() {
        let value = value.as_i64().unwrap().to_string();
        return Ok(value);
    } else {
        let value = value.as_str().unwrap().to_string();
        return Ok(value);
    }
}

//Get a reqwest and return json
fn request_json(conn: &Connection, url: &str) -> serde_json::Value {
    //request discord data
    let auth = &conn.auth;
    let client = &conn.client;

    let response: serde_json::Value = client.get(url)
        .header(&auth.0, &auth.1)
        .send()
        .expect("Shit out of luck")
        .json()
        .unwrap();

    // dbg!(&response);
    return response;
}

//get guilds
pub fn guilds(conn: &Connection) -> Vec<Guild> {
    //Url changes for every request
    let url = "https://discord.com/api/v9/users/@me/guilds";
    let response = request_json(conn, url);

    let mut server_list = Vec::new();

    let len = get_length(&response);
    for i in 0..len {
        let guild = Guild::from_partial(&response[i]);

        server_list.push(guild);
    }

    return server_list;
}


//MIGHT BREAK IF THERE ARE NO TEXT OR VOICE CHANNELS IN A SERVER
//MIGHT BE SLOW ?
pub fn channels(conn: &Connection, server: &Guild) 
        -> Vec<Channel> {
    let url = format!("https://discord.com/api/v9/guilds/{}/channels", server.id);
    let response = request_json(conn, url.as_str());

    let mut channel_list = Vec::new();

    //0 = guild text channel
    //2 = guild voice channel
    //4 = category

    let len = get_length(&response);
    for i in 0..len {
        let channel = Channel::from(&response[i]);
        
        let guild_vc = String::from("2");
        let category = String::from("4");
        let announcement_thread = String::from("10");
        let public_thread = String::from("11");
        let private_thread = String::from("12");
        let guild_stage_vc = String::from("13");
        let guild_directory = String::from("14");
        let guild_forum = String::from("15");

        let ignored_channels = Vec::from([ 
            guild_vc, category, announcement_thread, public_thread, private_thread,
            guild_stage_vc, guild_directory, guild_forum]);

        if ignored_channels.contains(&channel.channel_type) {
            continue;
        }

        channel_list.push(channel);
    }
    
    return channel_list;
}

pub fn find_channel(channels: &Vec<Channel>, title: &str) -> Result<Channel, &'static str> {
    for channel in channels {
        if channel.name.as_str() == title {
            return Ok(channel.clone());
        }
    }

    return Err("Could not find channel")
}

pub fn messages(conn: &Connection, channel: &Channel) -> anyhow::Result<Vec<Msg>, &'static str> {
    let url = format!("https://discord.com/api/v9/channels/{}/messages?limit=80", channel.id);
    let response = request_json(conn, url.as_str());

    //delete this last get
    let potential_panic = get(&response, 0, "code");

    match potential_panic {
        Ok(v) => return Err("ACCESS DENIED"),
        Err(v) => (),
    }

    let mut message_list = Vec::new();

    let len = get_length(&response);
    for i in 0..len {
        let msg = Msg::from(&response[i]);
        //RETURNS MESSAGES IN REVERSE
        message_list.push(msg);
    }
    message_list.reverse(); //fixes reverse order messages
    return Ok(message_list);
}

pub fn friends(conn: &Connection)  -> Vec<User> {
    let url = "https://discord.com/api/v9/users/@me/relationships";
    let response = request_json(conn, url);

    let mut friends_list = Vec::new();

    let len = get_length(&response);
    for i in 0..len {
        let user = User::from(&response[i]);
        friends_list.push(user);
    }

    return friends_list;
}

pub fn send_message(app: &mut App, input: &String) {
    let channel_id = app.get_channel().id;
    let conn = &app.conn;
    let client = &conn.client;
    let header = conn.auth.clone();

    let params = [("content", input)];

    let url = format!("https://discord.com/api/v9/channels/{}/messages", channel_id);
    let response = client.post(url)
        .header(header.0, header.1)
        .form(&params)
        .send()
        .expect("Failed to send input");
}