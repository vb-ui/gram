use std::collections::HashMap;

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

#[derive(Debug, PartialEq)]
pub struct Commit {
    index: usize,
    message: String,
    merged_from: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Branch {
    commits: Vec<Commit>,
    base_commit: Option<usize>,
}

pub type GitGraph = HashMap<String, Branch>;

fn init_git_graph() -> GitGraph {
    let mut git_graph = HashMap::new();

    git_graph.insert(
        String::from("main"),
        Branch {
            commits: Vec::new(),
            base_commit: None,
        },
    );

    git_graph
}

pub fn parse(input: &str) -> Result<GitGraph, ParseError> {
    let mut git_graph = init_git_graph();
    let mut current_branch_name = String::from("main");
    let mut current_index = 0;

    for (line_number, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let line_number = line_number + 1;

        let (action, rest) = line.trim().split_once(' ').ok_or(ParseError {
            line: line_number,
            message: "Invalid syntax".to_string(),
        })?;
        let action = action.trim();
        let rest = rest.trim().to_string();

        match action {
            "commit" => {
                let current_branch = git_graph
                    .get_mut(&current_branch_name)
                    .expect("Internal error. Current branch not found");

                current_branch.commits.push(Commit {
                    index: current_index,
                    message: rest,
                    merged_from: None,
                });
                current_index += 1;
            }
            "branch" => {
                let new_branch_name = rest;

                if git_graph.contains_key(&new_branch_name) {
                    return Err(ParseError {
                        line: line_number,
                        message: format!(
                            "Cannot create new branch. Branch {} already exists",
                            new_branch_name
                        ),
                    });
                }

                let base_index = git_graph
                    .get(&current_branch_name)
                    .expect("Internal error. Current branch not found")
                    .commits
                    .last()
                    .map(|commit| commit.index);

                match base_index {
                    Some(base_index) => {
                        let new_branch = Branch {
                            commits: Vec::new(),
                            base_commit: Some(base_index),
                        };
                        current_branch_name = new_branch_name.clone();
                        git_graph.insert(new_branch_name, new_branch);
                    }
                    None => {
                        return Err(ParseError {
                            line: line_number,
                            message: format!(
                                "Cannot create a new branch. Current branch ({}) has no commits yet",
                                current_branch_name
                            ),
                        });
                    }
                }
            }
            "checkout" => {
                let branch_name = rest;

                if git_graph.contains_key(&branch_name) {
                    current_branch_name = branch_name;
                } else {
                    return Err(ParseError {
                        line: line_number,
                        message: format!("Branch {} does not exist", branch_name),
                    });
                }
            }
            "merge" => {
                let target_branch_name = rest;

                if target_branch_name == current_branch_name {
                    return Err(ParseError {
                        line: line_number,
                        message: format!("Cannot merge branch {} into itself", target_branch_name),
                    });
                }
                match git_graph.get(&target_branch_name) {
                    Some(target_branch) => {
                        if target_branch.commits.is_empty() {
                            return Err(ParseError {
                                line: line_number,
                                message: format!(
                                    "Cannot merge branch {} because it has no commits",
                                    target_branch_name
                                ),
                            });
                        }
                    }
                    None => {
                        return Err(ParseError {
                            line: line_number,
                            message: format!("Branch {} does not exist", target_branch_name),
                        });
                    }
                }

                let current_branch = git_graph
                    .get_mut(&current_branch_name)
                    .expect("Internal error. Current branch not found");

                current_branch.commits.push(Commit {
                    index: current_index,
                    message: format!(
                        "Merge branch {} into branch {}",
                        target_branch_name, current_branch_name
                    ),
                    merged_from: Some(target_branch_name),
                });

                current_index += 1;
            }
            _ => {
                return Err(ParseError {
                    line: line_number,
                    message: "Invalid syntax: expected '<action> <name>'".to_string(),
                });
            }
        }
    }

    Ok(git_graph)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_perfect_input() {
        let input = "\
commit     init
commit     core

branch     dev
commit     setup
commit     config
commit     refactor

branch     feature-search
commit     ui
commit     api

checkout   dev
merge      feature-search
commit     stabilize

checkout   main
merge      dev";

        let git_graph = parse(input).unwrap();
        for branch in vec!["main", "dev", "feature-search"] {
            assert!(git_graph.contains_key(branch));
        }

        let expected_main_branch = Branch {
            base_commit: None,
            commits: vec![
                Commit {
                    index: 0,
                    message: "init".to_string(),
                    merged_from: None,
                },
                Commit {
                    index: 1,
                    message: "core".to_string(),
                    merged_from: None,
                },
                Commit {
                    index: 9,
                    message: "Merge branch dev into branch main".to_string(),
                    merged_from: Some("dev".to_string()),
                },
            ],
        };
        assert_eq!(git_graph.get("main").unwrap(), &expected_main_branch);

        let expected_dev_branch = Branch {
            commits: vec![
                Commit {
                    index: 2,
                    message: "setup".to_string(),
                    merged_from: None,
                },
                Commit {
                    index: 3,
                    message: "config".to_string(),
                    merged_from: None,
                },
                Commit {
                    index: 4,
                    message: "refactor".to_string(),
                    merged_from: None,
                },
                Commit {
                    index: 7,
                    message: "Merge branch feature-search into branch dev".to_string(),
                    merged_from: Some("feature-search".to_string()),
                },
                Commit {
                    index: 8,
                    message: "stabilize".to_string(),
                    merged_from: None,
                },
            ],
            base_commit: Some(1),
        };
        assert_eq!(git_graph.get("dev").unwrap(), &expected_dev_branch);

        let expected_featute_search_branch = Branch {
            commits: vec![
                Commit {
                    index: 5,
                    message: "ui".to_string(),
                    merged_from: None,
                },
                Commit {
                    index: 6,
                    message: "api".to_string(),
                    merged_from: None,
                },
            ],
            base_commit: Some(4),
        };

        assert_eq!(
            git_graph.get("feature-search").unwrap(),
            &expected_featute_search_branch
        );
    }
}
