use std::{io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Flex, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Terminal,
};
use ratatui_opentui_loader::{KittLoader, Theme};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let all_themes = Theme::all();
    let mut loaders: Vec<KittLoader> = all_themes
        .iter()
        .map(|theme| KittLoader::with_theme(*theme))
        .collect();

    let dim = Style::default().fg(ratatui::style::Color::Rgb(140, 140, 140));
    let title_style = Style::default()
        .fg(ratatui::style::Color::Rgb(200, 200, 200))
        .add_modifier(Modifier::BOLD);

    // Pagination
    let mut page = 0;
    let per_page = 12;
    let total_pages = (all_themes.len() + per_page - 1) / per_page;

    loop {
        terminal.draw(|frame| {
            let start = page * per_page;
            let end = (start + per_page).min(all_themes.len());
            let visible = &all_themes[start..end];
            let visible_count = visible.len();

            // rows: title + blank + (label + loader) * visible + blank + footer
            let mut constraints = vec![
                Constraint::Length(1), // title
                Constraint::Length(1), // blank
            ];
            for _ in 0..visible_count {
                constraints.push(Constraint::Length(1)); // label
                constraints.push(Constraint::Length(1)); // loader
            }
            constraints.push(Constraint::Length(1)); // blank
            constraints.push(Constraint::Length(1)); // footer

            let row_count: u16 = constraints.iter().map(|c| match c {
                Constraint::Length(n) => *n,
                _ => 0,
            }).sum();

            let [_, center, _] = Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(row_count),
                Constraint::Fill(1),
            ])
            .areas(frame.area());

            let [col] = Layout::horizontal([Constraint::Percentage(50)])
                .flex(Flex::Center)
                .areas(center);

            let rows = Layout::vertical(constraints).split(col);

            // Title
            frame.render_widget(
                Paragraph::new(Line::from(vec![Span::styled(
                    "KITT Loader — opencode themes",
                    title_style,
                )])),
                rows[0],
            );

            // Render each loader
            for (i, theme) in visible.iter().enumerate() {
                let label_row = 2 + i * 2;
                let loader_row = label_row + 1;

                frame.render_widget(
                    Paragraph::new(Line::from(vec![Span::styled(theme.name(), dim)])),
                    rows[label_row],
                );
                frame.render_widget(&loaders[start + i], rows[loader_row]);
            }

            // Footer
            let footer_row = 2 + visible_count * 2 + 1;
            frame.render_widget(
                Paragraph::new(Line::from(vec![Span::styled(
                    format!(
                        "page {}/{} — left/right to navigate, q to quit",
                        page + 1,
                        total_pages,
                    ),
                    dim,
                )])),
                rows[footer_row],
            );
        })?;

        if event::poll(Duration::from_millis(40))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Right | KeyCode::Char('l') => {
                        if page + 1 < total_pages {
                            page += 1;
                        }
                    }
                    KeyCode::Left | KeyCode::Char('h') => {
                        page = page.saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }

        for loader in &mut loaders {
            loader.tick();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
