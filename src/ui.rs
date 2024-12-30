#![allow(unused_imports)]
use std::default;

use crate::{App, CurrentScreen, CurrentlyEditing, MaterialType};
use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Styled, Stylize},
    symbols::line::TOP_RIGHT,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use rtwlib::material::Material;

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
    let outer = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Min(5)])
        .split(frame.area());
    let sidebar = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer[0]);
    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(outer[1]);

    //the info on the sidebar
    let info_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .style(Style::default());

    let mut info_lines = vec![
        Line::styled("rtweekend.rs", Style::default().fg(Color::Red)),
        Line::styled("Here's a little raytracer", Style::default()),
        Line::styled(
            "CONTROLS",
            Style::default().fg(Color::Yellow).bg(Color::DarkGray),
        ),
    ];

    match app.current_screen {
        CurrentScreen::Main => {
            info_lines.push(Line::styled("Main Page:", Style::default().fg(Color::Blue)));
            info_lines.push(Line::styled("  [N]: Create new object", Style::default()));
            info_lines.push(Line::styled("  [R]: Render the scene", Style::default()));
            info_lines.push(Line::styled("  [Q]: Quit", Style::default()));
        }
        CurrentScreen::Editor => {
            info_lines.push(Line::styled("Editor:", Style::default().fg(Color::Green)));
            info_lines.push(Line::styled(
                "  TAB: Cycle through inputs",
                Style::default(),
            ));
            info_lines.push(Line::styled("  Type to input", Style::default()));
            info_lines.push(Line::styled("  Enter: Confirm", Style::default()));
            info_lines.push(Line::styled("  Esc: Cancel", Style::default()));
        }
        CurrentScreen::MaterialPicker => {
            info_lines.push(Line::styled(
                "Material Pickers:",
                Style::default().fg(Color::Green),
            ));
            info_lines.push(Line::styled("  Arrow Keys: Navigate", Style::default()));
            info_lines.push(Line::styled("  Enter: Save", Style::default()));
            info_lines.push(Line::styled(
                "  [N]: Create a new material",
                Style::default(),
            ));
        }
        CurrentScreen::MaterialEditor => {
            info_lines.push(Line::styled(
                "Material Editor:",
                Style::default().fg(Color::Green),
            ));
            info_lines.push(Line::styled(
                "  TAB: Cycle through inputs",
                Style::default(),
            ));
            info_lines.push(Line::styled("  Type to input", Style::default()));
            info_lines.push(Line::styled("  Enter: Save", Style::default()));
            info_lines.push(Line::styled("  Esc: Cancel", Style::default()));
        }
        _ => {}
    }

    let info = Paragraph::new(Text::from(info_lines)).block(info_block);

    //main object list
    let mut objects = Vec::<ListItem>::new();

    for object in app.world.as_simple_vec() {
        objects.push(ListItem::new(Line::from(Span::styled(
            object,
            Style::default().fg(Color::Yellow),
        ))));
    }
    let object_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let object_list = List::new(objects).block(object_block);

    //stats/info bar

    let stats_block = Block::default()
        .borders(Borders::union(Borders::RIGHT, Borders::BOTTOM))
        .border_type(BorderType::Thick)
        .title("Stats")
        .style(Style::default());

    let stats_lines = vec![Line::styled(
        format!("Objects in Scene: {}", app.world.objects.len()),
        Style::default().fg(Color::Green),
    )];

    let stats = Paragraph::new(Text::from(stats_lines)).block(stats_block);
    //exiting app popup
    let confirmation_block = Block::default()
        .title("Quit")
        .title_bottom("[Y / N]")
        .title_alignment(Alignment::Center)
        .title_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Double);
    let confirmation_text = Text::styled(
        "Are you SURE you want to quit? The current scene will not be saved, and all your changes will be lost", Style::default().fg(Color::LightRed)
    );
    let confirmation_paragraph = Paragraph::new(confirmation_text)
        .block(confirmation_block)
        .wrap(Wrap { trim: false });

    frame.render_widget(info, outer[0]);
    frame.render_widget(stats, main[1]);
    frame.render_widget(object_list, main[0]);

    match app.current_screen {
        CurrentScreen::Confirmation => {
            frame.render_widget(Clear, frame.area());
            frame.render_widget(confirmation_paragraph, centered_rect(40, 20, frame.area()))
        }
        CurrentScreen::Editor => editor(frame, app),
        CurrentScreen::MaterialPicker => material_picker(frame, app),
        CurrentScreen::MaterialEditor => material_editor(frame, app),
        _ => {}
    }
}

