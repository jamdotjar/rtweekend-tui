#![allow(unused_imports)]
mod app;
mod render;
mod ui;
use app::*;
use crossterm::event::{self, DisableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use ratatui::prelude::Backend;
use ratatui::Terminal;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::CrosstermBackend,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use render::*;
use rtwlib::camera::*;
use std::error::Error;
use std::io;
use std::result::Result::Ok;
use ui::*;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            println!("today");
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue; //skips loggint the release of keys
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('n') => {
                        app.current_screen = CurrentScreen::Editor;
                        app.current_edit = Some(CurrentlyEditing::Size)
                    }
                    KeyCode::Char('m') => {
                        app.current_screen = CurrentScreen::MaterialEditor;
                        app.current_edit = Some(CurrentlyEditing::MatType);
                        app.mat_type_input = Some(MaterialType::Lambertian);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Confirmation;
                    }
                    KeyCode::Char('r') => {
                        app.current_screen = CurrentScreen::Render;
                        app.current_edit = Some(CurrentlyEditing::Width);
                    }
                    _ => {}
                },
                CurrentScreen::Confirmation => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },
                CurrentScreen::Editor => match key.code {
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                        app.current_edit = None;
                    }
                    KeyCode::Tab => app.change_editing(),
                    KeyCode::Enter => {
                        if let Some(editing) = &app.current_edit {
                            match app.save_object() {
                                Ok(_) => app.current_screen = CurrentScreen::Main,
                                Err(_) => {}
                            }
                        }
                    }
                    KeyCode::Up => {
                        if let Some(CurrentlyEditing::Material) = &app.current_edit {
                            if app.material_input < app.materials.len() - 1 {
                                app.material_input += 1;
                            } else {
                                app.material_input = 0;
                            }
                        }
                    }

                    KeyCode::Down => {
                        if let Some(CurrentlyEditing::Material) = &app.current_edit {
                            if app.material_input > 0 {
                                app.material_input -= 1;
                            } else {
                                app.material_input = app.materials.len() - 1
                            }
                        }
                    }

                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.current_edit {
                            match editing {
                                CurrentlyEditing::Size => {
                                    app.size_input.push(value);
                                }
                                CurrentlyEditing::PositionX => {
                                    app.position_input_x.push(value);
                                }
                                CurrentlyEditing::PositionY => {
                                    app.position_input_y.push(value);
                                }
                                CurrentlyEditing::PositionZ => {
                                    app.position_input_z.push(value);
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(editing) = &app.current_edit {
                            match editing {
                                CurrentlyEditing::Size => {
                                    app.size_input.pop();
                                }
                                CurrentlyEditing::PositionX => {
                                    app.position_input_x.pop();
                                }
                                CurrentlyEditing::PositionY => {
                                    app.position_input_y.pop();
                                }
                                CurrentlyEditing::PositionZ => {
                                    app.position_input_z.pop();
                                }
                                _ => {}
                            }
                        }
                    }

                    _ => {}
                },
                CurrentScreen::MaterialPicker => match key.code {
                    KeyCode::Enter => {
                        //save the material choice
                        app.current_screen = CurrentScreen::Editor
                    }
                    _ => {}
                },
                CurrentScreen::MaterialEditor => match key.code {
                    KeyCode::Tab => app.change_editing(),
                    KeyCode::Esc => app.current_screen = CurrentScreen::Main,
                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.current_edit {
                            match editing {
                                CurrentlyEditing::MatColor => {
                                    app.mat_color_input.push(value);
                                }
                                CurrentlyEditing::MatProperty => {
                                    app.mat_other_input.push(value);
                                }
                                CurrentlyEditing::MatName => {
                                    app.mat_name_input.push(value);
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(editing) = &app.current_edit {
                            match editing {
                                CurrentlyEditing::MatColor => {
                                    app.mat_color_input.pop();
                                }
                                CurrentlyEditing::MatProperty => {
                                    app.mat_other_input.pop();
                                }
                                CurrentlyEditing::MatName => {
                                    app.mat_name_input.pop();
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Up => {
                        if let Some(editing) = &app.current_edit {
                            match editing {
                                CurrentlyEditing::MatType => {
                                    if let Some(mat_type) = &app.mat_type_input {
                                        app.mat_type_input = match mat_type {
                                            MaterialType::Lambertian => Some(MaterialType::Metal),
                                            MaterialType::Metal => Some(MaterialType::Dielectric),
                                            MaterialType::Dielectric => Some(MaterialType::Normal),
                                            MaterialType::Normal => Some(MaterialType::Lambertian),
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Enter => match app.save_material() {
                        Ok(_) => {
                            app.current_screen = CurrentScreen::Main;
                            app.current_edit = None;
                        }
                        Err(_) => {}
                    },
                    _ => {}
                },
                CurrentScreen::ColorEditor => match key.code {
                    KeyCode::Tab => app.change_editing(),
                    _ => {}
                },
                CurrentScreen::Render => match key.code {
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Enter => {
                        render_image(app, terminal);
                    }
                    KeyCode::Tab => app.change_editing(),
                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.current_edit {
                            match editing {
                                CurrentlyEditing::Width => {
                                    app.image_width.push(value);
                                }
                                CurrentlyEditing::Height => {
                                    app.image_height.push(value);
                                }
                                CurrentlyEditing::ImgName => {
                                    app.image_name_input.push(value);
                                }
                                CurrentlyEditing::Samples => {
                                    app.samples.push(value);
                                }
                                CurrentlyEditing::Bounces => {
                                    app.bounces.push(value);
                                }
                                CurrentlyEditing::CamX => {
                                    app.camx.push(value);
                                }
                                CurrentlyEditing::CamY => {
                                    app.camy.push(value);
                                }
                                CurrentlyEditing::CamZ => {
                                    app.camz.push(value);
                                }
                                CurrentlyEditing::LookX => {
                                    app.lookx.push(value);
                                }
                                CurrentlyEditing::LookY => {
                                    app.looky.push(value);
                                }
                                CurrentlyEditing::LookZ => {
                                    app.lookz.push(value);
                                }
                                CurrentlyEditing::Fov => {
                                    app.fov.push(value);
                                }
                                CurrentlyEditing::FocusDist => {
                                    app.focus_dist.push(value);
                                }
                                CurrentlyEditing::Aperture => {
                                    app.aperture.push(value);
                                }
                                _ => {}
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(editing) = &app.current_edit {
                            match editing {
                                CurrentlyEditing::Width => {
                                    app.image_width.pop();
                                }
                                CurrentlyEditing::Height => {
                                    app.image_height.pop();
                                }
                                CurrentlyEditing::ImgName => {
                                    app.image_name_input.pop();
                                }
                                CurrentlyEditing::Samples => {
                                    app.samples.pop();
                                }
                                CurrentlyEditing::Bounces => {
                                    app.bounces.pop();
                                }
                                CurrentlyEditing::CamX => {
                                    app.camx.pop();
                                }
                                CurrentlyEditing::CamY => {
                                    app.camy.pop();
                                }
                                CurrentlyEditing::CamZ => {
                                    app.camz.pop();
                                }
                                CurrentlyEditing::LookX => {
                                    app.lookx.pop();
                                }
                                CurrentlyEditing::LookY => {
                                    app.looky.pop();
                                }
                                CurrentlyEditing::LookZ => {
                                    app.lookz.pop();
                                }
                                CurrentlyEditing::Fov => {
                                    app.fov.pop();
                                }
                                CurrentlyEditing::FocusDist => {
                                    app.focus_dist.pop();
                                }
                                CurrentlyEditing::Aperture => {
                                    app.aperture.pop();
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
    }
}
