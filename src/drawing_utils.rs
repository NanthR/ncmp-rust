use crate::utils::fmt_time;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::Frame,
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table,
        TableState,
    },
};

pub fn render_header<'a>(
    chunk: Rect,
    rect: &mut Frame<CrosstermBackend<Stdout>>,
    title: &'a str,
    volume: i8,
) {
    let header = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)].as_ref())
        .split(chunk);

    rect.render_widget(create_title(title, true, Alignment::Left, true), header[0]);
    rect.render_widget(
        create_title(
            &format!("Volume: {}%", volume),
            true,
            Alignment::Right,
            false,
        ),
        header[1],
    );
}

pub fn create_title<'a>(
    title: &'a str,
    is_border: bool,
    align: Alignment,
    is_bold: bool,
) -> Paragraph<'a> {
    let border = if is_border {
        Borders::BOTTOM
    } else {
        Borders::NONE
    };

    let modif = if is_bold {
        Modifier::BOLD
    } else {
        Modifier::empty()
    };

    Paragraph::new(title)
        .style(Style::default().add_modifier(modif))
        .alignment(align)
        .block(
            Block::default()
                .borders(border)
                .style(Style::default().fg(Color::White)),
        )
}

pub fn render_songs<'a>(queue: &Vec<mpd::Song>) -> Table<'a> {
    let heading_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);
    let heading = Row::new(vec![
        Cell::from(Span::styled("Artist", heading_style)),
        Cell::from(Span::styled("Track", heading_style)),
        Cell::from(Span::styled("Title", heading_style)),
        Cell::from(Span::styled("Album", heading_style)),
        Cell::from(Span::styled("Time", heading_style)),
    ])
    .bottom_margin(1);
    let items: Vec<_> = queue
        .iter()
        .map(move |i| {
            let title = match &i.title {
                Some(x) => x.clone(),
                None => {
                    let temp = i.file.split("/").last();
                    if temp.is_none() {
                        "".to_string()
                    } else {
                        temp.unwrap().to_string()
                    }
                }
            };
            let artist = match i.tags.get("Artist") {
                Some(x) => x.clone(),
                None => "<empty>".to_string(),
            };
            let track = match i.tags.get("Track") {
                Some(x) => x.clone(),
                None => "".to_string(),
            };
            let album = match i.tags.get("Album") {
                Some(x) => x.clone(),
                None => "<empty>".to_string(),
            };
            let time = match i.duration {
                Some(x) => fmt_time(x.num_seconds()).1,
                None => "".to_string(),
            };
            Row::new(
                vec![
                    Cell::from(Span::styled(artist, Style::default().fg(Color::Yellow))),
                    Cell::from(Span::styled(track, Style::default().fg(Color::Green))),
                    Cell::from(Span::styled(title, Style::default().fg(Color::Gray))),
                    Cell::from(Span::styled(album, Style::default().fg(Color::Cyan))),
                    Cell::from(Span::styled(time, Style::default().fg(Color::Magenta))),
                ]
                .into_iter(),
            )
        })
        .collect();
    Table::new(items.into_iter())
        .block(Block::default())
        .widths(&[
            Constraint::Percentage(20),
            Constraint::Percentage(10),
            Constraint::Percentage(50),
            Constraint::Percentage(15),
            Constraint::Percentage(5),
        ])
        .style(Style::default().fg(Color::White))
        .header(heading)
        .highlight_style(Style::default().bg(Color::Gray).fg(Color::Black))
}
