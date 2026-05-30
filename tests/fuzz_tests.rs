use minicompiler::lexer::Scanner;
use minicompiler::parser::Parser;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[test]
fn test_lexer_fuzzing() {
    let mut rng = StdRng::seed_from_u64(42);
    
    // Generate 10,000 random garbage strings and ensure lexer doesn't panic
    for _ in 0..10_000 {
        let len = rng.gen_range(1..100);
        let random_string: String = (0..len)
            .map(|_| rng.gen::<u8>() as char)
            .collect();
            
        let mut scanner = Scanner::new(&random_string);
        // Just consume all tokens, ensuring no panics
        loop {
            let token = scanner.next_token();
            if token.token_type == minicompiler::lexer::TokenType::EndOfFile {
                break;
            }
        }
    }
}

#[test]
fn test_parser_fuzzing() {
    let mut rng = StdRng::seed_from_u64(1337);
    
    // Generate 1,000 semi-random token streams or garbage strings to ensure parser doesn't panic
    for _ in 0..1000 {
        let len = rng.gen_range(1..50);
        let random_string: String = (0..len)
            .map(|_| {
                let charset = b"abcdefghijklmnopqrstuvwxyz0123456789 ()[]{};+-*/=<>!";
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .collect();
            
        let mut scanner = Scanner::new(&random_string);
        let mut tokens = Vec::new();
        loop {
            let token = scanner.next_token();
            let eof = token.token_type == minicompiler::lexer::TokenType::EndOfFile;
            tokens.push(token);
            if eof { break; }
        }
        
        let mut parser = Parser::new(tokens);
        // We don't care if it fails to parse (it almost certainly will),
        // we only care that it returns a clean Error and does NOT panic.
        let _ = parser.parse();
    }
}
