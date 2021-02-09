use std::io::stdout;
use std::{thread, time};
use crossterm::{
    cursor,
    execute,
    style::Print,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{
       self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetSize,
    },
};

struct Position {
    x: i16,
    y: i16,
}

struct Direction {
    x: i16,
    y: i16,
}

fn main() -> crossterm::Result<()> {
    let mut stdout = stdout();

    let mut pos = Position{x: 10, y:10};
    let mut dir = Direction{x: 1, y:0};

    execute!(stdout, SetSize(120, 40))?;
    terminal::enable_raw_mode()?;
    execute!(stdout, Clear(ClearType::All), EnterAlternateScreen, cursor::Hide)?;

    loop {
        execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(pos.x as u16, pos.y as u16),
            Print("\u{2588}")
        )?;

        if poll(time::Duration::from_millis(100))? {
            match read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('a'),
                    modifiers: KeyModifiers::NONE,
                }) => {
                    if dir.x == 0 {
                        dir.x = -1;
                        dir.y = 0;
                    }
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::NONE,
                }) => {
                    if dir.x == 0 {
                        dir.x = 1;
                        dir.y = 0; 
                    }
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Char('w'),
                    modifiers: KeyModifiers::NONE,
                }) => {
                    if dir.y == 0 {
                        dir.x = 0;
                        dir.y = -1;
                    }
                    
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: KeyModifiers::NONE,
                }) => {
                    if dir.y == 0 {
                        dir.x = 0;
                        dir.y = 1;
                    }
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                }) => break,
                _ => (),
            }
        } else {
            ()
        }

        pos.x += dir.x;
        pos.y += dir.y;

        if pos.x < 0 || pos.y < 0 || pos.x > 120 || pos.y > 40 {
            break
        }
        
        thread::sleep(time::Duration::from_millis(100));
    }   

    execute!(
        stdout,
        Clear(ClearType::All),
        LeaveAlternateScreen,
        cursor::Show,
        cursor::MoveTo(0, 0)
    )?;

    terminal::disable_raw_mode()?;

    Ok(())
}

