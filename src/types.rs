use iced::Event;

#[derive(Debug, Clone)]
pub enum Message {
    CellClicked{row: usize, col: usize},
    EventOccurred(Event),
    Clear,
    Solve,
}

#[derive(Debug, Clone)] 
pub struct GridApp {
    pub grid: [[i32; 9]; 9],
    pub selected : Option<(usize, usize)>,
}

impl Default for GridApp {
    fn default() -> Self {
        Self {
            grid: [[0; 9]; 9], // Initialize your 2D array here
            selected: Some((0, 0)),
        }
    }
}