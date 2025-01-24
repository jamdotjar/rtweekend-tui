#![allow(unused_imports)]
use std::default;

use crate::{
    render::{self, render_view},
    render_preview, App, CurrentScreen, CurrentlyEditing, MaterialType,
};
use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Styled, Stylize},
    symbols::line::TOP_RIGHT,
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Cell, Clear, List, ListItem, ListState, Padding, Paragraph,
        Row, Scrollbar, ScrollbarState, Table, TableState, Wrap,
    },
    Frame,
};
use rtwlib::material::Material;

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Min(4),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Min(10),
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
        Line::styled("╦═╗╔╦╗╦ ╦ ┬─┐┌─┐", Style::default().fg(Color::Red)),
        Line::styled("╠╦╝ ║ ║║║ ├┬┘└─┐", Style::default().fg(Color::Red)),
        Line::styled("╩╚═ ╩ ╚╩╝°┴└─└─┘", Style::default().fg(Color::Red)),
        Line::styled("Raytracing in rust", Style::default().fg(Color::Green)),
        Line::styled(
            "CONTROLS",
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    match app.current_screen {
        CurrentScreen::Main => {
            info_lines.push(Line::styled(
                "Main Page:",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ));
            info_lines.push(Line::styled(
                "  ↑ & ↓: Scroll object list",
                Style::default(),
            ));
            info_lines.push(Line::styled(
                "  [D]: Delete selected object",
                Style::default(),
            ));
            info_lines.push(Line::styled("  [N]: Create a new object", Style::default()));
            info_lines.push(Line::styled(
                "  [M]: Create a new material",
                Style::default(),
            ));
            info_lines.push(Line::styled(
                "  [P]: View a preview render (ESC to close)",
                Style::default(),
            ));

            info_lines.push(Line::styled("  [R]: Render the scene", Style::default()));
            info_lines.push(Line::styled("  [Q]: Quit", Style::default()));
        }
        CurrentScreen::Editor => {
            info_lines.push(Line::styled("Editor:", Style::default().fg(Color::Green)));
            info_lines.push(Line::styled(
                "  Tab & Shift+Tab: Change inputs",
                Style::default(),
            ));
            info_lines.push(Line::styled("  ← & →: Change inputs", Style::default()));
            info_lines.push(Line::styled("  Type to input", Style::default()));
            info_lines.push(Line::styled("  ↑ & ↓: Choose Material", Style::default()));
            info_lines.push(Line::styled("  Enter: Save", Style::default()));
            info_lines.push(Line::styled("  Esc: Cancel", Style::default()));
        }
        CurrentScreen::MaterialEditor => {
            info_lines.push(Line::styled(
                "Material Editor:",
                Style::default().fg(Color::Green),
            ));
            info_lines.push(Line::styled(
                "  Tab & Shift+Tab: Change inputs",
                Style::default(),
            ));
            info_lines.push(Line::styled("  ← & →: Change inputs", Style::default()));
            info_lines.push(Line::styled("  Type to input color", Style::default()));
            info_lines.push(Line::styled(
                "  ↑ & ↓: cycle through material types",
                Style::default(),
            ));
            info_lines.push(Line::styled("  Enter: Save", Style::default()));
            info_lines.push(Line::styled("  Esc: Cancel", Style::default()));
        }
        CurrentScreen::Render => {
            info_lines.push(Line::styled(
                "Render settings",
                Style::default().fg(Color::Red),
            ));
            info_lines.push("  Tab & Shift+Tab: Change inputs".into());
            info_lines.push(Line::styled("  ← & →: Change inputs", Style::default()));
            info_lines.push("  Type to input".into());
            info_lines.push("  Enter: Render scene ( this might take a bit )".into());
            info_lines.push("  Esc: Close".into());
        }
        CurrentScreen::Preview => {
            info_lines.push(Line::styled("Preview", Style::default().fg(Color::Red)));
            info_lines.push("  [F]: Full Screen".into());
            info_lines.push("  Esc: Close".into());
        }
        _ => {}
    }

    let info = Paragraph::new(Text::from(info_lines))
        .block(info_block)
        .wrap(Wrap { trim: false });

    //object table
    let object_data = app.world.as_info_vec();

    let object_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let rows = object_data
        .iter()
        .enumerate()
        .map(|(i, data)| {
            let color: Color;
            if Some(i) == app.selected_object {
                color = Color::Rgb(45, 45, 55);
            } else {
                color = match i % 2 {
                    0 => Color::Rgb(30, 30, 40),
                    _ => Color::Rgb(25, 25, 35),
                };
            }
            let item = data
                .iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Vec<_>>();
            Row::new(item)
                .style(Style::default().fg(Color::White).bg(color))
                .height(3)
        })
        .collect::<Vec<_>>();

    let widths = [
        Constraint::Length(8),
        Constraint::Length(6),
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Min(10),
    ];
    let mut table_state = TableState::default();
    table_state.select(app.selected_object);
    let table = Table::new(rows, widths)
        .header(
            Row::new(vec!["Type", "Radius", "X", "Y", "Z", "Material"])
                .style(Style::default().bg(Color::Rgb(30, 40, 75)))
                .height(2),
        )
        .block(object_block);

    //material list
    let mut materials = Vec::<ListItem>::new();

    for material in app.materials.iter() {
        materials.push(ListItem::new(Line::from(Span::styled(
            material.0.as_str(),
            Style::default().fg(Color::LightYellow),
        ))));
    }
    let material_block = Block::default()
        .borders(Borders::TOP)
        .border_type(BorderType::Plain)
        .title("Scene Materials")
        .title_alignment(Alignment::Center)
        .padding(Padding::left(2));

    let material_list = List::new(materials).block(material_block);

    //stats/info bar

    let stats_block = Block::default()
        .borders(Borders::union(Borders::RIGHT, Borders::BOTTOM))
        .border_type(BorderType::Thick)
        .style(Style::default());

    let stats_lines = vec![Line::from(vec![
        Span::raw(app.world.objects.len().to_string()),
        Span::styled(" Objects in Scene. ", Style::default().fg(Color::Green)),
        Span::raw(app.materials.len().to_string()),
        Span::styled(" Materials.", Style::default().fg(Color::LightBlue)),
        Span::styled("| ", Style::default().fg(Color::LightRed)),
        Span::raw(app.samples.to_string()),
        Span::styled(" Samples. ", Style::default().fg(Color::Cyan)),
        Span::raw(app.bounces.to_string()),
        Span::styled(" Bounces. ", Style::default().fg(Color::Magenta)),
        Span::styled("|", Style::default().fg(Color::LightRed)),
        Span::styled(
            format!(" {}.ppm ", app.image_name_input),
            Style::default().fg(Color::LightYellow),
        ),
        Span::raw(format!(
            "{}x{}",
            app.image_width.to_string(),
            app.image_height.to_string()
        )),
    ])];

    let stats = Paragraph::new(Text::from(stats_lines))
        .block(stats_block)
        .alignment(Alignment::Center);
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
    frame.render_widget(
        material_list,
        sidebar[1].inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
    );
    frame.render_widget(stats, main[1]);
    frame.render_stateful_widget(table, main[0], &mut table_state);

    match app.current_screen {
        CurrentScreen::Confirmation => {
            frame.render_widget(Clear, frame.area());
            frame.render_widget(confirmation_paragraph, centered_rect(40, 20, frame.area()))
        }
        CurrentScreen::Editor => editor(frame, app),
        CurrentScreen::MaterialEditor => material_editor(frame, app),
        CurrentScreen::Render => render_view(frame, main[0], app),
        CurrentScreen::Preview => render_preview(frame, main[0], app, true).unwrap_or(()),
        CurrentScreen::PreviewFull => render_preview(frame, frame.area(), app, false).unwrap_or(()),
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
        .margin(((editor_area.height - 2) / 2).clamp(1, 5))
        .spacing(2)
        .constraints([
            Constraint::Min(6),
            Constraint::Min(4),
            Constraint::Min(4),
            Constraint::Min(4),
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
        .title("Material")
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
    let txt_mat = Paragraph::new(app.materials[app.material_input].0.clone()).block(bl_mat);

    frame.render_widget(Clear, editor_area);
    frame.render_widget(editor_block, editor_area);

    frame.render_widget(txt_size, editor_chunks[0]);
    frame.render_widget(txt_posx, editor_chunks[1]);
    frame.render_widget(txt_posy, editor_chunks[2]);
    frame.render_widget(txt_posz, editor_chunks[3]);
    frame.render_widget(txt_mat, editor_chunks[4]);
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
            Constraint::Min(7),  //Type
            Constraint::Min(7),  //Color
            Constraint::Min(5),  //other
            Constraint::Min(15), //name
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
        ))
        .fg(Color::Rgb(
            255 - (app.get_color().x * 255.0) as u8,
            255 - (app.get_color().y * 255.0) as u8,
            255 - (app.get_color().z * 255.0) as u8,
        ));
    let mut bl_other = Block::default()
        .title(match app.mat_type_input {
            Some(MaterialType::Metal) => "Roughness",
            Some(MaterialType::Dielectric) => "IOR",
            _ => "ERROR",
        })
        .borders(Borders::NONE)
        .bg(Color::DarkGray);
    let mut bl_name = Block::default()
        .title("Name")
        .borders(Borders::NONE)
        .bg(Color::DarkGray);

    let selected_style = Style::default().bg(Color::White).fg(Color::Black);

    if let Some(editing) = &app.current_edit {
        match editing {
            CurrentlyEditing::MatType => bl_type = bl_type.style(selected_style),
            CurrentlyEditing::MatColor => bl_color = bl_color.style(selected_style),
            CurrentlyEditing::MatProperty => bl_other = bl_other.style(selected_style),
            CurrentlyEditing::MatName => bl_name = bl_name.style(selected_style),
            _ => {}
        }
    }

    let txt_type = Paragraph::new(app.mat_type_input.clone().unwrap().to_string()).block(bl_type);
    let txt_color = Paragraph::new(app.mat_color_input.clone()).block(bl_color);
    let txt_other = Paragraph::new(app.mat_other_input.clone()).block(bl_other);
    let txt_name = Paragraph::new(app.mat_name_input.clone()).block(bl_name);
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
    frame.render_widget(txt_name, editor_chunks[3]);
}
