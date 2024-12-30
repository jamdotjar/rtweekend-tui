#![allow(unused_imports)]
mod app;
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
use rtwlib::camera::*;
use std::error::Error;
use std::io;
use ui::*;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
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
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Confirmation;
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
                            match editing {
                                CurrentlyEditing::Material => match app.save_object() {
                                    Ok(_) => app.current_screen = CurrentScreen::Main,
                                    Err(_) => {}
                                },
                                _ => {}
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
                                CurrentlyEditing::Material => {
                                    app.current_screen = CurrentScreen::MaterialPicker;
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
                    KeyCode::Up => {
                        if app.material_input < app.materials.len() - 1 {
                            app.material_input += 1;
                        } else {
                            app.material_input = 0;
                        }
                    }
                    KeyCode::Down => {
                        if app.material_input > 0 {
                            app.material_input -= 1;
                        } else {
                            app.material_input = app.materials.len() - 1
                        }
                    }
                    KeyCode::Enter => {
                        //save the material choice
                        app.current_screen = CurrentScreen::Editor
                    }
                    KeyCode::Char('n') => {
                        app.current_screen = CurrentScreen::MaterialEditor;
                        app.current_edit = Some(CurrentlyEditing::MatColor);
                        app.mat_type_input = Some(MaterialType::Lambertian);
                    }
                    _ => {}
                },
                CurrentScreen::MaterialEditor => match key.code {
                    KeyCode::Tab => app.change_editing(),
                    KeyCode::Esc => app.current_screen = CurrentScreen::MaterialPicker,
                    KeyCode::Enter => {
                        if let Some(editing) = &app.current_edit {
                            match editing {
                                CurrentlyEditing::MatColor => {
                                    app.current_edit = Some(CurrentlyEditing::ColorR);
                                    app.current_screen = CurrentScreen::ColorEditor
                                }
                                CurrentlyEditing::MatType => app.cycle_mat_type(),
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                },
                CurrentScreen::ColorEditor => match key.code {
                    KeyCode::Tab => app.change_editing(),
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
