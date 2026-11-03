use iced::widget::{button, container, row, text, Grid, column};
use iced::widget::button::Status;
use iced::{alignment, Border, Color, Element, Length, Theme};

use crate::types::{GridApp, Message}; // Adjust this if your types are elsewhere

fn cell_style(is_selected: bool) -> impl Fn(&Theme, Status) -> button::Style {
    move |theme, status| {
        let base = if is_selected {
            button::primary(theme, status)
        } else {
            button::secondary(theme, status)
        };

        button::Style {
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..base
        }
    }
}

pub fn view(gridApp: &GridApp) -> Element<'_, Message> {
    let mut subgrids = Vec::new();

    for box_row in 0..3 {
        for box_col in 0..3 {
            let mut cells = Vec::new();

            for sub_row in 0..3 {
                for sub_col in 0..3 {
                    let row = box_row * 3 + sub_row;
                    let col = box_col * 3 + sub_col;

                    let cell_value = gridApp.grid[row][col];
                    let display_text = if cell_value == 0 {
                        String::new()
                    } else {
                        cell_value.to_string()
                    };

                    let is_selected = gridApp.selected == Some((row, col));

                    let content = text(display_text)
                        .width(Length::Fill)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center)
                        .size(30);

                    let cell = button(content)
                        .width(Length::Fixed(50.0))
                        .height(Length::Fixed(50.0))
                        .on_press(Message::CellClicked { row, col })
                        .style(cell_style(is_selected))
                        .into(); 
                    
                    cells.push(cell);
                }
            }

            let subgrid = Grid::with_children(cells)
                .columns(3)
                .spacing(1) // Thin lines
                .into();
            
            subgrids.push(subgrid);
        }
    }

    let sudoku_board = Grid::with_children(subgrids)
        .columns(3)
        .spacing(3); // Thick lines

    let board_with_lines = container(sudoku_board)
        .padding(3) // Outer border
        .style(|_theme| container::Style {
            background: Some(Color::BLACK.into()),
            ..Default::default()
        });

    row![
        container(
            column![
            button("Solve").on_press(Message::Solve),
            button("Clear").on_press(Message::Clear)
            ].spacing(10)
        ).padding(10),
        container(board_with_lines)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
    ].padding(10).into()
}