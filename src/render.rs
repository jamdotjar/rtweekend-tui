#![warn(clippy::pedantic)]

use std::fs::File;

use color_eyre::Result;
use crossterm::terminal;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::Backend,
    style::{Modifier, Style, Stylize},
    symbols,
    widgets::{Block, BorderType, Borders, Clear, Gauge, LineGauge, Paragraph},
    Frame, Terminal,
};
use rtwlib::{
    camera::{self, Camera},
    vec3::Point3,
};

use crate::{centered_rect, App, CurrentlyEditing};

pub fn render_view(frame: &mut Frame, area: Rect, app: &App) {
    // set settings for render
    // required settings:
    // image size ( horizontal and vertical )
    // samples per pixel
    // bounces
    // camera position
    // camera look at

    // render image

    let render_block = Block::default()
        .title("Render your scene")
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .title_alignment(Alignment::Center)
        .title_style(Style::default().add_modifier(Modifier::BOLD));
    let render_popup_area = centered_rect(90, 90, area);

    let render_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Min(2),
            Constraint::Length(1),
            Constraint::Min(2),
            Constraint::Length(1),
            Constraint::Min(2),
            Constraint::Min(2),
        ])
        .margin(1)
        .split(render_popup_area);
    let image_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(5),  //width
            Constraint::Min(5),  //height
            Constraint::Min(10), //filename
        ])
        .spacing(1)
        .split(render_chunks[1]);
    let quality_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Min(6), Constraint::Min(6)])
        .spacing(1)
        .split(render_chunks[3]);
    let camera_position_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(6), //x
            Constraint::Min(6), //y
            Constraint::Min(6), //z
            Constraint::Min(6), //lookx
            Constraint::Min(6), //looky
            Constraint::Min(6), //lookz
        ])
        .spacing(1)
        .split(render_chunks[5]);
    let camera_settings_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(6), //fov
            Constraint::Min(6), //focus_dist
            Constraint::Min(6), //aperture
        ])
        .spacing(1)
        .split(render_chunks[6]);

    let base_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain);

    let mut width_block = base_block.clone().title("Image Width");
    let mut height_block = base_block.clone().title("Image Height");
    let mut filename_block = base_block.clone().title("Filename");
    let mut samples_block = base_block.clone().title("Samples");
    let mut bounces_block = base_block.clone().title("Bounces");
    let mut camx_block = base_block.clone().title("Camera X");
    let mut camy_block = base_block.clone().title("Camera Y");
    let mut camz_block = base_block.clone().title("Camera Z");
    let mut lookx_block = base_block.clone().title("Look X");
    let mut looky_block = base_block.clone().title("Look Y");
    let mut lookz_block = base_block.clone().title("Look Z");
    let mut fov_block = base_block.clone().title("FOV");
    let mut focus_dist_block = base_block.clone().title("Focus Distance");
    let mut aperture_block = base_block.clone().title("Blur amount");

    let style = Style::default().bold();

    if let Some(editing) = &app.current_edit {
        match editing {
            CurrentlyEditing::Width => {
                width_block = width_block.border_type(BorderType::Double).style(style)
            }
            CurrentlyEditing::Height => {
                height_block = height_block.border_type(BorderType::Double).style(style)
            }
            CurrentlyEditing::ImgName => {
                filename_block = filename_block.border_type(BorderType::Double).style(style)
            }
            CurrentlyEditing::Samples => {
                samples_block = samples_block.border_type(BorderType::Double).style(style)
            }
            CurrentlyEditing::Bounces => {
                bounces_block = bounces_block.border_type(BorderType::Double).style(style)
            }
            CurrentlyEditing::CamX => {
                camx_block = camx_block.border_type(BorderType::Double).style(style)
            }
            CurrentlyEditing::CamY => {
                camy_block = camy_block.border_type(BorderType::Double).style(style)
            }
            CurrentlyEditing::CamZ => {
                camz_block = camz_block.border_type(BorderType::Double).style(style)
            }
            CurrentlyEditing::LookX => {
                lookx_block = lookx_block.border_type(BorderType::Double).style(style)
            }
            CurrentlyEditing::LookY => {
                looky_block = looky_block.border_type(BorderType::Double).style(style)
            }
            CurrentlyEditing::LookZ => {
                lookz_block = lookz_block.border_type(BorderType::Double).style(style)
            }
            CurrentlyEditing::Fov => {
                fov_block = fov_block.border_type(BorderType::Double).style(style)
            }
            CurrentlyEditing::FocusDist => {
                focus_dist_block = focus_dist_block
                    .border_type(BorderType::Double)
                    .style(style)
            }
            CurrentlyEditing::Aperture => {
                aperture_block = aperture_block.border_type(BorderType::Double).style(style)
            }
            _ => {}
        }
    };

    let txt_width = Paragraph::new(app.image_width.clone()).block(width_block);
    let txt_height = Paragraph::new(app.image_height.clone()).block(height_block);
    let txt_filename = Paragraph::new(app.image_name_input.clone()).block(filename_block);
    let txt_samples = Paragraph::new(app.samples.clone()).block(samples_block);
    let txt_bounces = Paragraph::new(app.bounces.clone()).block(bounces_block);
    let txt_camx = Paragraph::new(app.camx.clone()).block(camx_block);
    let txt_camy = Paragraph::new(app.camy.clone()).block(camy_block);
    let txt_camz = Paragraph::new(app.camz.clone()).block(camz_block);
    let txt_lookx = Paragraph::new(app.lookx.clone()).block(lookx_block);
    let txt_looky = Paragraph::new(app.looky.clone()).block(looky_block);
    let txt_lookz = Paragraph::new(app.lookz.clone()).block(lookz_block);
    let txt_fov = Paragraph::new(app.fov.clone()).block(fov_block);
    let txt_focus_dist = Paragraph::new(app.focus_dist.clone()).block(focus_dist_block);
    let txt_aperture = Paragraph::new(app.aperture.clone()).block(aperture_block);

    let txt_render = Paragraph::new("Edit the settings below, and then hit ENTER to render")
        .style(Style::default().add_modifier(Modifier::BOLD));
    let txt_quality =
        Paragraph::new("Quality Settings").style(Style::default().add_modifier(Modifier::BOLD));
    let txt_camera =
        Paragraph::new("Camera Settings").style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(Clear, render_popup_area);
    frame.render_widget(render_block, render_popup_area);
    frame.render_widget(txt_render, render_chunks[0]);
    frame.render_widget(txt_quality, render_chunks[2]);
    frame.render_widget(txt_camera, render_chunks[4]);

    frame.render_widget(txt_width, image_chunks[0]);
    frame.render_widget(txt_height, image_chunks[1]);
    frame.render_widget(txt_filename, image_chunks[2]);
    frame.render_widget(txt_samples, quality_chunks[0]);
    frame.render_widget(txt_bounces, quality_chunks[1]);
    frame.render_widget(txt_camx, camera_position_chunks[0]);
    frame.render_widget(txt_camy, camera_position_chunks[1]);
    frame.render_widget(txt_camz, camera_position_chunks[2]);
    frame.render_widget(txt_lookx, camera_position_chunks[3]);
    frame.render_widget(txt_looky, camera_position_chunks[4]);
    frame.render_widget(txt_lookz, camera_position_chunks[5]);
    frame.render_widget(txt_fov, camera_settings_chunks[0]);
    frame.render_widget(txt_focus_dist, camera_settings_chunks[1]);
    frame.render_widget(txt_aperture, camera_settings_chunks[2]);
}

