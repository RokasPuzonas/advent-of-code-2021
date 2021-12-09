use std::convert::TryInto;


pub struct Entry([String; 10], [String; 4]);

fn parse_line(line: &str) -> Entry {
    let parts: Vec<&str> = line.split(" | ").collect();
    let unique_patterns = parts[0]
        .split_whitespace()
        .map(|s| String::from(s))
        .collect::<Vec<String>>()
        .try_into()
        .unwrap();
    let output_digits = parts[1]
        .split_whitespace()
        .map(|s| String::from(s))
        .collect::<Vec<String>>()
        .try_into()
        .unwrap();
    return Entry(unique_patterns, output_digits);
}

pub fn parse_input(input: &str) -> Vec<Entry> {
    input.lines()
        .map(parse_line)
        .collect()
}

pub fn part1(entries: &[Entry]) -> u32 {
    let mut count = 0;
    for entry in entries {
        for digit in entry.1.iter() {
            let len = digit.len();
            if len == 2 || len == 3 || len == 4 || len == 7 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    // I know it's ugly
    #[test]
    fn part1_example() {
        let input = vec![
            Entry(["be"      .into(), "cfbegad" .into(), "cbdgef"  .into(), "fgaecd"  .into(), "cgeb"    .into(), "fdcge"   .into(), "agebfd" .into(), "fecdb"  .into(), "fabcd"   .into(), "edb".into()], ["fdgacbe"    .into(), "cefdb"   .into(), "cefbgd"  .into(), "gcbe".into()]),
            Entry(["edbfga"  .into(), "begcd"   .into(), "cbg"     .into(), "gc"      .into(), "gcadebf" .into(), "fbgde"   .into(), "acbgfd" .into(), "abcde"  .into(), "gfcbed"  .into(), "gfec".into()], ["fcgedb"    .into(), "cgb"     .into(), "dgebacf" .into(), "gc".into()]),
            Entry(["fgaebd"  .into(), "cg"      .into(), "bdaec"   .into(), "gdafb"   .into(), "agbcfd"  .into(), "gdcbef"  .into(), "bgcad"  .into(), "gfac"   .into(), "gcb"     .into(), "cdgabef".into()], ["cg"     .into(), "cg"      .into(), "fdcagb"  .into(), "cbg".into()]),
            Entry(["fbegcd"  .into(), "cbd"     .into(), "adcefb"  .into(), "dageb"   .into(), "afcb"    .into(), "bc"      .into(), "aefdc"  .into(), "ecdab"  .into(), "fgdeca"  .into(), "fcdbega".into()], ["efabcd" .into(), "cedba"   .into(), "gadfec"  .into(), "cb".into()]),
            Entry(["aecbfdg" .into(), "fbg"     .into(), "gf"      .into(), "bafeg"   .into(), "dbefa"   .into(), "fcge"    .into(), "gcbea"  .into(), "fcaegb" .into(), "dgceab"  .into(), "fcbdga".into()], ["gecf"    .into(), "egdcabf" .into(), "bgf"     .into(), "bfgea".into()]),
            Entry(["fgeab"   .into(), "ca"      .into(), "afcebg"  .into(), "bdacfeg" .into(), "cfaedg"  .into(), "gcfdb"   .into(), "baec"   .into(), "bfadeg" .into(), "bafgc"   .into(), "acf".into()], ["gebdcfa"    .into(), "ecba"    .into(), "ca"      .into(), "fadegcb".into()]),
            Entry(["dbcfg"   .into(), "fgd"     .into(), "bdegcaf" .into(), "fgec"    .into(), "aegbdf"  .into(), "ecdfab"  .into(), "fbedc"  .into(), "dacgb"  .into(), "gdcebf"  .into(), "gf".into()], ["cefg"        .into(), "dcbef"   .into(), "fcge"    .into(), "gbcadfe".into()]),
            Entry(["bdfegc"  .into(), "cbegaf"  .into(), "gecbf"   .into(), "dfcage"  .into(), "bdacg"   .into(), "ed"      .into(), "bedf"   .into(), "ced"    .into(), "adcbefg" .into(), "gebcd".into()], ["ed"       .into(), "bcgafe"  .into(), "cdgba"   .into(), "cbgef".into()]),
            Entry(["egadfb"  .into(), "cdbfeg"  .into(), "cegd"    .into(), "fecab"   .into(), "cgb"     .into(), "gbdefca" .into(), "cg"     .into(), "fgcdab" .into(), "egfdb"   .into(), "bfceg".into()], ["gbdfcae"  .into(), "bgc"     .into(), "cg"      .into(), "cgb".into()]),
            Entry(["gcafb"   .into(), "gcf"     .into(), "dcaebfg" .into(), "ecagb"   .into(), "gf"      .into(), "abcdeg"  .into(), "gaef"   .into(), "cafbge" .into(), "fdbac"   .into(), "fegbdc".into()], ["fgae"    .into(), "cfgab"   .into(), "fg"      .into(), "bagce".into()]),
        ];
        let result = part1(&input);
        assert_eq!(result, 26);
    }
}
