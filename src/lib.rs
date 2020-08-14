#![no_std]

/// Emulates a `switch` statement.
///
/// The syntax is similar to `match` except that every left-side expression is
/// interpreted as an expression rather than a pattern. The expression to
/// compare against must be at the beginning with a semicolon. A default case
/// is required at the end with a `_`, similar to `match`.
///
/// Example:
///
/// ```
/// use switch_statement::switch;
///
/// const A: u32 = 1 << 0;
/// const B: u32 = 1 << 1;
///
/// let n = 3;
/// let val = switch! { n;
///     A => false,
///     // this is a bitwise OR
///     A | B => true,
///     _ => false,
/// };
/// assert!(val);
/// ```
#[macro_export]
macro_rules! switch {
    ($v:expr; $($a:expr => $b:expr,)* _ => $e:expr,) => {
        match $v {
            $(v if v == $a => $b,)*
            _ => $e,
        }
    };

    ($v:expr; $($a:expr => $b:expr),* , _ => $e:expr) => (switch!($v; $($a => $b,)* _ => $e,));
}

#[cfg(test)]
mod tests {
    const A: u32 = 1 << 0;
    const B: u32 = 1 << 1;
    const C: u32 = 1 << 2;
    const D: u32 = 1 << 3;

    #[test]
    fn it_works() {
        let v = switch! { A | B;
            A => false,
            B | C => false,
            A | B => true,
            C | D => {
                unreachable!();
            },
            _ => false,
        };
        assert!(v);
    }

    #[test]
    fn no_trailing_comma() {
        let v = switch! { 1;
            1 => true,
            _ => false
        };
        assert!(v);
    }
}
