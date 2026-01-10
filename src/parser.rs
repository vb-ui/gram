use crate::tokenizer::Token;

pub type Participant = String;

#[derive(Debug)]
pub struct Edge {
    pub from: Participant,
    pub to: Participant,
    pub message: Option<String>,
}

#[derive(Debug)]
pub struct SequenceDiagram {
    pub participants: Vec<Participant>,
    pub edges: Vec<Edge>,
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

pub fn parse(tokens: Vec<Token>) -> Result<SequenceDiagram, ParseError> {
    let mut participants = Vec::new();
    let mut edges = Vec::new();
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

        let (from_participant, to_participant) = match &tokens[i + 1] {
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

        if !participants.contains(&from_participant) {
            participants.push(from_participant.clone());
        }
        if !participants.contains(&to_participant) {
            participants.push(to_participant.clone());
        }

        edges.push(Edge {
            from: from_participant,
            to: to_participant,
            message,
        });

        i += 3;
    }

    Ok(SequenceDiagram {
        participants,
        edges,
    })
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

        let diagram = parse(tokens).unwrap();

        assert_eq!(diagram.participants.len(), 3);
        assert!(diagram.participants.contains(&"Client".to_string()));
        assert!(diagram.participants.contains(&"Server".to_string()));
        assert!(diagram.participants.contains(&"Database".to_string()));

        let edge1 = &diagram.edges[0];
        assert_eq!(edge1.from, "Client");
        assert_eq!(edge1.to, "Server");
        assert_eq!(edge1.message, Some("GET /api/data".to_string()));

        let edge2 = &diagram.edges[1];
        assert_eq!(edge2.from, "Server");
        assert_eq!(edge2.to, "Database");
        assert_eq!(edge2.message, Some("SELECT query".to_string()));

        let edge3 = &diagram.edges[2];
        assert_eq!(edge3.from, "Database");
        assert_eq!(edge3.to, "Server");
        assert_eq!(edge3.message, Some("Result set".to_string()));

        let edge4 = &diagram.edges[3];
        assert_eq!(edge4.from, "Server");
        assert_eq!(edge4.to, "Client");
        assert_eq!(edge4.message, Some("JSON response".to_string()));
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
            // Token::ArrowMessage("Result set".to_string()),
            Token::Participant("Client".to_string()),
            Token::LeftArrow,
            Token::Participant("Server".to_string()),
            // Token::ArrowMessage("JSON response".to_string()),
        ];

        let diagram = parse(tokens).unwrap();

        assert_eq!(diagram.participants.len(), 3);
        assert!(diagram.participants.contains(&"Client".to_string()));
        assert!(diagram.participants.contains(&"Server".to_string()));
        assert!(diagram.participants.contains(&"Database".to_string()));

        let edge1 = &diagram.edges[0];
        assert_eq!(edge1.from, "Client");
        assert_eq!(edge1.to, "Server");
        assert_eq!(edge1.message, Some("GET /api/data".to_string()));

        let edge2 = &diagram.edges[1];
        assert_eq!(edge2.from, "Server");
        assert_eq!(edge2.to, "Database");
        assert_eq!(edge2.message, Some("SELECT query".to_string()));

        let edge3 = &diagram.edges[2];
        assert_eq!(edge3.from, "Database");
        assert_eq!(edge3.to, "Server");
        assert_eq!(edge3.message, None);

        let edge4 = &diagram.edges[3];
        assert_eq!(edge4.from, "Server");
        assert_eq!(edge4.to, "Client");
        assert_eq!(edge4.message, None);
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
            // Token::LeftArrow,
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
