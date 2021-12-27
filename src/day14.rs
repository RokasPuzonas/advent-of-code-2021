use std::collections::HashMap;

pub struct InsertionRule([char; 2], char);

pub struct InputData {
    polymer_template: String,
    rules: Vec<InsertionRule>
}

fn parse_rule(line: &str) -> InsertionRule {
    let (pattern, expansion) = line.split_once(" -> ").unwrap();
    return InsertionRule(
        [pattern.chars().nth(0).unwrap(), pattern.chars().nth(1).unwrap()],
        expansion.chars().nth(0).unwrap()
    );
}

pub fn parse_input(input: &str) -> InputData {
    let (polymer_template, rules_section) = input.split_once("\n\n").unwrap();
    let rules = rules_section.lines()
        .map(parse_rule)
        .collect();
    return InputData {
        polymer_template: polymer_template.into(),
        rules
    };
}

fn expand_polymer(polymer: Vec<char>, rules: &Vec<InsertionRule>) -> Vec<char> {
    let mut new_polymer = Vec::new();
    for i in 0..polymer.len()-1 {
        new_polymer.push(polymer[i]);
        for rule in rules {
            if rule.0[0] == polymer[i] && rule.0[1] == polymer[i+1] {
                new_polymer.push(rule.1);
            }
        }
    }
    new_polymer.push(*polymer.last().unwrap());
    return new_polymer;
}

pub fn part1(input: &InputData) -> u32 {
    let mut polymer = input.polymer_template.chars().collect();
    for _ in 0..10 {
        polymer = expand_polymer(polymer, &input.rules);
    }

    let mut element_amounts = HashMap::new();
    for c in polymer {
        let amount = element_amounts.entry(c).or_insert(0);
        *amount += 1;
    }

    let least_common_element = element_amounts.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().1;
    let most_common_element = element_amounts.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1;
    return most_common_element - least_common_element;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let polymer_template = String::from("NNCB");
        let rules = vec![
            InsertionRule(['C', 'H'], 'B'),
            InsertionRule(['H', 'H'], 'N'),
            InsertionRule(['C', 'B'], 'H'),
            InsertionRule(['N', 'H'], 'C'),
            InsertionRule(['H', 'B'], 'C'),
            InsertionRule(['H', 'C'], 'B'),
            InsertionRule(['H', 'N'], 'C'),
            InsertionRule(['N', 'N'], 'C'),
            InsertionRule(['B', 'H'], 'H'),
            InsertionRule(['N', 'C'], 'B'),
            InsertionRule(['N', 'B'], 'B'),
            InsertionRule(['B', 'N'], 'B'),
            InsertionRule(['B', 'B'], 'N'),
            InsertionRule(['B', 'C'], 'B'),
            InsertionRule(['C', 'C'], 'N'),
            InsertionRule(['C', 'N'], 'C')
        ];
        let result = part1(&InputData { polymer_template, rules });
        assert_eq!(result, 1588);
    }
}
