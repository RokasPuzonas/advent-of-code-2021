
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

// I know that it is not ideal to throw a panic here. But I'm too lazy to do
// error handling. And the input is guaranteed to be correct to this is fine.
fn get_chunk_variant(c: char) -> ChunkVariant {
    match c {
        '('|')' => ChunkVariant::Parenthesis,
        '['|']' => ChunkVariant::Bracket,
        '{'|'}' => ChunkVariant::Curly,
        '<'|'>' => ChunkVariant::Pointy,
        _ => panic!("Invalid chunk character")
    }
}

fn find_corrupted_chunk_symbol(line: &str) -> Option<ChunkVariant> {
    let mut active_chunks: Vec<ChunkVariant> = Vec::new();
    for c in line.chars() {
        let variant = get_chunk_variant(c);
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

fn find_unclosed_chunks(line: &str) -> Vec<ChunkVariant> {
    let mut active_chunks: Vec<ChunkVariant> = Vec::new();
    for c in line.chars() {
        if is_opening(c) {
            active_chunks.push(get_chunk_variant(c));
        } else {
            active_chunks.pop();
        }
    }
    active_chunks
}

pub fn part2(lines: &Vec<String>) -> u64 {
    let mut scores = Vec::new();
    for line in lines {
        let corrupted = find_corrupted_chunk_symbol(line);
        if corrupted == None {
            let mut score = 0;
            let mut unclosed_chunks = find_unclosed_chunks(line);
            unclosed_chunks.reverse();
            for chunk in unclosed_chunks {
                score *= 5;
                score += match chunk {
                    ChunkVariant::Parenthesis => 1,
                    ChunkVariant::Bracket => 2,
                    ChunkVariant::Curly => 3,
                    ChunkVariant::Pointy => 4
                }
            }
            scores.push(score);
        }
    }
    scores.sort();
    return scores[scores.len()/2];
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

    #[test]
    fn part2_example() {
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
        let result = part2(&input);
        assert_eq!(result, 288957);
    }
}

