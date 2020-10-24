use super::*;

#[test]
fn ignores_whitespace() {
    let scanner = Scanner::new("    \r\n.");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![Token::Dot]);
}

#[test]
fn scans_empty_string() {
    let scanner = Scanner::new("");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![]);
}

#[test]
fn scans_keyword() {
    let scanner = Scanner::new("while");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![Token::While]);
}

#[test]
fn stops_scanning() {
    let scanner = Scanner::new(".");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![Token::Dot]);
}

#[test]
#[should_panic]
fn panics_unexpected_symbol() {
    let mut scanner = Scanner::new("§");
    scanner.next();
}

#[test]
fn scans_identifier() {
    let scanner = Scanner::new("columbia");
    assert_eq!(scanner.collect::<Vec<_>>(), vec![Token::Identifier]);
}

