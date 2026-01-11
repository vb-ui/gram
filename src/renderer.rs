use unicode_width::UnicodeWidthStr;

use crate::layout::{
    ArrowDirection, EdgeLayout, LifelineLayout, PARTICIPANT_HEIGHT, ParticipantLayout,
    SequenceDiagramLayout,
};

#[derive(Debug)]
pub struct Canvas {
    pub grid: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![' '; width]; height];
        Canvas {
            grid,
            width: width,
            height: height,
        }
    }

    pub fn set_char(&mut self, x: usize, y: usize, ch: char) {
        if y < self.height && x < self.width {
            self.grid[y][x] = ch;
        } else {
            panic!("Index out of range.")
        }
    }

    pub fn get_char(&self, x: usize, y: usize) -> char {
        if y < self.height && x < self.width {
            self.grid[y][x]
        } else {
            panic!("Index out of range.")
        }
    }

    pub fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub fn render(seq_diagram_layout: &SequenceDiagramLayout) -> String {
    let mut canvas = Canvas::new(seq_diagram_layout.width, seq_diagram_layout.height);

    for participant_layout in &seq_diagram_layout.participant_layouts {
        draw_participant_boxes(&mut canvas, participant_layout);
    }
    for lifeline_layout in &seq_diagram_layout.lifeline_layouts {
        draw_lifeline(&mut canvas, lifeline_layout);
    }
    for edge_layout in &seq_diagram_layout.edge_layouts {
        draw_edge(&mut canvas, edge_layout);
    }

    canvas.to_string()
}

fn draw_participant_boxes(canvas: &mut Canvas, participant_layout: &ParticipantLayout) {
    let half_width = (participant_layout.width + 1) / 2;

    let center_x = participant_layout.center_x;
    let left_x = center_x - half_width + 1;
    let right_x = left_x + participant_layout.width - 1;

    draw_box(
        canvas,
        center_x,
        left_x,
        right_x,
        participant_layout.top_box_y,
        participant_layout.name.clone(),
        true,
    );

    draw_box(
        canvas,
        center_x,
        left_x,
        right_x,
        participant_layout.bottom_box_y - PARTICIPANT_HEIGHT,
        participant_layout.name.clone(),
        false,
    );
}

fn draw_box(
    canvas: &mut Canvas,
    center_x: usize,
    left_x: usize,
    right_x: usize,
    y: usize,
    name: String,
    is_top_box: bool,
) {
    // Top border
    canvas.set_char(left_x, y, '┌');
    for x in left_x + 1..right_x {
        canvas.set_char(x, y, '─');
    }
    canvas.set_char(right_x, y, '┐');

    // Middle line
    canvas.set_char(left_x, y + 1, '│');
    let name_start_x = center_x - (name.width() - 1) / 2;
    for (i, ch) in name.chars().enumerate() {
        canvas.set_char(name_start_x + i, y + 1, ch);
    }
    canvas.set_char(right_x, y + 1, '│');

    // Bottom border
    canvas.set_char(left_x, y + 2, '└');
    for x in left_x + 1..right_x {
        canvas.set_char(x, y + 2, '─');
    }
    canvas.set_char(right_x, y + 2, '┘');

    if is_top_box {
        canvas.set_char(center_x, y + 2, '┬');
    } else {
        canvas.set_char(center_x, y, '┴');
    }
}

fn draw_lifeline(canvas: &mut Canvas, lifeline_layout: &LifelineLayout) {
    for y in lifeline_layout.start_y..=lifeline_layout.end_y {
        canvas.set_char(lifeline_layout.x, y, '│');
    }
}

fn draw_edge(canvas: &mut Canvas, edge_layout: &EdgeLayout) {
    // Swap (start_x, end_x) if this edge is right to left, make sure start_x always smaller than end_x
    let (start_x, end_x, arrow_head) = match edge_layout.direction {
        ArrowDirection::Right => (edge_layout.start_x, edge_layout.end_x, '>'),
        ArrowDirection::Left => (edge_layout.end_x, edge_layout.start_x, '<'),
    };

    let edge_y: usize = if edge_layout.message.is_some() {
        edge_layout.y + 1
    } else {
        edge_layout.y
    };

    for x in start_x..=end_x {
        canvas.set_char(x, edge_y, '─');
    }

    let arrowhead_x: usize = match edge_layout.direction {
        ArrowDirection::Right => end_x,
        ArrowDirection::Left => start_x,
    };
    canvas.set_char(arrowhead_x, edge_y, arrow_head);

    if let Some(msg) = &edge_layout.message {
        let message_start_x = (start_x + end_x) / 2 - msg.width() / 2;
        let message_y = edge_layout.y;

        for (i, ch) in msg.chars().enumerate() {
            canvas.set_char(message_start_x + i, message_y, ch);
        }
    }
}
