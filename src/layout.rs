use std::cmp::max;

use unicode_width::UnicodeWidthStr;

use crate::parser::{Edge, Participant, SequenceDiagram};

#[derive(Debug)]
pub struct ParticipantLayout {
    pub name: String,
    pub center_x: usize,
    pub top_box_y: usize,
    pub bottom_box_y: usize,
    pub width: usize,
}

#[derive(Debug)]
pub enum ArrowDirection {
    Left,
    Right,
}

#[derive(Debug)]
pub struct EdgeLayout {
    pub start_x: usize,
    pub end_x: usize,
    pub y: usize,
    pub direction: ArrowDirection,
    pub message: Option<String>,
}

#[derive(Debug)]
pub struct LifelineLayout {
    pub x: usize,
    pub start_y: usize,
    pub end_y: usize,
}

#[derive(Debug)]
pub struct SequenceDiagramLayout {
    pub participant_layouts: Vec<ParticipantLayout>,
    pub edge_layouts: Vec<EdgeLayout>,
    pub lifeline_layouts: Vec<LifelineLayout>,
    pub width: usize,
    pub height: usize,
}

pub const EDGE_SPACING: usize = 1;
pub const PARTICIPANT_HEIGHT: usize = 3;
pub const PARTICIPANT_PADDING_X: usize = 1;
pub const MESSAGE_PADDING_X: usize = 1;
pub const BORDER_WIDTH: usize = 1;

pub const MARGIN_LEFT: usize = 1;
pub const MARGIN_RIGHT: usize = 1;
pub const MARGIN_TOP: usize = 1;
pub const MARGIN_BOTTOM: usize = 1;

pub fn calculate_sequence_layout(sequence_diagram: &SequenceDiagram) -> SequenceDiagramLayout {
    let (edges_with_message, edges_without_message) = count_edges(sequence_diagram);

    let total_height = (edges_with_message + edges_without_message + 1) * EDGE_SPACING
        + edges_with_message * 2
        + edges_without_message * 1
        + PARTICIPANT_HEIGHT * 2
        + MARGIN_TOP
        + MARGIN_BOTTOM;

    let positions = calculate_horizontal_positions(sequence_diagram);

    let last_part_position = positions.last().copied().unwrap_or(0);
    let last_part_width = sequence_diagram
        .participants
        .last()
        .map(|p| p.width())
        .unwrap_or(0)
        + PARTICIPANT_PADDING_X * 2
        + BORDER_WIDTH * 2;

    // Plus 1 because of 0-base index. width = index of last column + 1
    let total_width = last_part_position + last_part_width / 2 + MARGIN_RIGHT + 1;

    let part_layouts = calculate_participant_layouts(total_height, sequence_diagram, &positions);
    let lifeline_layouts = calculate_lifeline_layouts(total_height, &positions);
    let edge_layouts = calculate_edge_layouts(sequence_diagram, &positions);

    SequenceDiagramLayout {
        edge_layouts,
        lifeline_layouts,
        participant_layouts: part_layouts,
        width: total_width,
        height: total_height,
    }
}

fn count_edges(sequence_diagram: &SequenceDiagram) -> (usize, usize) {
    let (edges_with_message, edges_without_message) =
        sequence_diagram
            .edges
            .iter()
            .fold((0, 0), |(with, without), edge| {
                if edge.message.is_some() {
                    (with + 1, without)
                } else {
                    (with, without + 1)
                }
            });

    (edges_with_message, edges_without_message)
}

fn calculate_horizontal_positions(sequence_diagram: &SequenceDiagram) -> Vec<usize> {
    let parts = &sequence_diagram.participants;

    let mut horizontal_positions = Vec::new();

    // Minus 1 because of 0-base index. The position of the left margin should be at 0, not at 1
    let mut current_position = MARGIN_LEFT - 1;

    if let Some(name) = parts.get(0) {
        current_position += BORDER_WIDTH + PARTICIPANT_PADDING_X + name.width() / 2;
        horizontal_positions.push(current_position);
    }

    for i in 1..parts.len() {
        let left_part = &parts[i - 1];
        let right_part = &parts[i];

        let space_without_message = left_part.width() / 2
            + (2 * PARTICIPANT_PADDING_X)
            + (2 * BORDER_WIDTH)
            + (right_part.width() + 1) / 2; // Round up

        let space_with_message = max_edge_width(&sequence_diagram.edges, left_part, right_part);

        let space = max(space_without_message, space_with_message + 1); // Plus 1 for space_with_message because it does not include position of next participant

        current_position += space;
        horizontal_positions.push(current_position);
    }

    horizontal_positions
}

fn max_edge_width(edges: &Vec<Edge>, part1: &Participant, part2: &Participant) -> usize {
    let mut max_width = 0;

    for edge in edges {
        if (&edge.from == part1 && &edge.to == part2) || (&edge.from == part2 && &edge.to == part1)
        {
            if let Some(msg) = &edge.message {
                max_width = max(max_width, msg.width() + MESSAGE_PADDING_X * 2);
            }
        }
    }

    max_width
}

fn calculate_participant_layouts(
    total_height: usize,
    sequence_diagram: &SequenceDiagram,
    positions: &Vec<usize>,
) -> Vec<ParticipantLayout> {
    let mut part_layouts = Vec::new();

    for (index, name) in sequence_diagram.participants.iter().enumerate() {
        let center_x = positions[index];

        part_layouts.push(ParticipantLayout {
            name: name.clone(),
            center_x,
            top_box_y: MARGIN_TOP,
            bottom_box_y: total_height - MARGIN_BOTTOM,
            width: name.width() + PARTICIPANT_PADDING_X * 2 + BORDER_WIDTH * 2,
        });
    }

    part_layouts
}

fn calculate_edge_layouts(
    sequence_diagram: &SequenceDiagram,
    positions: &Vec<usize>,
) -> Vec<EdgeLayout> {
    let mut edge_layouts = Vec::new();
    let mut current_y = MARGIN_TOP + PARTICIPANT_HEIGHT + EDGE_SPACING;

    for edge in &sequence_diagram.edges {
        let from_part = &edge.from;
        let to_part = &edge.to;

        let from_index = sequence_diagram
            .participants
            .iter()
            .position(|p| p == from_part)
            .unwrap();
        let to_index = sequence_diagram
            .participants
            .iter()
            .position(|p| p == to_part)
            .unwrap();

        let arrow_direction = if from_index < to_index {
            ArrowDirection::Right
        } else {
            ArrowDirection::Left
        };

        let (start_x, end_x) = match arrow_direction {
            ArrowDirection::Right => (positions[from_index] + 1, positions[to_index] - 1),
            ArrowDirection::Left => (positions[from_index] - 1, positions[to_index] + 1),
        };

        edge_layouts.push(EdgeLayout {
            start_x,
            end_x,
            y: current_y,
            direction: arrow_direction,
            message: edge.message.clone(),
        });

        current_y += EDGE_SPACING + 1;
        if edge.message.is_some() {
            current_y += 1;
        }
    }

    edge_layouts
}

fn calculate_lifeline_layouts(total_height: usize, positions: &Vec<usize>) -> Vec<LifelineLayout> {
    let mut lifeline_layouts = Vec::new();

    for &position in positions {
        lifeline_layouts.push(LifelineLayout {
            start_y: MARGIN_TOP + PARTICIPANT_HEIGHT,
            end_y: total_height - MARGIN_BOTTOM - PARTICIPANT_HEIGHT - EDGE_SPACING,
            x: position,
        });
    }

    lifeline_layouts
}
