/// Simple macro to insert variables into an expression in compile-time via [`format!`] macro
///
/// ## Example usage
/// ```
/// use rexpr::eval;
/// let result = rexpr::eval!("{} + {} - {}", 5, 1, 3).unwrap();
/// assert_eq!(result, 3.0);
/// ```
#[macro_export]
macro_rules! eval {
    ($fmt:literal, $($arg:expr),*) => {{
        let expr_string = format!($fmt, $($arg),*);
        rexpr::eval(&expr_string)
    }};
}
