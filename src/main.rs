#![allow(dead_code)] 
use iced;
pub mod types; 
mod view;
mod sudoku;
use crate::types::{Message, GridApp};

fn update( gridApp : &mut GridApp, message: Message) {
    match message {
        Message::Clear =>{
            sudoku::clear(&mut gridApp.grid);
        },
        Message::CellClicked { row, col } => {
            gridApp.selected = Some((row, col));
        },
        Message::Solve =>{
            sudoku::solve(gridApp);
        },
        Message::EventOccurred(event) => {
            if let iced::Event::Keyboard(keyboard_event) = event {
                match keyboard_event {
                    // Match a specific key being pressed down
                    iced::keyboard::Event::KeyPressed { key, modifiers, .. } => {
                        let key_string: String = match key {
                                        // If it's a standard letter, number, or symbol character
                                        iced::keyboard::Key::Character(text) => text.to_string(),
                                        
                                        // If it's a named key (like Escape, Enter, ArrowUp), format its name
                                        iced::keyboard::Key::Named(named_key) => format!("{:?}", named_key),
                                        
                                        _ => String::from("Unknown"),
                                    };
                        if let Ok(num) = key_string.parse::<i32>() {
                            let (row, col) = gridApp.selected.unwrap();
                            gridApp.grid[row][col] = num;
                            
                        }
                        
                    }
                    iced::keyboard::Event::KeyReleased { key, .. } => {
                                           // Do something when a key is let go
                    }
                    // Match when modifiers like Ctrl, Shift, or Alt change
                    iced::keyboard::Event::ModifiersChanged(modifiers) => {
           
                    }
                }
            }
            
        } 
        _ =>{}
        
    }

}

fn subscription(state: &GridApp) -> iced::Subscription<Message> {
        iced::event::listen().map(Message::EventOccurred)
    }


pub fn main() -> iced::Result {
    iced::application(
        GridApp::default, // or pass your initial instance: `grid_app`
        update,
        view::view,
    )
    .subscription(subscription)
    .title("Sudoku")
    .window_size(iced::Size::new(600.0, 600.0))
    .run()
}