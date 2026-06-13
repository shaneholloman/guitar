use crate::{
    app::{
        app::{App, Focus, Viewport},
        app_default::ViewerMode,
    },
    git::queries::diffs::{get_file_at_oid, get_file_at_workdir, get_file_diff_at_oid, get_file_diff_at_workdir},
    helpers::text::wrap_words,
};
use git2::Oid;
use ratatui::{
    Frame,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

impl App {
    pub fn draw_viewer(&mut self, frame: &mut Frame) {
        // Viewer content gets horizontal padding so diff prefixes do not touch borders.
        let padding = ratatui::widgets::Padding { left: 1, right: 1, top: 0, bottom: 0 };

        // Retained for quick tuning of wrap width while working on the viewer layout.
        // let available_width = self.layout.graph.width as usize - 1;
        // let max_text_width = available_width.saturating_sub(2);

        // Hunk mode presents only changed-line anchors while reusing the full viewer rows.
        let active_lines: Vec<&ListItem> = match self.viewer_mode {
            ViewerMode::Full => self.viewer_lines.iter().collect(),
            ViewerMode::Hunks => self.viewer_hunks.iter().filter_map(|&i| self.viewer_lines.get(i)).collect(),
        };

        let total_lines = active_lines.len();
        let visible_height = if self.layout_config.is_zen { self.layout.graph.height.saturating_sub(2) as usize } else { self.layout.graph.height as usize };

        if total_lines == 0 {
            self.viewer_selected = 0;
        } else if self.viewer_selected >= total_lines {
            self.viewer_selected = total_lines.saturating_sub(1);
        }

        self.trap_selection(self.viewer_selected, &self.viewer_scroll, total_lines, visible_height);

        let start = self.viewer_scroll.get().min(total_lines.saturating_sub(visible_height));
        let end = (start + visible_height).min(total_lines);

        // Clone visible rows so selection styling does not mutate the cached viewer data.
        let list_items: Vec<ListItem> = active_lines[start..end]
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let absolute_idx = start + i;
                let mut item = (*line).clone();
                if absolute_idx == self.viewer_selected && self.focus == Focus::Viewport {
                    item = item.style(Style::default().bg(self.theme.COLOR_GREY_800));
                }
                item
            })
            .collect();

        if self.layout_config.is_zen {
            // Zen mode frames the viewer as a standalone surface.
            let list = List::new(list_items)
                .block(Block::default().padding(padding).borders(Borders::ALL).border_style(Style::default().fg(self.theme.COLOR_BORDER)).border_type(ratatui::widgets::BorderType::Rounded));

            frame.render_widget(list, self.layout.graph);

            let mut scrollbar_state = ScrollbarState::new(total_lines.saturating_sub(visible_height)).position(self.viewer_scroll.get());
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("╮"))
                .end_symbol(Some("╯"))
                .track_symbol(Some("│"))
                .thumb_symbol("▌")
                .thumb_style(Style::default().fg(if self.focus == Focus::Viewport { self.theme.COLOR_GREY_600 } else { self.theme.COLOR_BORDER }));

            frame.render_stateful_widget(scrollbar, self.layout.graph_scrollbar, &mut scrollbar_state);

            return;
        }

        // Normal mode shares the graph viewport borders with neighboring panes.
        let list = List::new(list_items).block(
            Block::default().padding(padding).borders(Borders::RIGHT | Borders::LEFT).border_style(Style::default().fg(self.theme.COLOR_BORDER)).border_type(ratatui::widgets::BorderType::Rounded),
        );

        frame.render_widget(list, self.layout.graph);

        let mut scrollbar_state = ScrollbarState::new(total_lines.saturating_sub(visible_height)).position(self.viewer_scroll.get());
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(if self.layout_config.is_inspector || self.layout_config.is_status { Some("─") } else { Some("╮") })
            .end_symbol(if self.layout_config.is_inspector || self.layout_config.is_status { Some("─") } else { Some("╯") })
            .track_symbol(Some("│"))
            .thumb_symbol("▌")
            .thumb_style(Style::default().fg(if self.focus == Focus::Viewport { self.theme.COLOR_GREY_600 } else { self.theme.COLOR_BORDER }));

        frame.render_stateful_widget(scrollbar, self.layout.graph_scrollbar, &mut scrollbar_state);
    }

    // Resolve the selected status row into a repository-relative path.
    pub fn get_selected_file_name(&self) -> Option<String> {
        match self.focus {
            Focus::StatusTop => {
                if self.graph_selected != 0 && !self.current_diff.is_empty() {
                    // Commit status rows come from current_diff.
                    Some(self.current_diff.get(self.status_top_selected)?.filename.to_string())
                } else if self.graph_selected == 0 && self.uncommitted.is_staged {
                    // Staged uncommitted rows are grouped modified, added, then deleted.
                    let modified_len = self.uncommitted.staged.modified.len();
                    let added_len = self.uncommitted.staged.added.len();
                    let index = self.status_top_selected;

                    if index < modified_len {
                        self.uncommitted.staged.modified.get(index).cloned()
                    } else if index < modified_len + added_len {
                        self.uncommitted.staged.added.get(index - modified_len).cloned()
                    } else {
                        self.uncommitted.staged.deleted.get(index - modified_len - added_len).cloned()
                    }
                } else {
                    None
                }
            },
            Focus::StatusBottom => {
                if self.graph_selected == 0 && self.uncommitted.is_unstaged {
                    // Unstaged rows use the same grouping as staged rows.
                    let modified_len = self.uncommitted.unstaged.modified.len();
                    let added_len = self.uncommitted.unstaged.added.len();
                    let index = self.status_bottom_selected;

                    if index < modified_len {
                        self.uncommitted.unstaged.modified.get(index).cloned()
                    } else if index < modified_len + added_len {
                        self.uncommitted.unstaged.added.get(index - modified_len).cloned()
                    } else {
                        self.uncommitted.unstaged.deleted.get(index - modified_len - added_len).cloned()
                    }
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    pub fn open_viewer(&mut self, repo: &git2::Repository) {
        if let Some(file_name) = self.get_selected_file_name() {
            self.file_name = Some(file_name);
            let oid = if self.graph_selected != 0 { self.oids.get_oid_by_idx(self.graph_selected) } else { &Oid::zero() };
            self.update_viewer(*oid, repo);
            self.viewport = Viewport::Viewer;
        }
    }

    pub fn update_viewer(&mut self, oid: Oid, repo: &git2::Repository) {
        // The selected filename is owned by App so viewer reloads can reuse it.
        let filename = self.file_name.clone().unwrap();

        // Oid::zero represents the uncommitted pseudo-row and reads from the working tree.
        let (original_lines, hunks) = if oid == Oid::zero() {
            (get_file_at_workdir(repo, &filename), get_file_diff_at_workdir(repo, &filename).unwrap_or_default())
        } else {
            (get_file_at_oid(repo, oid, &filename), get_file_diff_at_oid(repo, oid, &filename).unwrap_or_default())
        };

        self.viewer_lines.clear();
        self.viewer_edges.clear();
        self.viewer_hunks.clear();
        let mut current_line: usize = 0;
        let mut current_line_old: usize = 0;

        // Origin changes mark useful navigation edges inside a diff.
        let mut last_origin: Option<char> = None;

        for hunk in hunks.iter() {
            let header = &hunk.header;
            let old_start_idx: usize = header.old_start.saturating_sub(1) as usize;

            // Fill unchanged file content before the next hunk starts.
            while current_line < old_start_idx && current_line < original_lines.len() {
                let wrapped = wrap_words(original_lines[current_line].clone(), (self.layout.graph.width as usize).saturating_sub(8));
                for (idx, line) in wrapped.into_iter().enumerate() {
                    self.viewer_lines.push(ListItem::new(
                        Line::from(vec![
                            Span::styled((if idx == 0 { format!("{:3}  ", current_line + 1) } else { "     ".to_string() }).to_string(), Style::default().fg(self.theme.COLOR_BORDER)),
                            Span::styled(line.to_string(), Style::default().fg(self.theme.COLOR_GREY_500)),
                        ])
                        .style(Style::default()),
                    ));
                }
                current_line += 1;
                current_line_old += 1;
            }

            // Process patch lines after dropping hunk header marker lines.
            for line in hunk.lines.iter().filter(|l| l.origin != 'H') {
                let text = line.content.trim_end_matches('\n');

                // Store edge positions where additions, removals, and context switch.
                if let Some(prev) = last_origin
                    && prev != line.origin
                {
                    self.viewer_edges.push(self.viewer_lines.len().saturating_sub(1));
                }
                last_origin = Some(line.origin);

                // Line origin controls prefix, color, and which side's counter advances.
                let (style, prefix, side, fg, count) = match line.origin {
                    '-' => (Style::default().bg(self.theme.COLOR_DARK_RED).fg(self.theme.COLOR_RED), "- ".to_string(), self.theme.COLOR_RED, self.theme.COLOR_RED, current_line_old + 1),
                    '+' => (Style::default().bg(self.theme.COLOR_LIGHT_GREEN_900).fg(self.theme.COLOR_GREEN), "+ ".to_string(), self.theme.COLOR_GREEN, self.theme.COLOR_GREEN, current_line + 1),
                    ' ' => (Style::default(), "".to_string(), self.theme.COLOR_BORDER, self.theme.COLOR_GREY_500, current_line + 1),
                    _ => (Style::default(), "".to_string(), self.theme.COLOR_BORDER, self.theme.COLOR_GREY_500, 0),
                };

                let wrapped = wrap_words(format!("{}{}", prefix, text), (self.layout.graph.width as usize).saturating_sub(9));
                for (idx, line_wrapped) in wrapped.into_iter().enumerate() {
                    // Hunk mode indexes only changed rows.
                    if line.origin != ' ' {
                        self.viewer_hunks.push(self.viewer_lines.len());
                    }

                    self.viewer_lines.push(
                        ListItem::new(Line::from(vec![
                            Span::styled((if idx == 0 { format!("{:3}  ", count) } else { "     ".to_string() }).to_string(), Style::default().fg(side)),
                            Span::styled(line_wrapped.to_string(), Style::default().fg(fg)),
                        ]))
                        .style(style),
                    );
                }

                // Deleted lines advance only the old side, added lines only the new side.
                match line.origin {
                    '-' => {
                        current_line_old += 1;
                    },
                    '+' => {
                        current_line += 1;
                    },
                    ' ' => {
                        current_line += 1;
                        current_line_old += 1;
                    },
                    _ => {},
                }
            }
        }

        // Append unchanged file content after the final hunk.
        while current_line < original_lines.len() {
            let wrapped = wrap_words(original_lines[current_line].clone(), (self.layout.graph.width as usize).saturating_sub(8));
            for (idx, line) in wrapped.into_iter().enumerate() {
                self.viewer_lines.push(
                    ListItem::new(Line::from(vec![
                        Span::styled((if idx == 0 { format!("{:3}  ", current_line + 1) } else { "     ".to_string() }).to_string(), Style::default().fg(self.theme.COLOR_BORDER)),
                        Span::styled(line.to_string(), Style::default().fg(self.theme.COLOR_GREY_500)),
                    ]))
                    .style(Style::default()),
                );
            }
            current_line += 1;
        }

        self.viewer_selected = self.viewer_edges.first().copied().unwrap_or(0);
    }
}
