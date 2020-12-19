use std::collections::*;
use itertools::Itertools;

static RULES: [&str; 136] = ["66: 69 116 | 9 115","91: 95 9 | 109 69","14: 110 69 | 15 9","4: 119 9 | 61 69","17: 9 23 | 69 93","37: 118 69 | 94 9","68: 9 80 | 69 19","117: 37 9 | 45 69","132: 9 109","74: 9 25 | 69 126","102: 122 9 | 6 69","98: 89 9 | 99 69","113: 83 9 | 7 69","92: 9 16 | 69 50","33: 9 39 | 69 133","134: 95 69 | 7 9","57: 9 7 | 69 110","31: 9 66 | 69 51","47: 9 3 | 69 83","21: 69 123 | 9 129","104: 9 86 | 69 7","40: 69 24 | 9 34","32: 9 44 | 69 88","45: 49 69 | 48 9","2: 69 88 | 9 23","5: 3 9 | 110 69","108: 20 86","76: 97 9 | 113 69","56: 9 121 | 69 58","29: 69 13 | 9 82","123: 86 20","46: 23 69 | 95 9","19: 7 69 | 109 9","12: 93 9 | 95 69","75: 85 9 | 128 69","127: 93 69 | 83 9","61: 9 7 | 69 23","78: 9 132 | 69 107","121: 88 69 | 23 9","60: 33 69 | 76 9","73: 26 69 | 100 9","116: 9 60 | 69 75","93: 69 9","38: 9 106 | 69 110","107: 9 7 | 69 86","82: 5 69 | 64 9","58: 44 69 | 23 9","34: 69 57 | 9 104","124: 69 106 | 9 44","109: 69 9 | 9 9","48: 69 44 | 9 83","28: 9 83 | 69 7","64: 9 93 | 69 7","54: 9 102 | 69 40","94: 83 69 | 7 9","80: 9 86 | 69 3","62: 86 69 | 83 9","42: 125 69 | 70 9","71: 69 112 | 9 114","8: 42","131: 69 44 | 9 7","88: 69 9 | 9 69","87: 15 69 | 7 9","3: 9 9 | 69 69","9: \"b\"","41: 21 69 | 78 9","65: 9 69 | 69 20","128: 134 69 | 101 9","44: 9 69 | 69 69","51: 9 18 | 69 54","55: 69 79 | 9 5","95: 69 69 | 69 9","110: 9 69","22: 92 69 | 10 9","67: 9 44 | 69 23","7: 9 20 | 69 69","90: 69 96 | 9 68","53: 111 9 | 27 69","125: 69 72 | 9 22","11: 42 31","23: 9 69 | 9 9","114: 69 105 | 9 131","81: 53 69 | 29 9","69: \"a\"","1: 86 9 | 95 69","18: 9 73 | 69 90","83: 20 20","130: 89 69 | 47 9","89: 44 9 | 93 69","15: 9 9","0: 8 11","105: 69 110","13: 135 9 | 14 69","6: 69 105 | 9 132","103: 120 69 | 71 9","85: 1 69 | 2 9","96: 89 9 | 12 69","101: 69 95 | 9 23","39: 69 15 | 9 109","133: 88 69 | 65 9","86: 69 20 | 9 9","122: 127 9 | 43 69","20: 69 | 9","52: 69 109 | 9 3","119: 15 69 | 88 9","77: 9 7 | 69 93","50: 87 9 | 61 69","129: 9 65 | 69 110","97: 69 95 | 9 93","111: 59 69 | 35 9","115: 69 84 | 9 41","84: 36 69 | 4 9","72: 117 9 | 74 69","135: 110 69","112: 62 69 | 124 9","43: 69 3 | 9 15","118: 109 9 | 93 69","49: 69 23 | 9 83","26: 69 67","63: 106 9 | 44 69","70: 103 69 | 81 9","25: 17 9 | 77 69","36: 91 9 | 32 69","10: 69 98 | 9 56","30: 9 109 | 69 3","126: 9 28 | 69 63","16: 9 38 | 69 30","99: 106 9 | 109 69","59: 23 9 | 3 69","120: 130 9 | 55 69","27: 69 91 | 9 108","24: 46 69 | 52 9","35: 9 93 | 69 109","100: 9 101 | 69 80","106: 69 69","79: 44 69 | 65 9"];
static INPUT: [&str; 343] = ["bbabbbaaaaaabbabbaabaaabbaababbbabbbabbbababbbbbbbbabbbbbbabaaaa","aabbbabbabaaaababbbaaabb","baababbbaababaaaabbaababbabbbaaabaaaabbbaaaabbaaabbaabba","abbbbaaabbaaaababbbabaaabbbabbbbbabaabab","aaabbbbbabbbbbabbabbaabbaaaaababababbbbaaaaabbbabbbbabbababbaaabaaaababa","bbbabaaaabbaaabbbababababababaaaabaabbbabaaaabbbabbabbabbbaaabbaaabaaaaa","bbaaaabbaabbbaabbabaaaba","abaabbabbaaaaabbbbbabaabbabbbababaaababb","baaaaabbbabbbbbbabababba","baaabbbaabbaabababbbaaab","baaaabaababbbbbababbbbaaabbabbaabbababababababbabbbabaab","baabbaaaaaaaaaababbaaabaaabbabaaabbababaaaabaaaaabbaabaaababaabbbaabbbaaaabbabbbabbbbbbb","ababbbaaabaabaababababbabbbaabbaaaabbbbbababbabaabbbbbaa","abaabbaabbbbbbbaaaabbbbaaaababba","ababaaabbaabbaaabbbaaaba","bababbbbbaabbabbabbabaaaabaababababababababaabab","bbaaaaababbbaababbbbbbaa","abaabababababbbaababaaaa","aabbababaaaababababbbbba","aabaababbbaaababbabbabab","baababababbbabbbbbbbbaba","bbbbaaaabbababaabbbabbabababbbab","baaabbbbbbabbbbbaaaabaaa","ababbbbabbaabbaaaababbaaaababaaabbbbaaabababbabbaababbbb","bbbbaababababbbbbabbaabbbabaaaba","abbbabbababbbaaaababbbbb","abbbbbabbabbabbbbaaabbabbaaaabbbbbaabaaabaabbaba","aabbabbabbbbbbabbbabbaaa","bbbbaaaabaababbbbabbbbab","bbababaaabbaaabbabaaaabb","bbbbbbbabaabababbbbababbbabbbaaaababaaaa","abbbbbaabbabbbaabbbababbababbaabbaaabbab","aaabaabbbbabababaaaabababababaabbbabbabb","abbabbabbbaaaabbabaaaababbbaabbabaaabaaa","babbbabbbbbabaaababaaabb","bbaaaaabaabbaaaabbbabbbb","abaaabbbbaababbaaabaaaab","bababaababbaaabbaaaabaab","bbaabaabbabaaaaabbbbbbbaaabaaabaaaabbbaabbaabbbabbaaaabaaabbabbbbbaaaaaabaabaaaaabbaaaba","abbabababbbbababbbaabaab","ababababaabbaabbbbabbaaa","bbbbbaabbbbabaaabaaaabba","aababaaabababbbabbababababaabbbaabbaaabababbaabbbaabbbaa","aabbaabaabbbbabbaabababb","aabaaaaaabbbbbbbabababbbbabaaaabbbaabbba","bbaaaaabaaabaabbaabaaaab","aaaaaaaaabbaaaaabbbbaaababaaabab","baababbaaaaaababbbbaaabb","abbbbaabaabbaabbbaaabbaaabbbbabbabbbaaaabaaaaaaaaabaabba","baabaabaaabbbabbbbaaaabaabaaaaabaaabaabbbabaabbb","abaaaaabbabbbabbbababbaaaabaaabbaabbababbabbbabababaabaabaabbbaa","aaaaaaabababbbbaaabaaabaababababababbabb","abbbabaabbabaababaaaabaa","abbbbbaabababbbbbabaaaba","aabbaabbbaabaaaabbaaabba","bbabababaaaaabaabbbbaabaaaaababb","aabbaabbbbbbbaabaaabaaba","bbbbbaabbabbbbbbabbbaaab","bbababbaabbbbabbbaabbbab","baaabbbbbababbaaaaabbbab","bbaabbbbbbbbbababaaaabbbbaaaababbbaababa","aabaabababbbbaabaaaaabba","abaabbbbaabbbaaabbbabbba","baabaaaaabbbaabaaabababb","baaabababbbababbaaaaaabb","babaababbaaaababbaabbaaaaaaabababbbbbbaabbaaabbbaaaabbabaaaaaabbaabbaaaaaabbbaaaaaabaaba","aaabaabbaaaababaaaaaaaabaabbbbbb","abbbababababbbbabaaaabab","abaabaabbbabababaaabbaaa","bbbbababaabaaaaaaabbbabbaababaabababaaabbbbabbab","bbbbaaabaabbbabbabbbabbabaababbbbababaaa","bbaabbbbaaabbabbaaababaaaabaaababbaabbba","aabaabbabbabbbaaaaaaabbb","aabbabaababbbabbaabbbbab","baababbbbbbbbbabbabbaaaababaabab","babbaababbaabbaaaaaaaaba","aaabbbbabbaaaabbaaabbaaa","aaaabbbbbaaababaababbbbabababbababaabbab","baaaaabbbaaabaababbaaabbabaabbaabbbbaaaababbaaaa","babbabaabbbbaaaababbbbaabbbbabbb","abbbbabaaabababababaabab","bbaaaaabababaababababbab","ababbaabaabbaaaabaabbaba","bbaabbaaabaababaaaabababbbbbaaba","babbbbaaaabbabbaaabbbbbb","baaabbaabababbaaabababaa","aaaabbbabaabababbabbaababbbbabbaaaabaabbaaaabababbbaabbb","bbbbbbabbaaabbaaabbbbaabbbabaababbbbbaaabbabaaabbabbbbba","baabaabaaabaabbaaabbababbabbabaabbabbaabbaaabaab","babaaaaaabbbbaabbbbbbaba","bbbaababaabbbabbaaababba","bababbbbbaaabbbbabbbabbaababbaababbbaaabbbbaaaba","abbbababbbbbaaabbabbaaab","bababaababaabbbabaaabbbbbaabababbbbaabaa","bbbaabbbaabbbbbaabaaabaaaaabbbbbaababbbbbbababbabbabababaaaababaaaabaabb","baaaaabbabaabaaaabbbbaaaaababababbbbabbb","bbbbaabbbbaaabbbbaabbbaabaaababbbbaababb","abaaaabaaabbababbbaabbba","bbabaabbbbaabbaaababbaabaaababaa","aababbbaabaabbbaaababbaabaaabaaaaabbbbba","babbaabbbbbbaabaaabbabbaaabbabaabbaabbab","aababaabbbbababbaabbbaababbabbabbaaaabaaabbaabaabbbaaabb","abaabbbbabaabbaababaaaba","aabbbaabbaaabababbaaabba","baabaaaaaaaabababaaaabab","bbaaaabbbaaabbbaabababba","bbaaaaabaabaaabbaaaaabbb","baaababaabbaaaaababbbbbbabbbbbaabbbbbbabababbbaa","abbbbabaababaabaaaaabaab","baaababaabbbabbbbaabbaab","abaabbaabababaababbaaaaaababaabb","abaababaaabbabbaabaaabba","bbaababbaabaaabbbababaabbbbabbab","babbaaaaabbabbbaabababbb","aabbaaabbabbbbaaaabbbbba","bababaaababaaababbbabbba","bababbbaabbaabababbbabbbbbababababbbbbaabaaababb","abbabbbababbbbaaaabbaabbaabbaaaaabbabbaa","aaaabababbabbbaaaabaabaaabbabbabbbbbaaaabbaaaababaabbabbbbbbbbbababbabaaaabbbaab","bbbaaaaabbaabbbaaabaabbaabaaababbabbbbbabbbabaab","bbaaaababbbaababbabbbaab","aabaaabaaaabaabbbbabbaaababbababbabaababbbbaabba","aaaabababaababbaaabaaabaababaababbaaabbbbabbbaab","bbaabbaaaaaaabababbabbbabaaaabbb","ababbababbabaababbbabbbb","bbbbbbabbbaabbbbbbbabbbbbbbbbbaaaabbbbabaaabaaba","aabaaababbaabbaababbaabaabbaabbaababbbbb","bbaaaaabbbbaabbabbabbabaabaaaaaababaabab","aababaaabbbbaabbbbababbabbabaaaabaaaababbaaaababbabbbaabbaaabaab","aabbaaaaabbbaaaaabbaaaaabbbabaab","aababbbabbbaababaabababaababbbaaabaaabab","bbbaaaababaaabaaaaaabbabaababbbaabababaaaaaabaabaaababab","abaaaabbabababbaababbbab","bbabaabbaababababaabbbbb","babbabaabbabbbbbaaabaabbbabbaaaabaaabbba","aaaaabaababbabaaabbabbbb","bbaaaabbaabaaabbbaabbaba","abaaababaaabbbbbbbabbababbaaabaa","bbbbaababbbabaaababbabab","ababababaababaaabababbbbaaabbaababbbbabababaabba","babbaabaabaabbbbbaabaaaaaaabaabbbaabbabb","aaabaaaaababbaabaabbabaaabaabbabbbbbbaaabaaabaaa","baaabbaabaabaabbbaabbbab","aaaabbbaabaabbbbabbbbbaabbbbbbbbababbbaa","bbabbbaabababaababbaabaa","bbabaabaabbababaabaaaaabaaaaaaaabbbaabba","bbaababbbaabbaaabaaaaaba","baabbabbabbababbbbbbbbaabbbaaaaabaaaabbb","babaaabaaababbabbabaaaaaaaabbaaaaabbabbabaaabbbabbaabbab","babbabaaabaaaabaabbbababbaaaabba","abbababaaabaabbaabbbaaaabbaabaababbbbbabaabbbaaaaaababbaabbbbbbb","abbabbbaabaabaaaaababaabaaaabbbbabbaaaaababababbbbabbaabaabbababaaaabaabaaabababbbabbaab","bbaababbaaaaabaaaabbbaababbbbbab","babbbabbaaabaaababaabbbabbbaaaba","aaabaaabbaababbaabbbbbaaaabbbaba","bbaaabababaaaabaababaabaaabbbabbbbbbaabbbbaabbaababbbbabbbabbabbbababbab","abbbabbabbbbaaababbbbaabbaaaaaab","babbbabbaabbaabaaabbaaabbbbbaaabbaababbbaaaaaabb","aabbaabaabaabbabbbbbbbbabbbaabbb","abaaaaabbbbbaababaaaabab","abbbbbbaaabaaaaabbabaaaa","bbbbbbbaaabababaaaabaaba","bababbaababaaaaababaaaba","abbbaababaababbbbbbbabbabbbaabaa","abbbbbaabbbababababbbbbbbaaaabaabbbaaaba","abaaaabaabaaabbbbaababaa","bbaabaaaaaaaababaabbaabbabbbbbbabaabbbbabbababba","baabbaabaaaaabaabaabaabbbbabbbab","bbaaabbaaabbabbaaaaaaabbbbbaabbabbaaaaaabaabbbaa","aaaaaaabbbbbbbabbbababaabbababbb","abbbabbbbabbbaaaabababbb","aaabaaabaaaabbabaaaabbbb","babbaaaabbbbbbabbabbabba","ababababaabbabaabbabbbbbbaabbbaa","aabbbaabaaaaaaabaaababbb","aaaaabaaaaabbbbaaaaabbababaabbababaabababaaaabaa","abbbbaabbbabaabbababaaabaaabaaabbaaaabbabbaaabba","aaabbbbabbbbaabaababababbababbbabaabaabbbbbaabbb","abbbababbaababbababaabaa","ababaaababbbbaabbbabbabb","babbbbbbbaababbbbbbabaaaabababbbbaabbabb","aabaaabbaabbaaaabaaaabab","abbbabbbbababbbbbbabababababaabbaabbbbba","bbbabaaaaababaabaaaabbaa","bbbbaaabbabbaababbbaabba","ababababbaaababababaaaaaaaabababbaaabbbbbbaaabaaaaababbabaaaabba","bababbbbbbaaababaababaabaabbababbbbbbbba","baababbaaabaabababbaabba","bbbaababbbbbbbbabbaabbab","babbbbaababbbbbbbaabbbba","aaabaabbabbbbaabbbaababa","abbbaaaaaaaaabaabbabbaab","aabababbababaabaabbbbbbbaabbaababbaaaaaaaaabbbbaaaaabbaa","bababbbbaaababababbabababbbbbbbaababaaaaaaabbaaa","bbbbaaababbabbababababaa","bbbbbaabaaaababbaabbbbba","baabbaaabbbbabbbbbabaabaaabaabbbaabbbbbb","bababbaababbbbbbbaaaaabbbbaaabab","aabbaaabbbbbbbabaababbabbbaaabaa","bbbbaaabbaababbaaababbab","ababbaabaabbababbaaaabaa","baabaabbabbbabbbbaaaabbb","abbabbbabbbaababbbbaabba","abaaabbaaaabbabbababaabbbaaaababbaaabbbbaaabaabb","baabaabbbaaaaabbbabbaaaaaabbabbbbbbaabbb","abbaabababaabababbaababbabbaabaa","bbaaaaaabbaaaabaaabbababababbbab","aabbaaabbababbaabbbbabbbbaababababbaabba","bbbbaabbaababaababbbbaabbbabbbbaababbbaaabbbbbbbbbababbb","aaaaababaaaababaabbbababaabababaabbaaabbbabbbbab","aaabbbbabbabaabbbbaaabaa","bbabababbabaaabbbbbaaaba","aabbabbabbbabababaaabaab","bbbbaabbabbbabbbaabbbbbabaabababaaabbbab","baaabbbbaabbbabbbbaaaaaaabbbaabbaababbbb","bbaaabababbbaabaabbbaababababbabbbbabbab","bbbaaaababbaaababbbabbabbbbbbaaaababaabb","abaababbbabbbbabaababbabbaaaaabbbaababaaabbaabaabbababaaabaababb","ababbaabaabaabbaabbabaaa","bababbaababbaababaabbaabaabaaaaabaabaabaaababbabbbabaaaa","babaaaaaaabbbabbaaabbbaa","abbbbaabbbbbaababbbaababaaaaaaababababbb","bbabababbababbbbbbbbbaaa","aabbbaabbababbaaaabaaababbabbabb","baababbbabbbbbbabbbbbbbaaaabababbaaabbab","bababbbbabaabbaaabbbbaaabaaaabbb","abbaabaaabababbbaaaaaaaaaabbaabbbaabaabb","bbabbbbbbbabbbbabbbbaabbabaaaabbbaabbaba","bbaabbaabbababaabbaaababbabaaabb","bbaaaabbbaabaababbbbabbaababaabb","aaaaaaababaabbbabbbbabaa","bbbbaabaabababbabbaabaaabbababbaababbbbbaaaabbaabbbabbbabbaababb","abaaabbbbbbbababbbaababa","abaaaababbbbabbaabababba","aababbbabbbbaabbbababbbbabaaaaababbbaabbbababbab","bbbbaaaaabaabaabbbbbababbaaaaaba","babbaaaabaabbaabaaaaabba","bbbbaaaabaababbabbbaabba","ababababbbaaaaaaabbabaab","bababbbabbbaababaabbabaaabbabbbabbabbbab","baabaaabbabbbbbbbbbbbaabaabababa","ababbbbaabaaaabababbbbba","aaaaababbbabbbaababaaaba","baabababbaaaaabbbbababbb","bbbbabbabaabaaaababbbbba","aababaaabbabbbabbbbbbabbbbbaabbbbaabbabb","bbabaabaabbabababbbabaaaabaabaaababaaaba","aabbbbaaabbbbabbbabbabab","abaababaababbbbbbababbab","bbbabaaaabbbbabaaaabbabb","aabbaabbabbabbabbabbabba","bbababababaabbbbbabaaaab","bbbbaaaaaabaaabbbaabbbbb","aaaababbbaababababbabbaa","abaabbaaaabaaabababbbbba","aababbaabbaaababbbbbbbbababbaabb","babbbababbbbbaabbaabbbabbaaaababbbaabbabbbbaaabbaabbbabbbaababaa","bbaaaaabbaabaabaabbaaabbababaababaabaabbabbabbaabbaaabbababbbbab","abbabbbabaabaabbabbbaaab","babbababbaabbabaabaaabbabbababbbaaababbb","baababbbbbababaaababaaaabbabbaaa","bbbabaaaaaabbbaaaaaabbabbabababbabbbbaaaaaababab","ababaaabbaaabbbbababaaabbbbbaaaababbaabaabbbbabbbbabbaaa","abaaaaababbaaaaabbabbbaabaaaabaababaaaba","abaabababaabbaabbbbaaaab","babbbbbbbaaabbaaabbbbababbaaabbbabababaabbbaaaba","baabaaabaaaaaaabbbabaabbbbabbaba","aababbaaabbbaaaaabababba","abbbbabaabaabbaaaaaaaaababbbaaaabbabbaaababaaaab","bbaaaabbaababaabaabaaabaabbabbbb","bababaabbabbbaaabbaabaaa","bbbaabababbbbaabbabbabaabbaabaaa","babaaaaaabaaabbbbbababbb","abbbbbaabaabaabababbabba","baababbaaabbababababbbaa","aaabbbbaaaabababbabaabba","abaabbbabbbaabababaaabab","babbbaaaaabaaabababbbaba","abaabaabbabbaabaabaababb","aaaaaaabbabbaabaaaabbbab","bbabbbaabaabaababbaaaaab","bbaaaabbbbbbbbabababaabb","bbaababbbaaabbbabbaabbab","aaababababbabbabbabbabab","aabbaaaababaaaaaabbbabbaabaabbbaababaaaa","aabaaaaaabbaabababaaabab","baabaaabbbbababaababbbbb","ababababbbbbaababbaabaaa","bbbababaaababbbaabaaabab","bbbababbbbababababababbb","baababbbbbbbbbabaabbbbaabbbbbaaa","ababbbbaaabbaabbbbaabbbb","baababbababababbbbbbbbbaabbbbaabbbaaaaabbbaabbab","abaabbbabaababbaabbaabaa","aabbabbabbbbaaaaabaababb","baababbaabbaababaabaaaaabbbabbba","aabaaaaabbbbababbbbbaaaabbbabaabaababbab","aaaaaaaaabbaaabababbbbab","baaabbbbabbbaaaaababbbab","abaababaaababaaabababbaaabbbababbbabbbaaaaababbaaaaaabbaaaabbbbb","aabbbaaaabaabaaabbbbbbababbbbaaabbbabbaa","babbaaaaaabbbaaaabbbbbaaaabbabbabbbaabaa","bbbababbbabbbbaaaabaabaa","babbbabbabbbbbaaabbabbbb","aababaaabbaaaaababaaaaaabbbbbbababababbbbaaaabbabaababaababbabbaabbbbaaababababa","bbbabaaabaababbaaabaaaab","baabaababbabbbbbbaabbbbb","bbbbbbabaaababbbabbabbaabaabbbba","bababbabbaabbaaabaababbbabbbaababbabbaaaaababbabbbbbaaabbabbaaaaabaabbba","abbaaabbabbababaabaaabaa","bbaaababaabbabaaabaaabaa","bbabbbaaaaaaaaaaaaaabaab","bbaaaaabbbababbabbbaaabb","aaaaaaabababbabababaaaab","abbabbabbabbbbaabbabbabb","aabaaaaaabbbaaaaaabbabababaaabbbbbabaaabbabaabaabaaaabaa","bbbbbaabbbaaaaaababbbbbbaaaabbababbaabbbabbbbbabbaaaabab","aaabaaaaabbbabaabbaaaaba","aababaabaabbaaaabaaaaaaa","aaaabbbaaaabababaabbbaababbabaaabaaaaaaa","baabbaabbaabaabbbababbbbbbbabbaa","bbbaababbbbbaaaaaaaabaab","bababaabbabbabaabbabbaaa","baabbaaababaaaaabbaaaaaabbbabbbaaabbbaaabbbabaaaaababbbbbbbaaaab","abbaababbaababababaabbaaababbaababbaabbbaaaaaabb","babbabaaaaababbaabbaabaabbbbabba","babbaabbbabbaabbaaaabaab","aababbaaaaabaaabaabbbaabaabbabbaabbbbbbabbaabbbbbababaaa","aaaabbbababaaaaaaabaabbb","bbaababbabbbaaaaababaaabbbabbbbaaabbbbaababbababbbabbabb","aababbbaaaabaaaaabaababb","baaababbabbbbbbbbabaaabbabaabaabbbabbaba","abbabbabaaaaaabbaaabbabbbbbabaababbababbbbbaabba","abbbbabaaabbababbbbaaaba","abbbaababbabbabbbaabbabbbbabaaaa","abbbbaabaabbbabbaaaabbaa","aaabbbbaaaaabbababbaaabbaaaaabbb","bbbbaaabaaabaaabbaabbbaa","babbbabbbabbabaaabbabbbb","ababbbbaabbbabbbaaabbaaa","bbabbbbbababbaabaaaabbbb","bbbababbbbbbbbbabbbabbbb","abbbabbbbababbbbabbabbbb"];

