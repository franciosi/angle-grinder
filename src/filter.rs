#[derive(Debug)]
pub enum Filter {
    And(Vec<Filter>),
    Or(Vec<Filter>),
    Not(Box<Filter>),
    Keyword(regex::Regex),
}

impl Filter {
    pub fn matches(&self, inp: &str) -> bool {
        match self {
            Filter::Keyword(regex) => regex.is_match(inp),
            Filter::And(clauses) => clauses.iter().all(|clause| clause.matches(inp)),
            Filter::Or(clauses) => clauses.iter().any(|clause| clause.matches(inp)),
            Filter::Not(clause) => !clause.matches(inp),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lang::Keyword;

    pub fn from_str(inp: &str) -> Filter {
        Filter::Keyword(Keyword::new_exact(inp.to_owned()).to_regex())
    }

    #[test]
    fn filter_and() {
        let filt = Filter::And(vec![from_str("abc"), from_str("cde")]);
        assert!(filt.matches("abc cde"));
        assert!(!filt.matches("abc"));
    }

    #[test]
    fn filter_or() {
        let filt = Filter::Or(vec![from_str("abc"), from_str("cde")]);
        assert!(filt.matches("abc cde"));
        assert!(filt.matches("abc"));
        assert!(filt.matches("cde"));
        assert!(!filt.matches("def"));
    }

    #[test]
    fn filter_wildcard() {
        let filt = Filter::And(vec![]);
        assert!(filt.matches("abc"));
    }

    #[test]
    fn filter_not() {
        let filt = Filter::And(vec![from_str("abc"), from_str("cde")]);
        let filt = Filter::Not(Box::new(filt));
        assert!(!filt.matches("abc cde"));
        assert!(filt.matches("abc"));
    }

    #[test]
    fn filter_complex() {
        let filt_letters = Filter::And(vec![from_str("abc"), from_str("cde")]);
        let filt_numbers = Filter::And(vec![from_str("123"), from_str("456")]);
        let filt = Filter::Or(vec![filt_letters, filt_numbers]);
        assert!(filt.matches("abc cde"));
        assert!(!filt.matches("abc"));
        assert!(filt.matches("123 456"));
        assert!(!filt.matches("123 cde"));
    }
}
