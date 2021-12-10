
#[derive(PartialEq)]
enum ChunkVariant {
    Parenthesis,
    Bracket,
    Curly,
    Pointy
}

pub fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.into()).collect()
}

fn is_opening(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}

fn find_corrupted_chunk_symbol(line: &str) -> Option<ChunkVariant> {
    let mut active_chunks: Vec<ChunkVariant> = Vec::new();
    for c in line.chars() {
        let variant = match c {
            '('|')' => ChunkVariant::Parenthesis,
            '['|']' => ChunkVariant::Bracket,
            '{'|'}' => ChunkVariant::Curly,
            '<'|'>' => ChunkVariant::Pointy,
            _ => panic!("Invalid character found while finding corrupted chunks")
        };

        if is_opening(c) {
            active_chunks.push(variant);
        } else {
            if *active_chunks.last().unwrap() != variant {
                return Some(variant);
            }
            active_chunks.pop();
        }
    }
    None
}

pub fn part1(lines: &Vec<String>) -> u32 {
    let mut score = 0;
    for line in lines {
        let result = find_corrupted_chunk_symbol(line);
        score += match result {
            Some(ChunkVariant::Parenthesis) => 3,
            Some(ChunkVariant::Bracket) => 57,
            Some(ChunkVariant::Curly) => 1197,
            Some(ChunkVariant::Pointy) => 25137,
            None => 0
        }
    }
    return score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = vec![
            "[({(<(())[]>[[{[]{<()<>>".into(),
            "[(()[<>])]({[<{<<[]>>(".into(),
            "{([(<{}[<>[]}>{[]{[(<()>".into(),
            "(((({<>}<{<{<>}{[]{[]{}".into(),
            "[[<[([]))<([[{}[[()]]]".into(),
            "[{[{({}]{}}([{[{{{}}([]".into(),
            "{<[[]]>}<{[{[{[]{()[[[]".into(),
            "[<(<(<(<{}))><([]([]()".into(),
            "<{([([[(<>()){}]>(<<{{".into(),
            "<{([{{}}[<[[[<>{}]]]>[]]".into(),
        ];
        let result = part1(&input);
        assert_eq!(result, 26397);
    }
}