#[derive(Clone,Debug)]
enum Rule {
  Comb(Vec<Vec<usize>>),
  Char(char),
}

fn matches(rules: &HashMap<usize, Rule>, id: usize) -> Vec<String> {
  match &rules[&id] {
    Rule::Char(c) => vec![c.to_string()],
    Rule::Comb(v) => {
      v.iter().flat_map(|p| match p[..] {
        [p] => matches(rules, p),
        [p1,p2] => matches(rules, p1).iter()
          .cartesian_product(matches(rules, p2).iter())
          .map(|(s1,s2)| format!("{}{}", s1, s2))
          .collect(),
        _ => unreachable!()
      }).collect()
    }
  }
}

// Rule 0 has the form:
// 0: 8 11
// 8: 42
// 11: 42 31
// -> 0: 42 42 31
// I.e start with any of the two prefixes, and when with a suffix.
fn check_match_p1(prefixes: &[String], suffixes: &[String], s: &str) -> bool {
  let p_len = match prefixes.iter().find(|&p| s.starts_with(p)) {
    Some(p) => p.len(),
    None => return false,
  };
  let s = &s[p_len..];
  prefixes.iter()
    .cartesian_product(suffixes.iter())
    .filter(|(p1,p2)| s.len() == p1.len() + p2.len())
    .any(|(p1,p2)| s.starts_with(p1) && s.ends_with(p2))
}

