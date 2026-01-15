use chrono::{NaiveDate, TimeDelta};

#[derive(Debug)]
pub struct Task {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub name: String,
}

#[derive(Debug)]
pub struct GanttChart {
    pub tasks: Vec<Task>,
}

#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error at line {}: {}", self.line, self.message)
    }
}

// TODO: Make date format configurable
const DATE_FORMAT: &str = "%d-%m-%Y";

pub fn parse(input: &str) -> Result<GanttChart, ParseError> {
    let mut tasks: Vec<Task> = Vec::new();

    for (index, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let line_number = index + 1;

        let (task_name, date_str) = line.split_once(":").ok_or(ParseError {
            line: line_number,
            message: "Missing colon. Expects format: 'Task: start_date, end_date'".to_string(),
        })?;

        let task_name = task_name.trim();
        if task_name.is_empty() {
            return Err(ParseError {
                line: line_number,
                message: "Task name cannot be empty".to_string(),
            });
        }

        let (start_date_str, end_date_str) = date_str.split_once(",").ok_or(ParseError {
            line: line_number,
            message: "Missing delimiter. Expects format: 'Task: start_date, end_date'".to_string(),
        })?;

        let start_date_str = start_date_str.trim();
        let end_date_str = end_date_str.trim();

        let start_date = match NaiveDate::parse_from_str(start_date_str, DATE_FORMAT) {
            Ok(date) => date,
            Err(_) => {
                if start_date_str != "continue" {
                    return Err(ParseError {
                        line: line_number,
                        message: format!(
                            "Invalid start date '{}'. Expected format: 'DD-MM-YYYY' or 'continue'",
                            start_date_str
                        ),
                    });
                }

                if tasks.is_empty() {
                    return Err(ParseError {
                        line: line_number,
                        message: "No previous task exists".to_string(),
                    });
                }

                let prev_task = tasks.last().unwrap();
                prev_task.end_date
            }
        };

        let end_date = match NaiveDate::parse_from_str(end_date_str, DATE_FORMAT) {
            Ok(date) => date,
            Err(_) => {
                let duration = parse_duration(line_number, end_date_str)?;
                start_date + duration
            }
        };

        if end_date < start_date {
            return Err(ParseError {
                line: line_number,
                message: "End date cannot be earlier than start date".to_string(),
            });
        }

        tasks.push(Task {
            start_date,
            end_date,
            name: task_name.to_string(),
        });
    }

    Ok(GanttChart { tasks })
}

