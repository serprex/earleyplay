use std::hash::Hash;

pub trait Terminal: Copy + Clone + Hash + Eq + PartialEq {
    fn test(&self, c: u8) -> bool;
}
pub trait Grammar: Copy + Clone + Hash + Eq + PartialEq {
    type Term: Terminal;
    fn variants(&self) -> usize;
    fn len(&self, idx: usize) -> usize;
    fn getatom(&self, idx: usize, n: usize) -> RuleAtom<Self>;
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum RuleAtom<G: Grammar>
{
    Grammar(G),
    Terminal(G::Term),
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Rule<G: Grammar> {
    gram: G,
    idx: usize,
}

impl<G: Grammar> Rule<G> {
    pub fn new(gram: G, idx: usize) -> Self {
        Self {
            gram: gram,
            idx: idx,
        }
    }
    pub fn getatom(&self, idx: usize) -> RuleAtom<G> {
        self.gram.getatom(self.idx, idx)
    }
    pub fn len(&self) -> usize {
        self.gram.len(self.idx)
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum FactorSumTerminal {
    AddSub,
    MulDiv,
    Digit,
    Lp,
    Rp,
}
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum FactorSumGrammar {
    Sum,
    Product,
    Factor,
    Number,
}

// Sum = Sum +- Product | Product
// Product = Product */ Factor
// Factor = ( Sum ) | Number
// Number = Digit | Digit Number

impl Terminal for FactorSumTerminal {
    fn test(&self, c: u8) -> bool {
        match *self {
            FactorSumTerminal::AddSub => c == b'+' || c == b'-',
            FactorSumTerminal::MulDiv => c == b'*' || c == b'/',
            FactorSumTerminal::Digit => c >= b'0' && c <= b'9',
            FactorSumTerminal::Lp => c == b'(',
            FactorSumTerminal::Rp => c == b')',
        }
    }
}

impl Grammar for FactorSumGrammar {
    type Term = FactorSumTerminal;
    fn variants(&self) -> usize {
        match *self {
            FactorSumGrammar::Sum => 2,
            FactorSumGrammar::Product => 2,
            FactorSumGrammar::Factor => 2,
            FactorSumGrammar::Number => 2,
        }
    }
    fn len(&self, idx: usize) -> usize {
        match *self {
            FactorSumGrammar::Sum => if idx == 0 { 3 } else { 1 },
            FactorSumGrammar::Product => if idx == 0 { 3 } else { 1 },
            FactorSumGrammar::Factor => if idx == 0 { 3 } else { 1 },
            FactorSumGrammar::Number => if idx == 0 { 2 } else { 1 },
        }
    }
    fn getatom(&self, idx: usize, n: usize) -> RuleAtom<FactorSumGrammar> {
        match *self {
            FactorSumGrammar::Sum => {
                match (idx, n) {
                    (0, 0) => RuleAtom::Grammar(FactorSumGrammar::Sum),
                    (0, 1) => RuleAtom::Terminal(FactorSumTerminal::AddSub),
                    (0, 2) => RuleAtom::Grammar(FactorSumGrammar::Product),
                    (1, 0) => RuleAtom::Grammar(FactorSumGrammar::Product),
                    _ => unreachable!(),
                }
            },
            FactorSumGrammar::Product => {
                match (idx, n) {
                    (0, 0) => RuleAtom::Grammar(FactorSumGrammar::Product),
                    (0, 1) => RuleAtom::Terminal(FactorSumTerminal::MulDiv),
                    (0, 2) => RuleAtom::Grammar(FactorSumGrammar::Factor),
                    (1, 0) => RuleAtom::Grammar(FactorSumGrammar::Factor),
                    _ => unreachable!(),
                }
            },
            FactorSumGrammar::Factor => {
                match (idx, n) {
                    (0, 0) => RuleAtom::Terminal(FactorSumTerminal::Lp),
                    (0, 1) => RuleAtom::Grammar(FactorSumGrammar::Sum),
                    (0, 2) => RuleAtom::Terminal(FactorSumTerminal::Rp),
                    (1, 0) => RuleAtom::Grammar(FactorSumGrammar::Number),
                    _ => unreachable!(),
                }
            },
            FactorSumGrammar::Number => {
                match (idx, n) {
                    (0, 0) => RuleAtom::Terminal(FactorSumTerminal::Digit),
                    (0, 1) => RuleAtom::Grammar(FactorSumGrammar::Number),
                    (1, 0) => RuleAtom::Terminal(FactorSumTerminal::Digit),
                    _ => unreachable!(),
                }
            },
        }
    }
}

#[derive(Copy, Clone, Hash)]
struct Item<G: Grammar> {
    rule: Rule<G>,
    dot: usize,
    start: usize,
}

impl<G: Grammar> Item<G> {
    pub fn new(rule: Rule<G>, dot: usize, start: usize) -> Self
    {
        Self {
            rule: rule,
            dot: dot,
            start: start,
        }
    }
    pub fn incr(&self) -> Self
    {
        Self {
            rule: self.rule,
            dot: self.dot + 1,
            start: self.start,
        }
    }
}

pub fn parse(input: &[u8]) -> bool {
    let mut statesets = vec![vec![
        Item::new(Rule::new(FactorSumGrammar::Sum, 0), 0, 0),
        Item::new(Rule::new(FactorSumGrammar::Sum, 1), 0, 0),
    ]];
    let mut i = 0;
    while i < statesets.len() {
        let mut j = 0;
        while j < statesets[i].len() {
            let item = statesets[i][j];
            let rule = item.rule;
            println!("{}: {}", rule.len(), item.dot);
            if item.dot == rule.len() {
                let mut k = 0;
                while k < statesets[item.start].len() {
                    let willpush = {
                        let x = &statesets[k][item.start];
                        let r = x.rule;
                        x.dot < r.len() && r.getatom(x.dot) == item.rule.getatom(0)
                    };
                    if willpush {
                        statesets[i].push(item.incr());
                    }
                    k += 1;
                }
            } else {
                match rule.getatom(item.dot) {
                    RuleAtom::Terminal(term) => {
                        if i < input.len() {
                            if term.test(input[i]) {
                                if i == statesets.len() - 1 {
                                    statesets.push(vec![item.incr()]);
                                } else {
                                    statesets[i + 1].push(item.incr());
                                }
                            }
                        }
                    }
                    RuleAtom::Grammar(gram) => {
                        let mut stateset = &mut statesets[i];
                        for x in 0..gram.variants() {
                            stateset.push(Item::new(Rule::new(gram, x), 0, i));
                        }
                    }
                }
            }
            j += 1;
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test = b"1+(2*3-4)";
        assert!(parse(test));
    }
}