fn editor(frame: &mut Frame, app: &App) {
    //Editor popup
    let editor_block = Block::default()
        .title("Create a new object")
        .title_style(Style::default().fg(Color::Red))
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black))
        .border_type(BorderType::Rounded);
    let editor_area = centered_rect(45, 20, frame.area());
    let editor_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .spacing(2)
        .constraints([
            Constraint::Min(6),
            Constraint::Min(6),
            Constraint::Min(6),
            Constraint::Min(6),
            Constraint::Min(16),
        ])
        .split(editor_area);

    let mut bl_radius = Block::default()
        .title("Radius")
        .borders(Borders::NONE)
        .bg(Color::DarkGray);
    let mut bl_posx = Block::default()
        .title("X")
        .borders(Borders::NONE)
        .bg(Color::DarkGray);
    let mut bl_posy = Block::default()
        .title("Y")
        .borders(Borders::NONE)
        .bg(Color::DarkGray);
    let mut bl_posz = Block::default()
        .title("Z")
        .borders(Borders::NONE)
        .bg(Color::DarkGray);
    let mut bl_mat = Block::default()
        .title("Material Index")
        .borders(Borders::NONE)
        .bg(Color::DarkGray);

    let selected_style = Style::default().bg(Color::White).fg(Color::Black);

    if let Some(editing) = &app.current_edit {
        match editing {
            CurrentlyEditing::Size => bl_radius = bl_radius.style(selected_style),
            CurrentlyEditing::PositionX => bl_posx = bl_posx.style(selected_style),
            CurrentlyEditing::PositionY => bl_posy = bl_posy.style(selected_style),
            CurrentlyEditing::PositionZ => bl_posz = bl_posz.style(selected_style),
            CurrentlyEditing::Material => bl_mat = bl_mat.style(selected_style),
            _ => {}
        }
    }

    let txt_size = Paragraph::new(app.size_input.clone()).block(bl_radius);
    let txt_posx = Paragraph::new(app.position_input_x.clone()).block(bl_posx);
    let txt_posy = Paragraph::new(app.position_input_y.clone()).block(bl_posy);
    let txt_posz = Paragraph::new(app.position_input_z.clone()).block(bl_posz);
    let txt_mat = Paragraph::new(format!(
        "{} (hit any key to edit)",
        app.material_input.to_string()
    ))
    .block(bl_mat);

    frame.render_widget(Clear, editor_area);
    frame.render_widget(editor_block, editor_area);

    frame.render_widget(txt_size, editor_chunks[0]);
    frame.render_widget(txt_posx, editor_chunks[1]);
    frame.render_widget(txt_posy, editor_chunks[2]);
    frame.render_widget(txt_posz, editor_chunks[3]);
    frame.render_widget(txt_mat, editor_chunks[4]);
}

fn material_picker(frame: &mut Frame, app: &App) {
    let picker_block = Block::default()
        .title("Choose a material")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .style(Style::default())
        .border_type(BorderType::Rounded);
    let picker_area = centered_rect(30, 30, frame.area());

    let material_names: Vec<ListItem> = app
        .materials
        .iter()
        .map(|(name, _)| ListItem::new(name.as_str()))
        .collect();

    let material_list = List::new(material_names)
        .block(picker_block)
        .highlight_style(Style::default().bg(Color::White).fg(Color::Black));

    let mut state = ListState::default();
    state.select(Some(app.material_input));

    frame.render_stateful_widget(material_list, picker_area, &mut state);
}

fn material_editor(frame: &mut Frame, app: &App) {
    let editor_block = Block::default()
        .title("Create a new material")
        .borders(Borders::ALL)
        .style(Style::default())
        .border_type(BorderType::Rounded);
    let editor_area = centered_rect(50, 25, frame.area());
    let editor_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .spacing(2)
        .constraints([
            Constraint::Min(7), //Type
            Constraint::Min(7), //Color
            Constraint::Min(5), //Other
        ])
        .split(editor_area);
    let mut bl_type = Block::default()
        .title("Type")
        .borders(Borders::NONE)
        .bg(Color::DarkGray);
    let mut bl_color = Block::default()
        .title("Color")
        .borders(Borders::NONE)
        .bg(Color::Rgb(
            (app.get_color().x * 255.0) as u8,
            (app.get_color().y * 255.0) as u8,
            (app.get_color().z * 255.0) as u8,
        ));
    let mut bl_other = Block::default()
        .title(match app.mat_type_input {
            Some(MaterialType::Metal) => "Roughness",
            Some(MaterialType::Dielectric) => "IOR",
            _ => "ERROR",
        })
        .borders(Borders::NONE)
        .bg(Color::DarkGray);
    let selected_style = Style::default().bg(Color::White).fg(Color::Black);

    if let Some(editing) = &app.current_edit {
        match editing {
            CurrentlyEditing::MatType => bl_type = bl_type.style(selected_style),
            CurrentlyEditing::MatColor => bl_color = bl_color.style(selected_style),
            CurrentlyEditing::MatProperty => bl_other = bl_other.style(selected_style),
            _ => {}
        }
    }

    let txt_type = Paragraph::new(app.mat_type_input.clone().unwrap().to_string());
    let txt_color = Paragraph::new(app.mat_color_input.clone()).block(bl_color);
    let txt_other = Paragraph::new(app.mat_other_input.clone()).block(bl_other);

    frame.render_widget(Clear, editor_area);
    frame.render_widget(editor_block, editor_area);

    frame.render_widget(txt_type, editor_chunks[0]);
    frame.render_widget(txt_color, editor_chunks[1]);
    match app.mat_type_input {
        Some(MaterialType::Metal) => {
            frame.render_widget(txt_other, editor_chunks[2]);
        }
        Some(MaterialType::Dielectric) => {
            frame.render_widget(txt_other, editor_chunks[2]);
        }
        _ => {}
    }
}
