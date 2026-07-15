use std::ops::Rem;

pub struct Matcher<T> {
    predicate: Box<dyn Fn(T) -> bool>,
    substitute: String,
}

impl<T: Copy> Matcher<T> {
    pub fn new<F>(f: F, s: &str) -> Self
    where 
        F: Fn(T) -> bool + 'static,
    {
        Self {
            predicate: Box::new(f),
            substitute: s.to_string(),
        }
    }

    fn apply(&self, value: T) -> Option<&str> {
        if (self.predicate)(value) {
            Some(&self.substitute)
        } else {
            None
        }
    }
}

pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

#[allow(clippy::new_without_default)]
impl<T> Fizzy<T>
where 
    T: Copy + ToString,
{
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where 
        I: IntoIterator<Item = T>,
    {
        iter.into_iter()
            .map(move |n| {
                let out: String = self.matchers
                    .iter()
                    .filter_map(|m| m.apply(n))
                    .collect();

                if out.is_empty() {
                    n.to_string()
                } else {
                    out
                }
            })
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T> 
where 
    T: Copy
        + ToString
        + From<u8>
        + PartialEq
        + Rem<Output = T>
        + 'static,
{
    Fizzy::new()
        .add_matcher(
            Matcher::new(|n| n % T::from(3) == T::from(0), "fizz")
        )
        .add_matcher(
            Matcher::new(|n| n % T::from(5) == T::from(0), "buzz")
        )
}
