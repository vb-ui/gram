use std::collections::HashSet;

pub type Node = String;

#[derive(Debug, PartialEq)]
pub struct Edge {
    pub from: Node,
    pub to: Node,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashSet<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug)]
pub struct ParseError {
    line: usize,
    message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error at line {}: {}", self.line, self.message)
    }
}

pub fn parse(input: &str) -> Result<Graph, ParseError> {
    let mut nodes = HashSet::new();
    let mut edges = Vec::new();

    for (index, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let line_number = index + 1;

        if let Some((from_node, to_node)) = line.split_once("->") {
            let from_node = from_node.trim();
            let to_node = to_node.trim();

            validate_node(line_number, &from_node)?;
            validate_node(line_number, &to_node)?;

            nodes.insert(from_node.to_string());
            nodes.insert(to_node.to_string());
            edges.push(Edge {
                from: from_node.to_string(),
                to: to_node.to_string(),
            });
        } else {
            return Err(ParseError {
                line: line_number,
                message: format!("Invalid format, expected 'from -> to', found: '{}'", line),
            });
        }
    }

    Ok(Graph { nodes, edges })
}

pub fn validate_node(line_number: usize, name: &str) -> Result<(), ParseError> {
    if name.is_empty() {
        return Err(ParseError {
            line: line_number,
            message: "Node name cannot be empty".to_string(),
        });
    }

    if name.len() > 80 {
        return Err(ParseError {
            line: line_number,
            message: "Node name too long. Max 80 chars".to_string(),
        });
    }

    if name.contains("->") {
        // TODO: Currently, dont allow multiple edges on same line. Fix this later.
        return Err(ParseError {
            line: line_number,
            message: "Node name cannot contain '->'".to_string(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_whitespaces() {
        let input = "
cpu                  ->    \t   control-unit          \n
cpu                  ->    \t   alu                   \n
cpu                  ->    \t   registers             \n
cpu                  ->    \t   cache                 \n
control-unit         ->    \t   decoder               \n
control-unit         ->    \t   registers             \n
alu                  ->    \t   registers             \n
cache                ->    \t   bus                   \n
decoder              ->    \t   instruction-register  \n
instruction-register ->    \t   registers             \n
memory               ->    \t   bus                   \n
registers            ->    \t   bus                   \n";

        let graph = parse(input).unwrap();
        let expected_nodes: HashSet<String> = [
            "cpu",
            "control-unit",
            "alu",
            "registers",
            "cache",
            "decoder",
            "bus",
            "instruction-register",
            "memory",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let expected_edges = vec![
            Edge {
                from: "cpu".to_string(),
                to: "control-unit".to_string(),
            },
            Edge {
                from: "cpu".to_string(),
                to: "alu".to_string(),
            },
            Edge {
                from: "cpu".to_string(),
                to: "registers".to_string(),
            },
            Edge {
                from: "cpu".to_string(),
                to: "cache".to_string(),
            },
            Edge {
                from: "control-unit".to_string(),
                to: "decoder".to_string(),
            },
            Edge {
                from: "control-unit".to_string(),
                to: "registers".to_string(),
            },
            Edge {
                from: "alu".to_string(),
                to: "registers".to_string(),
            },
            Edge {
                from: "cache".to_string(),
                to: "bus".to_string(),
            },
            Edge {
                from: "decoder".to_string(),
                to: "instruction-register".to_string(),
            },
            Edge {
                from: "instruction-register".to_string(),
                to: "registers".to_string(),
            },
            Edge {
                from: "memory".to_string(),
                to: "bus".to_string(),
            },
            Edge {
                from: "registers".to_string(),
                to: "bus".to_string(),
            },
        ];

        assert_eq!(graph.nodes, expected_nodes);
        assert_eq!(graph.edges, expected_edges);
    }

    #[test]
    fn test_empty_node() {
        let input = "\
cpu -> control-unit
cpu -> 
cpu -> registers
cpu -> cache
control-unit -> decoder
control-unit -> registers
alu -> registers
cache -> bus
decoder -> instruction-register
instruction-register -> registers
memory -> bus
registers -> bus";
        let result = parse(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.line, 2);
        assert!(err.message.contains("Node name cannot be empty"));
    }

    #[test]
    fn test_missing_arrow() {
        let input = "\
cpu -> control-unit
cpu alu
cpu -> registers
cpu -> cache
control-unit -> decoder
control-unit -> registers
alu -> registers
cache -> bus
decoder -> instruction-register
instruction-register -> registers
memory -> bus
registers -> bus";
        let result = parse(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.line, 2);
        assert!(
            err.message
                .contains("Invalid format, expected 'from -> to'")
        );
    }

    #[test]
    fn test_multiplae_arrows_on_same_line() {
        let input = "\
cpu -> control-unit
cpu -> alu
cpu -> registers
cpu -> cache
control-unit -> decoder
control-unit -> registers
alu -> registers
cache -> bus
decoder -> instruction-register -> registers
memory -> bus
registers -> bus";
        let result = parse(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.line, 9);
        assert!(err.message.contains("Node name cannot contain '->'"));
    }
}
