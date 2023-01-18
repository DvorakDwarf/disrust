mod api;
mod ui;
use crate::api::data::*; //has all the structs used
use ui::gui;

//REWRITE TO BE LESS SPAGHETTI EVENTUALLY
fn main() {
    println!("Please paste in your token. If you don't know what that is, please google");
    let mut token = String::new();
    std::io::stdin().read_line(&mut token).expect("Could not read input");
    token.pop(); //get rid of \n on the end

    //Simplified way of passing token and client
    let conn = Connection::new(&token); 

    gui::summon_gooey(conn).expect("Could not run the main script. Possibly incorrect token.");
}
