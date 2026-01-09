use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftArrow,
    RightArrow,
    ArrowMessage(String),
    Participant(String),
}

#[derive(Debug)]
pub struct TokenizeError {
    pub line: usize,
    pub message: String,
}

impl std::fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tokenize error at line {}: {}", self.line, self.message)
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenizeError> {
    let mut tokens: Vec<Token> = Vec::new();

    for (line_number, line) in input.trim().lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        tokenize_line(line, line_number + 1, &mut tokens)?;
    }

    Ok(tokens)
}

fn tokenize_line(
    line: &str,
    line_number: usize,
    tokens: &mut Vec<Token>,
) -> Result<(), TokenizeError> {
    let arrow_regex = Regex::new(r"->|<-").unwrap();
    let arrow_match = arrow_regex.find(line).ok_or_else(|| TokenizeError {
        line: line_number,
        message: "Missing arrow ('->' or '<-')".to_string(),
    })?;

    if arrow_regex.is_match(&line[arrow_match.end()..]) {
        return Err(TokenizeError {
            line: line_number,
            message: "Multiple arrows found. Expected exactly one arrow per line".to_string(),
        });
    }

    let first_participant = line[..arrow_match.start()].trim();
    validate_participant(first_participant, line_number, "First")?;
    tokens.push(Token::Participant(first_participant.to_string()));

    let arrow_str = arrow_match.as_str();
    match arrow_str {
        "->" => tokens.push(Token::RightArrow),
        "<-" => tokens.push(Token::LeftArrow),
        _ => unreachable!(),
    }

    let rest = line[arrow_match.end()..].trim();
    if let Some(colon_pos) = rest.find(':') {
        let second_participant = rest[..colon_pos].trim();
        validate_participant(second_participant, line_number, "Second")?;
        tokens.push(Token::Participant(second_participant.to_string()));

        let message = rest[colon_pos + 1..].trim();
        if !message.is_empty() {
            tokens.push(Token::ArrowMessage(message.to_string()));
        }
    } else {
        validate_participant(rest, line_number, "Second")?;
        tokens.push(Token::Participant(rest.to_string()));
    }

    Ok(())
}

fn validate_participant(
    name: &str,
    line_number: usize,
    position: &str,
) -> Result<(), TokenizeError> {
    if name.is_empty() {
        return Err(TokenizeError {
            line: line_number,
            message: format!("{} participant is empty.", position),
        });
    }
    if name.len() > 80 {
        return Err(TokenizeError {
            line: line_number,
            message: format!("{} participant is too long (max 80 characters).", position),
        });
    }
    if name.contains('\n') {
        return Err(TokenizeError {
            line: line_number,
            message: format!("{} participant contains new line character.", position),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_input() {
        let input = "\
Client -> Server: GET /api/data
Server -> Database: SELECT query
Server <- Database: Result set
Client <- Server: JSON response";
        let tokens = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
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
            ]
        );
    }

    #[test]
    fn test_whitespaces() {
        let input = "
Client    ->    Server  :      GET /api/data \t
Server    ->    Database:      SELECT query  \t
Server    <-    Database:      Result set    \n
Client    <-    Server  :      JSON response \n";
        let tokens = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
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
            ]
        );
    }

    #[test]
    fn test_optional_arrow_messages() {
        let input = "\
Client -> Server: GET /api/data
Server -> Database: SELECT query
Server <- Database
Client <- Server";
        let tokens = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
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
            ]
        );
    }

    #[test]
    fn test_one_line() {
        let input = "\
Client -> Server: GET /api/data\n Server -> Database: SELECT query\n Server <- Database: Result set\n Client <- Server: JSON response";
        let tokens = tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
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
            ]
        );
    }

    #[test]
    fn test_missing_arrow() {
        let input = "\
Client -> Server: GET /api/data
Server Database: SELECT query
Server <- Database: Result set
Client <- Server: JSON response";
        let result = tokenize(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.line, 2);
        assert!(err.message.contains("Missing arrow ('->' or '<-')"));
    }

    #[test]
    fn test_multiple_arrows() {
        let input = "\
Client -> Server: GET /api/data
Server -> Cache -> Database: SELECT query
Server <- Database: Result set
Client <- Server: JSON response";
        let result = tokenize(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.line, 2);
        assert!(
            err.message
                .contains("Multiple arrows found. Expected exactly one arrow per line")
        );
    }

    #[test]
    fn test_empty_first_participant() {
        let input = "\
Client -> Server: GET /api/data
-> Database: SELECT query
Server <- Database: Result set
Client <- Server: JSON response";
        let result = tokenize(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.line, 2);
        assert!(err.message.contains("First participant is empty."));
    }

    #[test]
    fn test_empty_second_participant() {
        let input = "\
Client -> Server: GET /api/data
Server -> : SELECT query
Server <- Database: Result set
Client <- Server: JSON response";
        let result = tokenize(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.line, 2);
        assert!(err.message.contains("Second participant is empty."));
    }

    #[test]
    fn test_participant_too_long() {
        let long_name = "A".repeat(82);
        let input = format!(
            "\
Client -> Server: GET /api/data
Server -> {}: SELECT query
Server <- Database: Result set
Client <- Server: JSON response",
            long_name
        );
        let result = tokenize(&input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.line, 2);
        assert!(
            err.message
                .contains("Second participant is too long (max 80 characters).")
        );
    }
}
