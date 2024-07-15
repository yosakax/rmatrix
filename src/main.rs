use std::io::{stdout, Stdout, Write};
use std::thread;
use std::time::Duration;

use crossterm::cursor::{Hide, Show};
use crossterm::style::{PrintStyledContent, Stylize};
use crossterm::terminal::{Clear, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, execute, queue};

use terminal_size::terminal_size;

#[derive(Debug)]
struct TerminalMatrix {
    height: u16,
    width: u16,
    data: Vec<Vec<char>>,
}

impl TerminalMatrix {
    fn new(height: u16, width: u16) -> Self {
        let data = vec![vec!['@'; width as usize]; height as usize];
        TerminalMatrix {
            height,
            width,
            data,
        }
    }
    fn clear(&mut self) {
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                self.data[y][x] = ' ';
            }
        }
    }
    fn fill(&mut self) {
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                self.data[y][x] = '@';
            }
        }
    }
    fn toggle(&mut self) {
        if self.data[0][0] == ' ' {
            self.fill();
        } else {
            self.clear();
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let mut stdout = stdout();
    let size = terminal_size().unwrap();
    let width = size.0 .0;
    let height = size.1 .0;

    let duration = Duration::from_millis(300);
    let mut matrix = TerminalMatrix::new(height, width);
    println!("{:?}", matrix);
    execute!(
        stdout,
        Hide,
        EnterAlternateScreen,
        Clear(crossterm::terminal::ClearType::All)
    )?;
    // let block = PrintStyledContent("█".magenta());
    // for y in 0..height {
    //     for x in 0..width {
    //         let block = PrintStyledContent(matrix.data[y as usize][x as usize].stylize());
    //         queue!(stdout, cursor::MoveTo(x, y), block)?;
    //     }
    // }
    for _ in 0..100 {
        for y in 0..height {
            for x in 0..width {
                let block = PrintStyledContent(matrix.data[y as usize][x as usize].grey());
                queue!(stdout, cursor::MoveTo(x, y), block)?;
            }
        }
        stdout.flush()?; // ここでqueueされた順で遅延評価される
        matrix.toggle();
        thread::sleep(duration);
        execute!(stdout, Show, LeaveAlternateScreen,)?;
    }
    Ok(())
}
