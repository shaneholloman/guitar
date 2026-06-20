use crate::{core::chunk::LaneRef, helpers::colors::ColorPicker};
use ratatui::{
    style::{Color, Style},
    text::Span,
};

#[derive(Clone)]
struct LayerToken {
    symbol: String,
    color: Color,
}

// Small facade used by the graph renderer to collect symbols per visual layer.
#[derive(Clone)]
pub struct LayersContext {
    commits: Vec<LayerToken>,
    merges: Vec<LayerToken>,
    pipes: Vec<LayerToken>,
    color: ColorPicker,
    flattened_lanes: Vec<bool>,
}

impl LayersContext {
    pub fn new(color: ColorPicker) -> Self {
        Self { commits: Vec::new(), merges: Vec::new(), pipes: Vec::new(), color, flattened_lanes: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.commits.clear();
        self.merges.clear();
        self.pipes.clear();
        self.flattened_lanes.clear();
    }

    pub fn reserve(&mut self, additional: usize) {
        self.commits.reserve(additional);
        self.merges.reserve(additional);
        self.pipes.reserve(additional);
    }

    pub fn set_flattened_lanes(&mut self, flattened_lanes: Vec<bool>) {
        self.flattened_lanes = flattened_lanes;
    }

    pub fn commit(&mut self, sym: &str, lane: usize) {
        self.commit_ref(sym, self.lane_ref_for_index(lane));
    }

    pub fn commit_ref(&mut self, sym: &str, lane: LaneRef) {
        let color = self.color.get_lane_ref(lane);
        self.commits.push(LayerToken { symbol: sym.to_string(), color });
    }

    pub fn commit_at(&mut self, token_index: usize, sym: &str, lane: usize) {
        while self.commits.len() <= token_index {
            self.commits.push(LayerToken { symbol: " ".to_string(), color: Color::Black });
        }

        let color = self.color.get_lane_ref(self.lane_ref_for_index(lane));
        self.commits[token_index] = LayerToken { symbol: sym.to_string(), color };
    }

    pub fn pipe(&mut self, sym: &str, lane: usize) {
        self.pipe_ref(sym, self.lane_ref_for_index(lane));
    }

    pub fn pipe_ref(&mut self, sym: &str, lane: LaneRef) {
        let color = self.color.get_lane_ref(lane);
        self.pipes.push(LayerToken { symbol: sym.to_string(), color });
    }

    pub fn merge(&mut self, sym: &str, lane: usize) {
        self.merge_ref(sym, self.lane_ref_for_index(lane));
    }

    pub fn merge_ref(&mut self, sym: &str, lane: LaneRef) {
        let color = self.color.get_lane_ref(lane);
        self.merges.push(LayerToken { symbol: sym.to_string(), color });
    }

    pub fn merge_at(&mut self, token_index: usize, sym: &str, lane: usize) {
        self.merge_at_ref(token_index, sym, self.lane_ref_for_index(lane));
    }

    pub fn merge_at_ref(&mut self, token_index: usize, sym: &str, lane: LaneRef) {
        while self.merges.len() <= token_index {
            self.merges.push(LayerToken { symbol: " ".to_string(), color: Color::Black });
        }

        if is_empty(&self.merges[token_index].symbol) {
            let color = self.color.get_lane_ref(lane);
            self.merges[token_index] = LayerToken { symbol: sym.to_string(), color };
        }
    }

    pub fn pipe_custom(&mut self, sym: &str, _lane: usize, color: Color) {
        self.pipes.push(LayerToken { symbol: sym.to_string(), color });
    }

    fn lane_ref_for_index(&self, lane: usize) -> LaneRef {
        LaneRef::new(lane, self.flattened_lanes.get(lane).copied().unwrap_or(false))
    }

    pub fn bake(&mut self, spans: &mut Vec<Span<'static>>) {
        trim_empty(&mut self.commits);
        trim_empty(&mut self.merges);
        trim_empty(&mut self.pipes);

        // Composite up to the widest layer so sparse merge lines still render.
        let max_len = self.commits.len().max(self.merges.len()).max(self.pipes.len());

        for token_index in 0..max_len {
            let token = self
                .commits
                .get(token_index)
                .filter(|token| !is_empty(&token.symbol))
                .or_else(|| self.merges.get(token_index).filter(|token| !is_empty(&token.symbol)))
                .or_else(|| self.pipes.get(token_index).filter(|token| !is_empty(&token.symbol)));

            let (symbol, color) = token.map(|token| (token.symbol.clone(), token.color)).unwrap_or_else(|| (" ".to_string(), Color::Black));
            spans.push(Span::styled(symbol, Style::default().fg(color)));
        }
    }
}

fn trim_empty(tokens: &mut Vec<LayerToken>) {
    while tokens.last().is_some_and(|token| is_empty(&token.symbol)) {
        tokens.pop();
    }
}

fn is_empty(symbol: &str) -> bool {
    symbol.trim().is_empty()
}
