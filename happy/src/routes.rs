use regex::{Captures, Regex, RegexSet};
use hyper::Method;

#[derive(Clone)]
pub struct RouteBuilder<T> {
    set: RegexSet,
    regex_and_verbs: Vec<(Option<Method>, Regex)>,
    values: Vec<T>,
}

impl<T> RouteBuilder<T> {
    pub fn new<I: Iterator<Item = (Option<Method>, String, T)>>(routes: I) -> RouteBuilder<T> {
        let mut all_regex_strings = vec![];
        let mut regex_and_verbs = vec![];
        let mut values = vec![];

        for (method, regex, value) in routes {
            all_regex_strings.push(regex.clone());
            regex_and_verbs.push((method, Regex::new(&regex).unwrap()));
            values.push(value);
        }

        let regex_set = RegexSet::new(all_regex_strings).unwrap();

        RouteBuilder {
            set: regex_set,
            regex_and_verbs: regex_and_verbs,
            values: values,
        }
    }

    pub fn match_path<'a, 'b>(
        &'a self,
        in_verb: Method,
        target: &'b str,
    ) -> Option<(&'a T, Captures<'b>)> {
        let mut best_capture = None;
        let mut best_index = None;
        for index in self.set.matches(target) {
            let &(ref verb, ref regex) = &self.regex_and_verbs[index];
            if !(verb.is_none() || verb.as_ref() == Some(&in_verb)) {
                continue;
            }

            let capture = regex.captures(target);

            let (nc, ni) = match (best_capture, capture) {
                (None, new) => (new, Some(index)),
                (old, None) => (old, best_index),
                (Some(old), Some(new)) => if new.len() > old.len() {
                    (Some(new), Some(index))
                } else {
                    (Some(old), best_index)
                },
            };
            best_capture = nc;
            best_index = ni;
        }

        match (best_capture, best_index) {
            (Some(bc), Some(bi)) => Some((&self.values[bi], bc)),
            (None, None) => None,
            _ => unreachable!(),
        }
    }
}