pub fn render_image<B: Backend>(app: &mut App, terminal: &mut Terminal<B>) -> Result<()> {
    // render image
    let file = File::create(format!("{}.ppm", app.image_name_input))?;
    let mut cam = Camera::new(file);
    cam.aspect_ratio = app.image_width.parse::<f64>()? / app.image_height.parse::<f64>()?;
    cam.image_width = app.image_width.parse::<u32>()?;
    cam.samples = app.samples.parse::<u32>()?;
    cam.bounces = app.bounces.parse::<u32>()?;

    cam.lookfrom = Point3::new(
        app.camx.parse::<f64>()?,
        app.camy.parse::<f64>()?,
        app.camz.parse::<f64>()?,
    );

    cam.lookat = Point3::new(
        app.lookx.parse::<f64>()?,
        app.looky.parse::<f64>()?,
        app.lookz.parse::<f64>()?,
    );

    cam.vup = Point3::new(0.0, 1.0, 0.0);

    cam.vfov = app.fov.parse::<f64>()?;
    cam.focus_dist = app.focus_dist.parse::<f64>()?;
    cam.defocus_angle = app.aperture.parse::<f64>()?;

    cam.render(app.world.clone(), |progress| {
        app.render_progress = progress as f64 / app.image_height.parse::<f64>().unwrap();
        let _ = terminal.draw(|f| {
            progress_ui(f, app);
        });
    })?;
    Ok(())
}

fn progress_ui(frame: &mut Frame, app: &App) {
    let progress_block = Block::default()
        .title(format!("Rendering to {}.ppm", app.image_name_input))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title_alignment(Alignment::Center)
        .title_style(Style::default().add_modifier(Modifier::BOLD));
    let progress_popup_area = centered_rect(70, 20, frame.area());

    let progress_gauge = Gauge::default()
        .block(progress_block)
        .gauge_style(Style::new().white().on_black().bold())
        .ratio(app.render_progress)
        .use_unicode(true)
        .label(format!("{:.2}%", app.render_progress * 100.0));
    frame.render_widget(progress_gauge, progress_popup_area);
}
