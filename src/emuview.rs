pub mod emuview {
    use std::io;
    use termion::raw::{IntoRawMode, RawTerminal};
    use termion::screen::{AlternateScreen, IntoAlternateScreen};
    use tui::backend::TermionBackend;
    use tui::layout::{Constraint, Direction, Layout};
    use tui::style::{Color, Modifier, Style};
    use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Widget};
    use tui::text::Text;
    use tui::Terminal;

    pub struct TUIRenderer {
        terminal: Terminal<TermionBackend<AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>>>,
    }

    impl TUIRenderer {
        pub fn new() -> io::Result<Self> {
            let stdout = io::stdout().into_raw_mode()?;
            let screen = stdout.into_alternate_screen().unwrap();
            let backend = TermionBackend::new(screen);
            Ok(Self {
                terminal: Terminal::new(backend).unwrap(),
            })
        }

        pub fn render(&mut self, memory: &[u8; 1024*1024], registers: &[u32; 32]) -> io::Result<()> {
            let memory_bytes: Vec<String> = memory.iter().map(|&b| format!("{:02x}", b)).collect();
            let memory_lines: Vec<ListItem> = memory_bytes
                .chunks(32)
                .enumerate()
                .map(|(i, chunk)| ListItem::new(format!("x{:02}: {}\n", i, chunk.join(" "))).style(Style::default().fg(Color::Gray)))
                .collect();
            let memory_list = List::new(memory_lines)
                .block(Block::default().title("Memory").borders(Borders::ALL));
                //.highlight_style(Style::default().add_modifier(Modifier::BOLD))
                //.highlight_symbol(">>");
        
            let register_bytes: Vec<String> = registers.iter().map(|&b| format!("{:08x}", b)).collect();
            let registers: Vec<ListItem> = register_bytes
                .chunks(4)
                .enumerate()
                .map(|(i, chunk)| ListItem::new(format!("x{:02}: {}\n", i, chunk.join(" "))))
                .collect();
            let registers_block = List::new(registers)
                .block(Block::default().title("Registers").borders(Borders::ALL));
                //.wrap(tui::widgets::Wrap { trim: false });
        
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(self.terminal.size()?);
            self.terminal.draw(|f| {
                let text = [Text::raw("Hello TUI!")];
                let block = Block::default().title("Block").borders(Borders::ALL);
                f.render_widget(block, chunks[0]);
                f.render_widget(memory_list, chunks[0]);
                f.render_widget(registers_block, chunks[1]);
            }).unwrap();

            Ok(())
        }

    }
}