fn parse_duration(line_number: usize, duration_str: &str) -> Result<TimeDelta, ParseError> {
    if !duration_str.ends_with('d') {
        return Err(ParseError {
            line: line_number,
            message: format!(
                "Invalid end date '{}'. Expected format: 'DD-MM-YYYY' or '<number>d'",
                duration_str
            ),
        });
    }

    let number_part = &duration_str[..duration_str.len() - 1];
    let days: i64 = number_part.parse().map_err(|_| ParseError {
        line: line_number,
        message: "Invalid number in duration".to_string(),
    })?;

    if days <= 0 {
        return Err(ParseError {
            line: line_number,
            message: "Duration must be positive".to_string(),
        });
    }

    Ok(TimeDelta::days(days))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_whitespaces_input() {
        let input = "
Design              :   01-01-2026,     05-01-2026 \t
Implementation      :   05-01-2026,     15-01-2026 \t
Testing             :   15-01-2026,     20-01-2026 \t
Bugfix              :   20-01-2026,     03-02-2026 \t
Release             :   03-02-2026,     06-02-2026 \t";

        let gantt_chart = parse(input).unwrap();
        assert_eq!(gantt_chart.tasks.len(), 5);
        assert_eq!(
            gantt_chart.tasks[0].start_date,
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[0].end_date,
            NaiveDate::from_ymd_opt(2026, 1, 5).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[1].start_date,
            NaiveDate::from_ymd_opt(2026, 1, 5).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[1].end_date,
            NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[2].start_date,
            NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[2].end_date,
            NaiveDate::from_ymd_opt(2026, 1, 20).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[3].start_date,
            NaiveDate::from_ymd_opt(2026, 1, 20).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[3].end_date,
            NaiveDate::from_ymd_opt(2026, 2, 3).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[4].start_date,
            NaiveDate::from_ymd_opt(2026, 2, 3).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[4].end_date,
            NaiveDate::from_ymd_opt(2026, 2, 6).unwrap()
        );
    }

    #[test]
    fn test_auto_start_date() {
        let input = "
Design: 01-01-2026, 05-01-2026
Implementation: continue, 15-01-2026
Testing: continue, 20-01-2026
Bugfix: continue, 03-02-2026
Release: continue, 06-02-2026";

        let gantt_chart = parse(input).unwrap();
        assert_eq!(gantt_chart.tasks.len(), 5);
        assert_eq!(
            gantt_chart.tasks[0].start_date,
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[0].end_date,
            NaiveDate::from_ymd_opt(2026, 1, 5).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[1].start_date,
            NaiveDate::from_ymd_opt(2026, 1, 5).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[1].end_date,
            NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[2].start_date,
            NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[2].end_date,
            NaiveDate::from_ymd_opt(2026, 1, 20).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[3].start_date,
            NaiveDate::from_ymd_opt(2026, 1, 20).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[3].end_date,
            NaiveDate::from_ymd_opt(2026, 2, 3).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[4].start_date,
            NaiveDate::from_ymd_opt(2026, 2, 3).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[4].end_date,
            NaiveDate::from_ymd_opt(2026, 2, 6).unwrap()
        );
    }

    #[test]
    fn test_duration() {
        let input = "
Design: 01-01-2026, 4d
Implementation: 05-01-2026, 10d
Testing: 15-01-2026, 5d
Bugfix: 20-01-2026, 14d
Release: 03-02-2026, 3d";

        let gantt_chart = parse(input).unwrap();
        assert_eq!(gantt_chart.tasks.len(), 5);
        assert_eq!(
            gantt_chart.tasks[0].start_date,
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[0].end_date,
            NaiveDate::from_ymd_opt(2026, 1, 5).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[1].start_date,
            NaiveDate::from_ymd_opt(2026, 1, 5).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[1].end_date,
            NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[2].start_date,
            NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[2].end_date,
            NaiveDate::from_ymd_opt(2026, 1, 20).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[3].start_date,
            NaiveDate::from_ymd_opt(2026, 1, 20).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[3].end_date,
            NaiveDate::from_ymd_opt(2026, 2, 3).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[4].start_date,
            NaiveDate::from_ymd_opt(2026, 2, 3).unwrap()
        );
        assert_eq!(
            gantt_chart.tasks[4].end_date,
            NaiveDate::from_ymd_opt(2026, 2, 6).unwrap()
        );
    }

    #[test]
    fn test_start_date_not_specifed() {
        let input = "
Design: continue, 05-01-2026
Implementation: continue, 15-01-2026
Testing: continue, 20-01-2026
Bugfix: continue, 03-02-2026
Release: continue, 06-02-2026";

        let gantt_chart = parse(input);
        assert!(gantt_chart.is_err());
        assert!(
            gantt_chart
                .unwrap_err()
                .message
                .contains("No previous task exists")
        );
    }

    #[test]
    fn test_incorrect_date_format() {
        let input = "
Design: 2026-01-01, 2026-05-01
Implementation: 05-01-2026, 15-01-2026
Testing: 15-01-2026, 20-01-2026
Bugfix: 20-01-2026, 03-02-2026
Release: 03-02-2026, 06-02-2026";

        let gantt_chart = parse(input);
        assert!(gantt_chart.is_err());
        assert!(
            gantt_chart
                .unwrap_err()
                .message
                .contains("Invalid start date")
        );
    }
}
