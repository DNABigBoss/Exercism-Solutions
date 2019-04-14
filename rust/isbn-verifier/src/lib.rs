/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.is_empty() || isbn.contains(|c: char| c != '-' && !c.is_numeric() && c != 'X') {
        return false
    }

    let new = isbn.replace("-","");

    if new.len() != 10 || new[0..8].contains('X') {
        return false
    }

    new.chars()
    .map(|c| if c == 'X' {10} else {c.to_digit(10).expect(&format!{"Invalid Char = {}", c})})
    .enumerate()
    .map(|(i,n)| (10-i as u32, n)).map(|(i,n)| i*n).sum::<u32>() % 11 == 0
}