// This matches the following:
// 0: 8 11
// 8: 42 | 42 8
// 11: 42 31 | 42 11 31
// We start with 1 or more 42s, which we try in the function check_match_p2
// then we have any number of wrapping 42 x 31, which we try in check_rule_11
fn check_match_p2(prefixes: &[String], suffixes: &[String], s: &str) -> bool {
  prefixes.iter()
    .filter(|&p| s.starts_with(p))
    .map(|p| &s[p.len()..])
    .any(|s|
      check_match_p2(prefixes, suffixes, s)
      || check_rule_11(prefixes, suffixes, s)
    )
}

fn check_rule_11(prefixes: &[String], suffixes: &[String], s: &str) -> bool {
  prefixes.iter()
    .cartesian_product(suffixes.iter())
    .filter(|&(p1,p2)| p1.len() + p2.len() <= s.len())
    .filter(|&(p1,p2)| s.starts_with(p1) && s.ends_with(p2))
    .map(|(p1,p2)| &s[p1.len()..(s.len() - p2.len())])
    .any(|s| s.len() == 0 || check_rule_11(prefixes, suffixes, s))
}

aoc2020::main! {
  let rules = RULES.iter()
    .flat_map(|s| s.split(": "))
    .tuples()
    .map(|(id, s)| {
      let id = id.parse::<usize>().unwrap();
      if s.contains('"') {
        return (id, Rule::Char(s.as_bytes()[1] as char));
      }
      let v = s.split(" | ")
        .map(|p| p.split_whitespace()
          .map(|i| i.parse::<usize>().unwrap())
          .collect()
        ).collect();
      (id, Rule::Comb(v))
    })
    .collect::<HashMap<_,_>>();
  let v42 = matches(&rules, 42);
  let v31 = matches(&rules, 31);
  let p1 = INPUT.iter().filter(|s| check_match_p1(&v42, &v31, &s)).count();
  let p2 = INPUT.iter().filter(|s| check_match_p2(&v42, &v31, &s)).count();
  (p1, p2)
}
