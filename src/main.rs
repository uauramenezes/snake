use std::collections::VecDeque;
use std::io::stdout;
use std::time;
use crossterm::{
    cursor,
    execute,
    style::{Print, SetForegroundColor, Color, ResetColor},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{
       self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, SetSize,
    },
};
use rand::Rng;

struct Position {
    x: u16,
    y: u16,
}

struct Direction {
    x: i16,
    y: i16,
}

struct Size {
    width: u16,
    height: u16,
}

fn get_random_xy(number: u16) -> u16 {
    let mut rng = rand::thread_rng();
    let mut rn = rng.gen_range(1..number);
    while rn % 2 != 0 {
        rn = rng.gen_range(1..number);
    }
    rn
}

fn main() -> crossterm::Result<()> {
    let mut stdout = stdout();

    let window = Size{width: 60, height: 20};

    execute!(stdout, SetSize(window.width, window.height))?;
    terminal::enable_raw_mode()?;
    execute!(stdout, Clear(ClearType::All), EnterAlternateScreen, cursor::Hide)?;
    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(window.width / 2 - 10, window.height / 2 - 5),
        Print("Press ENTER to play!"),
        cursor::MoveTo(window.width / 2 - 10, window.height / 2),
        Print("Press ESC to quit!"),
    )?;

    loop {
        let mut game_over = false;
    
        loop {
            match read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: KeyModifiers::NONE,
                }) => {
                    game_over = true;
                    break
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                }) => {
                    break
                },
                _ => (),
            };
        }
    
        if game_over {break};
        
        let mut snake: VecDeque<Position> = VecDeque::new();
    
        let mut pos = Position{x: window.width / 2, y: window.height / 2};
        let mut dir = Direction{x: 1, y:0};
    
        for n in 0..5 {
            let p = Position{x: pos.x - n - 1, y: pos.y};
            snake.push_back(p);
        }
    
        let mut food = Position{x: get_random_xy(window.width), y: get_random_xy(window.height)};
    
        let mut score = 0;
        
        loop {
            execute!(
                stdout,
                Clear(ClearType::All),
            )?;
    
            for n in 0..snake.len() {
                execute!(
                    stdout, 
                    cursor::MoveTo(snake[n].x, snake[n].y),
                    Print("\u{2588}\u{2588}"),
                )?;
            }
            
            execute!(
                stdout,
                cursor::MoveTo(0, 0),
                SetForegroundColor(Color::Red),
                Print(format!("Score: {}", score)),
                SetForegroundColor(Color::Green),
                cursor::MoveTo(food.x, food.y),
                Print("\u{2588}\u{2588}"),
                ResetColor,
            )?;
    
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
                        code: KeyCode::Esc,
                        modifiers: KeyModifiers::NONE,
                    }) => {
                        break;
                    },
                    _ => (),
                }
            } else {
                ()
            }
    
            pos.x = (pos.x as i16 + dir.x) as u16;
            pos.y = (pos.y as i16 + dir.y) as u16;
    
            if (snake[0].x == food.x && snake[0].y == food.y) ||
            (snake[0].x == food.x + 1 && snake[0].y == food.y){
                score += 10;
                snake.push_front(Position{x: pos.x, y: pos.y});
                food = Position{x: get_random_xy(window.width), y: get_random_xy(window.height)};
            } else {
                snake.pop_back();
                snake.push_front(Position{x: pos.x, y: pos.y});
            }
    
            for i in 1..snake.len() {
                if (snake[0].x == snake[i].x && snake[0].y == snake[i].y) ||
                (snake[0].x + 1 == snake[i].x && snake[0].y == snake[i].y) {
                    game_over = true;
                }
            }

            if 0 > pos.x as i16 || 0 > pos.y as i16 || pos.x == window.width - 1 || pos.y == window.height || game_over {
                break
            }
        }
    
        execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(window.width / 2 - 5, window.height / 2 - 5),
            Print("GAME OVER"),
            cursor::MoveTo(window.width / 2 - 5, window.height / 2),
            Print(format!("Score: {}", score)),
        )?;
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
