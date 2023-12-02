#[macro_export]
macro_rules! regex {
    ($re:expr $(,)?) => {{
        static LAZY: once_cell::sync::Lazy<regex::Regex> =
            once_cell::sync::Lazy::new(|| regex::Regex::new($re).unwrap());
        &LAZY
    }};
}
