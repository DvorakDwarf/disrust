//GUI = gooey
use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, BorderType, Borders, ListItem, List, Paragraph},
    Frame, Terminal,
};
use std::sync::mpsc::Receiver;

use std::time::Duration;
use std::time::Instant;

use crate::ui::chat_box::{InputMode, ChatBox};
use crate::ui::channels::App;
use crate::ui::channels::DisplayMode::{GuildMode, ChannelMode};
use crate::api::data::*;
use crate::api::gateway_thread;

pub fn summon_gooey(conn: Connection) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // conn.auth.1 is the token
    let gate_rx = gateway_thread::start_thread(&conn.auth.1); 
    let guilds = gate_rx.recv().unwrap().guilds;

    let mut app = App::new(guilds, conn);
    let mut cbox = ChatBox::new();
    let response = run_app(&mut terminal, &mut app, &mut cbox, &gate_rx);

    // restore terminal. Closes the program
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = response {
        println!("{:?}", err)
    }

    Ok(())
}

//Main loop
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App, cbox: &mut ChatBox, gate_rx: &Receiver<GatewayResponse>)
         -> io::Result<()> {
            
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    loop {
        match gate_rx.try_recv() {
            Ok(v) => {app.react_to_gateway(&v)},
            Err(_v) => {},
        }

        //Draws the screen. Comment out when debugging
        terminal.draw(|f| ui(f, app, cbox))?;
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        //Read input
        //CodeAesthetic would be upset
        //Have to use poll to avoid blocking
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match cbox.input_mode {
                    InputMode::Normal => {
                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('e') => cbox.toggle(),
                            KeyCode::Left => app.unselect(),
                            KeyCode::Down => app.next(),
                            KeyCode::Up => app.previous(),
                            KeyCode::Enter => app.enter_guild(),
                            KeyCode::Esc => app.leave_guild(),
                            _ => (),
                        }
                    },
                    InputMode::Editing => {
                        match key.code {
                            KeyCode::Enter => cbox.send_message(app),
                            KeyCode::Esc => cbox.toggle(),
                            KeyCode::Char(c) => cbox.input.push(c),
                            KeyCode::Backspace => {cbox.input.pop();},
                            _ => (),
                        }
                    },
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

//Maybe make each block a function
//Sets up how the ui looks like
fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, cbox: &mut ChatBox) {
    //Wrapping block
    //Mandatory margin of 1+
    let wrapping_block = Block::default()
        .borders(Borders::ALL)
        .title("Disrust")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded);
    f.render_widget(wrapping_block, f.size());

    // this is all just defined boundaries used when drawing
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(94), Constraint::Percentage(6)].as_ref())
        .split(chunks[1]);

    // Create the channels part
    let mut items = guilds_to_listitems(&app.guilds.items);
    match app.mode {
        GuildMode => {},
        ChannelMode => {
            items = channels_to_listitems(&app.items.items);
        }
    }

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Guilds and Channels"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // We can now render the item list
    // Displays channels or guilds depending on mode
    match app.mode {
        GuildMode => {
            f.render_stateful_widget(items, chunks[0], &mut app.guilds.state);
        },
        //Weird that let items isn't used, or so vscode thinks
        ChannelMode => {
            f.render_stateful_widget(items, chunks[0], &mut app.items.state);
        }
    }
    
    // Could be better, a lot of cloning
    let title = app.get_current_title();
    let chat_messages = app.get_messages();

    //If there are messages, use those, if there aren't advertise
    match chat_messages {
        Some(v) => {
            let chat_messages = msg_to_list(v, &right_chunks[0]);
            let chat = List::new(chat_messages)
                .block(Block::default().borders(Borders::ALL)
                .title(title));

            f.render_widget(chat, right_chunks[0]);
        },
        None => {
            let ad = vec![ListItem::new("Check my other projects on https://github.com/DvorakDwarf")];
            let chat = List::new(ad)
                .block(Block::default().borders(Borders::ALL)
                .title(title));

            f.render_widget(chat, right_chunks[0]);
        },
    }

    //The chat box is here
    let input = Paragraph::new(cbox.input.as_ref())
        .style(match cbox.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, right_chunks[1]);

    match cbox.input_mode {
        InputMode::Normal => {}, //hides cursor
        InputMode::Editing => {
            //Set cursor as visible and move to right spot
            f.set_cursor(
                // Put cursor past the end of the input text
                right_chunks[1].x + cbox.input.len() as u16 + 1,
                // Move one line down, from the border to the input line
                right_chunks[1].y + 1,
            )
        }
    }
}

// Converts channels into list items that work with the tui-rs widget
fn channels_to_listitems(items: &Vec<Channel>) -> Vec<ListItem> {
    let item_list: Vec<ListItem> = items
    .iter()
    .map(|i| {
        let text = i.name.clone();
        let lines = vec![Spans::from(text)];
        ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
    })
    .collect();

    return item_list;
}

// Converts guilds into list items that work with the tui-rs widget
fn guilds_to_listitems(items: &Vec<Guild>) -> Vec<ListItem> {
    let item_list: Vec<ListItem> = items
    .iter()
    .map(|i| {
        let text = i.name.clone();
        let lines = vec![Spans::from(text)];
        ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
    })
    .collect();

    return item_list;
}

// Converts messages into list items that work with the tui-rs widget
fn msg_to_list(messages: Vec<Msg>, border: &Rect) -> Vec<ListItem> {
    let mut items: Vec<ListItem> = Vec::new();

    for msg in messages {
        let name = &msg.user.name;
        let content = &msg.content;
        let combo = format!("{}: {}", name, content);
        let new_item = ListItem::new(combo);

        items.push(new_item);
    }

    let height = border.height as usize;

    if height > items.len() {
        return items;
    } else {
        //Weird length/index bs. +2 seems to work fine tho
        let slice = items.as_slice()[items.len()-height+3..].to_vec();
        return slice;
    }
}