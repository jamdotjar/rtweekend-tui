use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style, Styled, Stylize}, text::{Line, Span, Text}, widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap}, Frame};
use crate::{App, CurrentScreen};




/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

pub fn ui(frame: &mut Frame, app: &App) { 
    //defines the main UI areas, a sidebar with 2 sections, and a main screen with a footer. 
    /* probably look smth like this
     side      main    
    ┌─────┬───────────┐
    │     │           │
    │  0  │           │
    │     │     0     │
    ├─────┤           │
    │     │           │
    │  1  ├───────────┤
    │     │     1     │
    └─────┴───────────┘*/ 
    let outer = Layout::default().direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Min(5),
        ]).split(frame.area());
    let sidebar = Layout::default().direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]).split(outer[0]);
    let main = Layout::default().direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(3),
        ]).split(outer[1]);


    //the info on the sidebar
    let info_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let info_lines = vec![
        Line::styled("rtweekend.rs", Style::default().fg(Color::Red)),
        Line::styled("Here's a little raytracer", Style::default()),
        Line::styled("CONTROLS", Style::default().fg(Color::Yellow).bg(Color::DarkGray)),


    ];
    
    let info = Paragraph::new(Text::from(info_lines)).block(info_block);
    
    //main object list
    let mut objects = Vec::<ListItem>::new();

    for object in app.world.as_simple_vec() {
        objects.push(ListItem::new(
            Line::from(Span::styled(object, Style::default().fg(Color::Yellow)))
        ));
    }

    let object_list = List::new(objects);
    
    //exiting app popup 
    let confirmation_block = Block::default()
        .title("Quit Confirmation")
        .borders(Borders::ALL)
        .border_type(BorderType::Double);
    let confirmation_text = Text::styled(
        "Are you SURE you want to quit? The current scene will not be saved, and all your changes will be lost\n\n Y \\ N", Style::default().fg(Color::Red)
    );
    let confirmation_paragraph = Paragraph::new(confirmation_text).block(confirmation_block).wrap(Wrap { trim: false});
    
    //Editor popup


    frame.render_widget(info, sidebar[0]);
    frame.render_widget(object_list, outer[1]);

    match app.current_screen {
        CurrentScreen::Confirmation => {
            frame.render_widget(Clear, frame.area());
            frame.render_widget(confirmation_paragraph, centered_rect(40, 20, frame.area()))
        }
        CurrentScreen::Editor {
            frame.render_widget()
        }
        _ => {}
    }
}
