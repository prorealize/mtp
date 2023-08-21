use crossterm::event::{MouseEvent, MouseEventKind};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::{error::Error, io};

use crate::app::App;


pub fn run(_c: Vec<Vec<u8>>, _p: Vec<Option<u8>>, _o: String) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new(_c, _p, _o);
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        let app_event = event::read()?;
        match app_event {
            Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(to_insert) => {
                            app.enter_char(to_insert);
                        }
                        KeyCode::Backspace => {
                            app.delete_char(true);
                        }
                        KeyCode::Delete => {
                            app.delete_char(false);
                        }
                        KeyCode::Home => {
                            app.move_cursor_home();
                        }
                        KeyCode::End => {
                            app.move_cursor_end();
                        }
                        KeyCode::Left => {
                            app.move_cursor_left();
                        }
                        KeyCode::Right => {
                            app.move_cursor_right();
                        }
                        KeyCode::Up => {
                            app.move_cursor_up();
                        }
                        KeyCode::Down => {
                            app.move_cursor_down();
                        }
                        KeyCode::Esc => {
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
            Event::Mouse(MouseEvent {
                kind: MouseEventKind::Down(_),
                column,
                row,
                ..
            }) => {
                app.move_cursor_to_position(column as usize, row as usize);
            }
            _ => {}
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(f.size());

    let (decryption_area, key_area) = (chunks[0], chunks[1]);
    app.min_x = decryption_area.x as usize + 1;
    app.min_y = decryption_area.y as usize + 1;
    app.max_x = decryption_area.width as usize - decryption_area.x as usize - 1;
    app.max_y = decryption_area.height as usize - decryption_area.y as usize - 1;

    let messages = decryption_widget(app);
    f.render_widget(messages, decryption_area);
    f.set_cursor(
        // Draw the cursor at the current position in the input field.
        // This position is can be controlled via the left and right arrow key
        decryption_area.x + app.cursor_position_x as u16 + 1,
        // Move one line down, from the border to the input line
        decryption_area.y + app.cursor_position_y as u16 + 1,
    );
    let key = key_widget(app);
    f.render_widget(key, key_area);
}

fn key_widget(app: &App) -> Paragraph {
    let mut line = vec![];
    for byte in app.partial_key.iter() {
        match byte {
            Some(b) => line.push(Span::raw(format!("{:02X}", b))),
            None => line.push(unknown_character()),
        }
    }
    let key = Paragraph::new(Line::from(line))
        .style(Style::default())
        .block(Block::default().borders(Borders::ALL).title(" Key "))
        .wrap(Wrap { trim: true });
    key
}

fn decryption_widget(app: &App) -> Paragraph {
    let messages: Vec<Line> = app
        .ciphertexts
        .iter()
        .enumerate()
        .map(|(i, m)| Line::from(partial_decrypt(&app.partial_key, m, i)))
        .collect();
    let paragraph = Paragraph::new(messages)
        .block(Block::default().borders(Borders::ALL).title(" Decryption "))
        .wrap(Wrap { trim: true });
    paragraph
}

/// Decrypt ciphertext using key
/// Decrypting a letter using an unknown key element will result in unknown_character
fn partial_decrypt<'a>(key: &'a Vec<Option<u8>>, ciphertext: &'a Vec<u8>, _i: usize) -> Vec<Span<'a>> {
    let mut message = Vec::new();
    // message.push(Span::styled(
    //     format!("{}", i + 1),
    //     Style::default().add_modifier(Modifier::BOLD).fg(Color::White).bg(Color::DarkGray)));
    // message.push(Span::raw(": "));
    for (c, d) in key.iter().zip(ciphertext.iter()) {
        match c {
            Some(c) => {
                let r = c ^ d;
                if r.is_ascii_graphic() || r == 0x20 {
                    message.push(Span::from(String::from(r as char)))
                } else {
                    message.push(unknown_character())
                }
            }
            None => message.push(unknown_character()),
        };
    }
    message
}

fn unknown_character() -> Span<'static> {
    Span::styled(
        "_",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Red)
            .bg(Color::DarkGray),
    )
}
