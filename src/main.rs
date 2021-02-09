use std::collections::VecDeque;
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

    execute!(stdout, SetSize(120, 40))?;
    terminal::enable_raw_mode()?;
    execute!(stdout, Clear(ClearType::All), EnterAlternateScreen, cursor::Hide)?;

    let mut snake: VecDeque<Position> = VecDeque::new();

    let mut pos = Position{x: 60, y:20};
    let mut dir = Direction{x: 1, y:0};

    for n in 0..5 {
        let p = Position{x: pos.x - n - 1, y: pos.y};
        snake.push_back(p);
    }

    loop {
        execute!(
            stdout,
            Clear(ClearType::All),
        )?;

        for n in 0..snake.len() {
            execute!(
                stdout, 
                cursor::MoveTo(snake[n].x as u16, snake[n].y as u16),
                Print("\u{2588}\u{2588}"),
            )?;
        }

        if poll(time::Duration::from_millis(100))? {
            match read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('a'),
                    modifiers: KeyModifiers::NONE,
                }) | Event::Key(
                    KeyEvent {
                        code: KeyCode::Left,
                        modifiers: KeyModifiers::NONE,
                    }
                ) => {
                    if dir.x == 0 {
                        dir.x = -2;
                        dir.y = 0;
                    }
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::NONE,
                }) | Event::Key(
                    KeyEvent {
                        code: KeyCode::Right,
                        modifiers: KeyModifiers::NONE,
                    }
                ) => {
                    if dir.x == 0 {
                        dir.x = 2;
                        dir.y = 0; 
                    }
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Char('w'),
                    modifiers: KeyModifiers::NONE,
                }) | Event::Key(
                    KeyEvent {
                        code: KeyCode::Up,
                        modifiers: KeyModifiers::NONE,
                    }
                ) => {
                    if dir.y == 0 {
                        dir.x = 0;
                        dir.y = -1;
                    }
                    
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: KeyModifiers::NONE,
                }) | Event::Key(
                    KeyEvent {
                        code: KeyCode::Down,
                        modifiers: KeyModifiers::NONE,
                    }
                ) => {
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
        snake.pop_back();
        snake.push_front(Position{x: pos.x, y: pos.y});

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

