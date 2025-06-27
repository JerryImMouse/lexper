/// Simple macro to insert variables into an expression in compile-time via [`format!`] macro
///
/// ## Example usage
/// ```
/// use lexper::eval;
/// let result = lexper::eval!("{} + {} - {}", 5, 1, 3).unwrap();
/// assert_eq!(result, 3.0);
/// ```
#[macro_export]
macro_rules! eval {
    ($fmt:literal, $($arg:expr),*) => {{
        let expr_string = format!($fmt, $($arg),*);
        lexper::eval(&expr_string)
    }};
}
