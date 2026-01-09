use std::collections::HashMap;

use crate::tokenizer::Token;

#[derive(Debug)]
pub struct Edge {
    pub to: String,
    pub message: Option<String>,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<String>,
    // TODO: Adjacent list is not a good way to represent the graph. It's not optimized for the layout calculation.
    pub adjacency: HashMap<String, Vec<Edge>>,
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser error: {}", self.message)
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Graph, ParseError> {
    let mut nodes: Vec<String> = Vec::new();
    let mut adjacency: HashMap<String, Vec<Edge>> = HashMap::new();
    let mut i = 0;

    while i < tokens.len() {
        if i + 2 >= tokens.len() {
            return Err(ParseError {
                message: "Incomplete edge. Expected at least 3 tokens".to_string(),
            });
        }

        let first_participant = match &tokens[i] {
            Token::Participant(name) => name.clone(),
            _ => {
                return Err(ParseError {
                    message: "Expected participant".to_string(),
                });
            }
        };

        let second_participant = match &tokens[i + 2] {
            Token::Participant(name) => name.clone(),
            _ => {
                return Err(ParseError {
                    message: "Expected participant".to_string(),
                });
            }
        };

        let (from_node, to_node) = match &tokens[i + 1] {
            Token::RightArrow => (first_participant, second_participant),
            Token::LeftArrow => (second_participant, first_participant),
            _ => {
                return Err(ParseError {
                    message: "Expected arrow".to_string(),
                });
            }
        };

        let message = if i + 3 < tokens.len() {
            match &tokens[i + 3] {
                Token::ArrowMessage(msg) => {
                    i += 1;
                    Some(msg.clone())
                }
                _ => None,
            }
        } else {
            None
        };

        if !nodes.contains(&from_node) {
            nodes.push(from_node.clone());
        }
        if !nodes.contains(&to_node) {
            nodes.push(to_node.clone());
        }

        adjacency
            .entry(from_node)
            .or_insert_with(Vec::new)
            .push(Edge {
                to: to_node,
                message,
            });

        i += 3;
    }

    Ok(Graph { nodes, adjacency })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_tokens() {
        let tokens = vec![
            Token::Participant("Client".to_string()),
            Token::RightArrow,
            Token::Participant("Server".to_string()),
            Token::ArrowMessage("GET /api/data".to_string()),
            Token::Participant("Server".to_string()),
            Token::RightArrow,
            Token::Participant("Database".to_string()),
            Token::ArrowMessage("SELECT query".to_string()),
            Token::Participant("Server".to_string()),
            Token::LeftArrow,
            Token::Participant("Database".to_string()),
            Token::ArrowMessage("Result set".to_string()),
            Token::Participant("Client".to_string()),
            Token::LeftArrow,
            Token::Participant("Server".to_string()),
            Token::ArrowMessage("JSON response".to_string()),
        ];

        let graph = parse(tokens).unwrap();

        assert_eq!(graph.nodes.len(), 3);
        assert!(graph.nodes.contains(&"Client".to_string()));
        assert!(graph.nodes.contains(&"Server".to_string()));
        assert!(graph.nodes.contains(&"Database".to_string()));

        let node1_edges = graph.adjacency.get("Client").unwrap();
        assert_eq!(node1_edges.len(), 1);
        assert_eq!(node1_edges[0].to, "Server");
        assert_eq!(node1_edges[0].message, Some("GET /api/data".to_string()));

        let node2_edges = graph.adjacency.get("Server").unwrap();
        assert_eq!(node2_edges.len(), 2);
        assert_eq!(node2_edges[0].to, "Database");
        assert_eq!(node2_edges[0].message, Some("SELECT query".to_string()));
        assert_eq!(node2_edges[1].to, "Client");
        assert_eq!(node2_edges[1].message, Some("JSON response".to_string()));

        let node3_edges = graph.adjacency.get("Database").unwrap();
        assert_eq!(node3_edges.len(), 1);
        assert_eq!(node3_edges[0].to, "Server");
        assert_eq!(node3_edges[0].message, Some("Result set".to_string()));
    }

    #[test]
    fn test_with_optional_message() {
        let tokens = vec![
            Token::Participant("Client".to_string()),
            Token::RightArrow,
            Token::Participant("Server".to_string()),
            Token::ArrowMessage("GET /api/data".to_string()),
            Token::Participant("Server".to_string()),
            Token::RightArrow,
            Token::Participant("Database".to_string()),
            Token::ArrowMessage("SELECT query".to_string()),
            Token::Participant("Server".to_string()),
            Token::LeftArrow,
            Token::Participant("Database".to_string()),
            Token::Participant("Client".to_string()),
            Token::LeftArrow,
            Token::Participant("Server".to_string()),
        ];

        let graph = parse(tokens).unwrap();

        assert_eq!(graph.nodes.len(), 3);
        assert!(graph.nodes.contains(&"Client".to_string()));
        assert!(graph.nodes.contains(&"Server".to_string()));
        assert!(graph.nodes.contains(&"Database".to_string()));

        let node1_edges = graph.adjacency.get("Client").unwrap();
        assert_eq!(node1_edges.len(), 1);
        assert_eq!(node1_edges[0].to, "Server");
        assert_eq!(node1_edges[0].message, Some("GET /api/data".to_string()));

        let node2_edges = graph.adjacency.get("Server").unwrap();
        assert_eq!(node2_edges.len(), 2);
        assert_eq!(node2_edges[0].to, "Database");
        assert_eq!(node2_edges[0].message, Some("SELECT query".to_string()));
        assert_eq!(node2_edges[1].to, "Client");
        assert_eq!(node2_edges[1].message, None);

        let node3_edges = graph.adjacency.get("Database").unwrap();
        assert_eq!(node3_edges.len(), 1);
        assert_eq!(node3_edges[0].to, "Server");
        assert_eq!(node3_edges[0].message, None);
    }

    #[test]
    fn test_incomplete_edge() {
        let tokens = vec![
            Token::Participant("Client".to_string()),
            Token::RightArrow,
            Token::Participant("Server".to_string()),
            Token::ArrowMessage("GET /api/data".to_string()),
            Token::Participant("Server".to_string()),
            Token::RightArrow,
            Token::Participant("Database".to_string()),
            Token::ArrowMessage("SELECT query".to_string()),
            Token::Participant("Server".to_string()),
            Token::LeftArrow,
            Token::Participant("Database".to_string()),
            Token::ArrowMessage("Result set".to_string()),
            Token::Participant("Client".to_string()),
            // Missing arrow
            Token::Participant("Server".to_string()),
        ];

        let result = parse(tokens);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .message
                .contains("Incomplete edge. Expected at least 3 tokens")
        );
    }

    #[test]
    fn test_invalid_token_order() {
        let tokens = vec![
            Token::Participant("Client".to_string()),
            Token::RightArrow,
            Token::Participant("Server".to_string()),
            Token::ArrowMessage("GET /api/data".to_string()),
            Token::Participant("Server".to_string()),
            Token::RightArrow,
            Token::Participant("Database".to_string()),
            Token::ArrowMessage("SELECT query".to_string()),
            Token::Participant("Server".to_string()),
            Token::LeftArrow,
            Token::Participant("Database".to_string()),
            Token::ArrowMessage("Result set".to_string()),
            Token::Participant("Client".to_string()),
            Token::Participant("Server".to_string()), // Wrong order
            Token::LeftArrow,
            Token::ArrowMessage("JSON response".to_string()),
        ];

        let result = parse(tokens);
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("Expected participant"));
    }
}
