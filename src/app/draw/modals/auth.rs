use crate::{
    app::{
        app::{App, AuthInputField},
        draw::buffered::DrawTarget,
    },
    git::auth::AuthProtocol,
    helpers::text::{truncate_start_with_ellipsis, wrap_words},
};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

impl App {
    pub fn draw_modal_network_progress(&mut self, frame: &mut impl DrawTarget) {
        let max_modal_width = (frame.area().width as f32 * 0.8) as usize;
        let text_width = max_modal_width.saturating_sub(10).clamp(1, 70);
        let wrapped_message = wrap_words(self.modal_network_message.clone(), text_width);
        let mut lines = Vec::new();
        lines.push(Line::default());
        lines.extend(wrapped_message.into_iter().map(|line| Line::from(Span::styled(line, Style::default().fg(self.theme.COLOR_TEXT)))));
        lines.push(Line::default());
        lines.push(Line::from(Span::styled("working...", Style::default().fg(self.theme.COLOR_HIGHLIGHTED))));

        self.draw_auth_text_modal(frame, &self.modal_network_title.clone(), lines, self.theme.COLOR_BORDER);
    }

    pub fn draw_modal_auth(&mut self, frame: &mut impl DrawTarget) {
        let Some(challenge) = self.pending_auth_prompt.clone() else {
            return;
        };

        let modal_width = 72.min((frame.area().width as f32 * 0.86) as usize).max(34) as u16;
        let modal_height = match challenge.protocol {
            AuthProtocol::Https | AuthProtocol::Http => 17,
            AuthProtocol::Ssh => 16,
            _ => 14,
        }
        .min((frame.area().height as f32 * 0.8) as u16)
        .max(10);
        let x = frame.area().x + (frame.area().width.saturating_sub(modal_width)) / 2;
        let y = frame.area().y + (frame.area().height.saturating_sub(modal_height)) / 2;
        let modal_area = Rect::new(x, y, modal_width, modal_height);

        let bg_block = Block::default().style(Style::default().fg(self.theme.COLOR_BORDER));
        bg_block.render(frame.area(), frame.buffer_mut());
        self.theme.clear_area(modal_area, frame.buffer_mut());

        let title = challenge.title();
        let modal_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.COLOR_ORANGE))
            .title(Span::styled(format!(" {title} "), Style::default().fg(self.theme.COLOR_TEXT)))
            .title_alignment(Alignment::Center)
            .padding(ratatui::widgets::Padding { left: 3, right: 3, top: 1, bottom: 1 })
            .border_type(ratatui::widgets::BorderType::Rounded);

        let inner_width = modal_width.saturating_sub(8) as usize;
        let mut lines = Vec::new();
        lines.push(Line::from(Span::styled(challenge.operation.clone(), Style::default().fg(self.theme.COLOR_HIGHLIGHTED))));
        lines.push(Line::default());
        lines.push(Line::from(Span::styled(truncate_start_with_ellipsis(&challenge.url, inner_width), Style::default().fg(self.theme.COLOR_TEXT))));

        if challenge.protocol == AuthProtocol::Ssh {
            lines.push(Line::default());
            if let Some(username) = &challenge.username {
                lines.push(Line::from(Span::styled(format!("user: {username}"), Style::default().fg(self.theme.COLOR_GREY_600))));
            }
            if let Some(path) = &challenge.key_path {
                lines.push(Line::from(Span::styled(
                    format!("key: {}", truncate_start_with_ellipsis(&path.display().to_string(), inner_width.saturating_sub(5))),
                    Style::default().fg(self.theme.COLOR_GREY_600),
                )));
            }
        }

        lines.push(Line::default());
        lines.push(Line::from(Span::styled("Tab switches fields   Enter submits   Esc cancels", Style::default().fg(self.theme.COLOR_GREY_600))));
        Paragraph::new(Text::from(lines)).block(modal_block).alignment(Alignment::Center).render(modal_area, frame.buffer_mut());

        let field_width = modal_width.saturating_sub(14);
        let field_x = modal_area.x + 7;
        let first_field_y = modal_area.y + if challenge.protocol.is_http() { 8 } else { 9 };
        if challenge.protocol.is_http() {
            self.draw_auth_field(frame, Rect::new(field_x, first_field_y, field_width, 3), "username", AuthInputField::Username, false);
            self.draw_auth_field(frame, Rect::new(field_x, first_field_y + 4, field_width, 3), "password / token", AuthInputField::Secret, true);
        } else {
            self.draw_auth_field(frame, Rect::new(field_x, first_field_y, field_width, 3), "passphrase", AuthInputField::Secret, true);
        }
    }

    fn draw_auth_field(&mut self, frame: &mut impl DrawTarget, area: Rect, label: &str, field: AuthInputField, masked: bool) {
        let active = self.auth_input_field == field;
        let input = if field == AuthInputField::Username { &mut self.auth_username_input } else { &mut self.auth_secret_input };
        let visible_width = area.width.saturating_sub(4) as usize;
        input.set_max_width(visible_width);
        let start = *input.scroll();
        let end = (start + visible_width).min(input.value().len());
        let value = if masked { "*".repeat(input.value().chars().count()) } else { input.value().to_string() };
        let visible = if masked {
            let start = start.min(value.len());
            let end = end.min(value.len());
            value[start..end].to_string()
        } else {
            input.value()[start..end].to_string()
        };

        let border = if active { self.theme.COLOR_HIGHLIGHTED } else { self.theme.COLOR_GREY_800 };
        let label_style = if active { Style::default().fg(self.theme.COLOR_HIGHLIGHTED).add_modifier(Modifier::BOLD) } else { Style::default().fg(self.theme.COLOR_GREY_600) };
        let block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(border)).title(Span::styled(format!(" {label} "), label_style));

        Paragraph::new(Line::from(Span::styled(visible, Style::default().fg(self.theme.COLOR_TEXT)))).block(block).render(area, frame.buffer_mut());

        if active {
            let cursor_x = (input.cursor().saturating_sub(*input.scroll()) as u16).min(area.width.saturating_sub(3));
            frame.set_cursor_position((area.x + 1 + cursor_x, area.y + 1));
        }
    }

    fn draw_auth_text_modal(&mut self, frame: &mut impl DrawTarget, title: &str, lines: Vec<Line>, border_color: ratatui::style::Color) {
        let max_modal_width = (frame.area().width as f32 * 0.8) as usize;
        let content_width = lines.iter().map(|line| line.width()).max().unwrap_or(0);
        let modal_width = (content_width + 10).max(34).min(max_modal_width) as u16;
        let max_modal_height = (frame.area().height as f32 * 0.6) as usize;
        let modal_height = (lines.len() + 4).max(8).min(max_modal_height.max(1)) as u16;
        let x = frame.area().x + (frame.area().width.saturating_sub(modal_width)) / 2;
        let y = frame.area().y + (frame.area().height.saturating_sub(modal_height)) / 2;
        let modal_area = Rect::new(x, y, modal_width, modal_height);

        self.theme.clear_area(modal_area, frame.buffer_mut());

        let modal_block = Block::default()
            .title(Span::styled(format!(" {title} "), Style::default().fg(self.theme.COLOR_TEXT)))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(border_color));

        Paragraph::new(Text::from(lines)).block(modal_block).alignment(Alignment::Center).render(modal_area, frame.buffer_mut());
    }
}

#[cfg(test)]
#[path = "../../../tests/app/draw/modals/auth.rs"]
mod tests;
