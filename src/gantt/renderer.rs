use unicode_width::UnicodeWidthStr;

use crate::gantt::layout::{GanttLayout, MARGIN_BOTTOM, MARGIN_TOP, TaskLayout, TickLayout};

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

pub fn render(gantt_layout: &GanttLayout) -> String {
    let mut canvas = Canvas::new(gantt_layout.width, gantt_layout.height);

    for tick_layout in &gantt_layout.tick_layouts {
        draw_tick(tick_layout, &mut canvas);
    }

    for task_layout in &gantt_layout.task_layouts {
        draw_task(task_layout, &mut canvas);
    }

    canvas.to_string()
}

fn draw_task(task_layout: &TaskLayout, canvas: &mut Canvas) {
    let x_start = task_layout.x_start;
    let x_end = task_layout.x_end;
    let y = task_layout.y;
    let name = &task_layout.name;
    let box_internal_width = x_end - x_start - 1;

    // Top border
    canvas.set_char(x_start, y, '┌');
    for x in x_start + 1..x_end {
        canvas.set_char(x, y, '─');
    }
    canvas.set_char(x_end, y, '┐');

    // Mid line
    canvas.set_char(x_start, y + 1, '|');
    // Remove tick lines inside the box
    for x in x_start + 1..x_end {
        canvas.set_char(x, y + 1, ' ');
    }

    let name_start_x = if name.width() > box_internal_width {
        x_end + 1
    } else {
        x_start + (box_internal_width + 1) / 2 - (name.width() - 1) / 2
    };

    for (i, ch) in name.chars().enumerate() {
        // TODO: Handle text overflow.
        if name_start_x + i >= canvas.width {
            break;
        }
        canvas.set_char(name_start_x + i, y + 1, ch);
    }
    canvas.set_char(x_end, y + 1, '|');

    // Bottom border
    canvas.set_char(x_start, y + 2, '└');
    for x in x_start + 1..x_end {
        canvas.set_char(x, y + 2, '─');
    }
    canvas.set_char(x_end, y + 2, '┘');
}

fn draw_tick(tick_layout: &TickLayout, canvas: &mut Canvas) {
    for y in MARGIN_TOP - 1..canvas.height - MARGIN_BOTTOM + 1 {
        canvas.set_char(tick_layout.x, y, '|');
    }
    let date = tick_layout.date.format("%d-%m-%Y").to_string();

    let date_start_x = tick_layout.x - date.width() / 2;

    for (i, ch) in date.chars().enumerate() {
        canvas.set_char(date_start_x + i, canvas.height - MARGIN_BOTTOM + 1, ch);
    }
}
