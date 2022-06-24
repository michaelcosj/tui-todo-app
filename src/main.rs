use std::io::{stdout, Write};

use crossterm::{
    cursor,
    execute, queue,
    terminal, Result,
    event::{self, Event, KeyCode, KeyEvent},
    style::{self, Color, Colors},
};

use todo_app::TodoList;

struct Screen {
    row: u16,
    col: u16,
}

impl Screen {
    fn new(row: u16, col: u16) -> Screen {
        Screen { row, col }
    }

    fn display_line(&mut self, writer: &mut impl Write, text: &str) -> Result<()> {
        queue!(writer, cursor::MoveTo(0, self.row))?;
        queue!(writer, style::Print(text))?;
        writer.flush()?;
        self.row += 1;
        Ok(())
    }

    fn display_line_color(
        &mut self,
        writer: &mut impl Write,
        text: &str,
        colors: Colors,
    ) -> Result<()> {
        queue!(writer, style::SetColors(colors))?;
        self.display_line(writer, text)?;
        execute!(writer, style::ResetColor)?;
        Ok(())
    }

    fn skip_line(&mut self) {
        self.row += 1;
    }

    fn reset(&mut self) {
        self.col = 0;
        self.row = 0;
    }
}

fn main() -> Result<()> {
    let mut stdout = stdout();

    let mut list = TodoList::new("./todo");
    let mut selected = 0;

    let mut app_screen = Screen::new(0, 0);
    let select_color = Colors::new(Color::Black, Color::White);

    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
    terminal::enable_raw_mode()?;

    loop {
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

        app_screen.display_line(&mut stdout, "TUI TODO APP BY MICHAEL")?;
        app_screen.skip_line();

        app_screen.display_line(
            &mut stdout,
            "Keybindings: 'j' to move down, 'k' to move up, 'a' to add a todo,",
        )?;

        app_screen.display_line(
            &mut stdout,
            "             't' to tick a todo, 'r' to remove a todo, 'c' to clear done list",
        )?;

        app_screen.skip_line();

        app_screen.display_line(&mut stdout, "TODO")?;
        for (index, todo) in list.todo.iter().enumerate() {
            let todo_text = &format!("- [ ]  {}\n", todo);
            if index == selected {
                app_screen.display_line_color(&mut stdout, todo_text, select_color)?;
                continue;
            }
            app_screen.display_line(&mut stdout, todo_text)?;
        }

        app_screen.skip_line();

        app_screen.display_line(&mut stdout, "DONE")?;
        for done in list.done.iter() {
            app_screen.display_line(&mut stdout, &format!("- [X]\t{}\n", done))?;
        }

        execute!(stdout, cursor::MoveTo(0, selected as u16))?;

        app_screen.skip_line();
        app_screen.display_line(&mut stdout, ">")?;

        let key = read_char().unwrap();
        match key {
            'a' => {
                let new_todo = read_line(&mut stdout);
                if !new_todo.is_empty() {
                    list.add_todo(&new_todo);
                }
            }
            'j' => {
                if list.todo.len() > 0 && selected < list.todo.len() - 1 {
                    selected = selected + 1;
                }
            }
            'k' => {
                if selected > 0 {
                    selected = selected - 1;
                }
            }
            't' => list.tick_todo(selected),
            'r' => list.remove_todo(selected),
            'c' => list.clear_done(),
            'q' => break,
            _ => (),
        }

        app_screen.reset();
    }

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            if let KeyCode::Char(c) = code {
                return Ok(c);
            }
        }
    }
}

fn read_line(writer: &mut impl Write) -> String {
    execute!(writer, cursor::Show, cursor::SavePosition).unwrap();

    let mut line = String::new();
    while let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
        match code {
            KeyCode::Enter => break,
            KeyCode::Char(c) => line.push(c),
            KeyCode::Backspace => {
                line.pop();
                execute!(
                    writer,
                    cursor::RestorePosition,
                    terminal::Clear(terminal::ClearType::UntilNewLine)
                )
                .unwrap();
            }
            _ => {}
        }

        execute!(writer, cursor::RestorePosition, style::Print(&line)).unwrap();
    }

    execute!(writer, cursor::Hide).unwrap();
    line
}
