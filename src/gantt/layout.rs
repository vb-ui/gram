use crate::gantt::parser::GanttChart;
use chrono::{NaiveDate, TimeDelta};
use num_rational::Ratio;
use std::cmp::{max, min};

#[derive(Debug)]
pub struct TaskLayout {
    pub x_start: usize,
    pub x_end: usize,
    pub y: usize,
    pub name: String,
}

#[derive(Debug)]
pub struct TickLayout {
    pub x: usize,
    pub date: NaiveDate,
}

#[derive(Debug)]
pub struct GanttLayout {
    pub task_layouts: Vec<TaskLayout>,
    pub tick_layouts: Vec<TickLayout>,
    pub min_date: NaiveDate,
    pub max_date: NaiveDate,
    pub width: usize,
    pub height: usize,
}

pub const MARGIN_LEFT: usize = 6;
pub const MARGIN_RIGHT: usize = 6;
pub const MARGIN_TOP: usize = 2;
pub const MARGIN_BOTTOM: usize = 2;

pub const CHART_WIDTH: usize = 120;
pub const TASK_HEIGHT: usize = 3;
pub const MIN_TICK_SPACING: usize = 10;

pub fn layout(gantt_chart: &GanttChart) -> GanttLayout {
    let (min_date, max_date) = find_date_range(gantt_chart);
    let total_days = (max_date - min_date).num_days() as usize;

    // How many pixels (char columns) represent one day
    let pixels_per_day = Ratio::new(CHART_WIDTH, total_days);

    let task_layouts = layout_tasks(gantt_chart, min_date, pixels_per_day);
    let tick_layouts = layout_ticks(min_date, total_days);

    let height = TASK_HEIGHT * gantt_chart.tasks.len() + MARGIN_TOP + MARGIN_BOTTOM;
    let width = CHART_WIDTH + MARGIN_LEFT + MARGIN_RIGHT;

    GanttLayout {
        task_layouts,
        tick_layouts,
        min_date,
        max_date,
        width,
        height,
    }
}

fn layout_tasks(
    gantt_chart: &GanttChart,
    min_date: NaiveDate,
    pixels_per_day: Ratio<usize>,
) -> Vec<TaskLayout> {
    let mut task_layouts = Vec::new();
    let mut y = MARGIN_TOP;

    for task in &gantt_chart.tasks {
        let x_start = date_to_x(task.start_date, min_date, pixels_per_day) + MARGIN_LEFT;
        let x_end = date_to_x(task.end_date, min_date, pixels_per_day) + MARGIN_LEFT;

        task_layouts.push(TaskLayout {
            x_start,
            x_end,
            y,
            name: task.name.clone(),
        });

        y += TASK_HEIGHT;
    }

    task_layouts
}

fn layout_ticks(min_date: NaiveDate, total_days: usize) -> Vec<TickLayout> {
    // TODO: Maybe need to improve this. Calculate ticks base on date range instead of fixed it.
    let ticks_count = (CHART_WIDTH / MIN_TICK_SPACING).max(2);

    let mut ticks_layout = Vec::new();
    let days_per_tick = total_days / (ticks_count - 1);
    let pixels_per_tick = CHART_WIDTH / (ticks_count - 1);

    for i in 0..ticks_count {
        ticks_layout.push(TickLayout {
            x: i * pixels_per_tick + MARGIN_LEFT,
            date: min_date + TimeDelta::days((days_per_tick * i) as i64),
        });
    }

    ticks_layout
}

fn find_date_range(chart: &GanttChart) -> (NaiveDate, NaiveDate) {
    let mut min_date = NaiveDate::MAX;
    let mut max_date = NaiveDate::MIN;

    for task in &chart.tasks {
        min_date = min(min_date, task.start_date);
        max_date = max(max_date, task.end_date);
    }

    (min_date, max_date)
}

fn date_to_x(date: NaiveDate, min_date: NaiveDate, pixels_per_day: Ratio<usize>) -> usize {
    let days = (date - min_date).num_days() as usize;
    let days = Ratio::from_integer(days);

    let pixels = days * pixels_per_day;
    pixels.to_integer()
}
