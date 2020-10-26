#![no_std]

/// Expands to the number of token trees in the macro arguments
///
/// ```
/// use count_tts::count_tts;
///
/// assert_eq!(count_tts!(), 0);
/// assert_eq!(count_tts!(a b c), 3);
/// assert_eq!(count_tts!(a (b c)), 2);
/// assert_eq!(count_tts!(a, b, c), 5);
/// ```
#[macro_export]
macro_rules! count_tts {
    () => (0);
    ($one:tt) => (1);
    ($($a:tt $b:tt)+) => (count_tts!($($a)+) << 1);
    ($first:tt $($a:tt $b:tt)+) => (count_tts!($($a)+) << 1 | 1);
}

#[cfg(test)]
mod test {
    use static_assertions::const_assert_eq;

    const_assert_eq!(count_tts!(), 0);
    const_assert_eq!(count_tts!(-), 1);
    const_assert_eq!(count_tts!(--), 2);
    const_assert_eq!(count_tts!(---), 3);
    const_assert_eq!(count_tts!(----), 4);
    const_assert_eq!(count_tts!(-----), 5);
    const_assert_eq!(count_tts!(------), 6);
    const_assert_eq!(count_tts!(-------), 7);
    const_assert_eq!(count_tts!(--------), 8);
    const_assert_eq!(count_tts!(---------), 9);
    const_assert_eq!(count_tts!(----------), 10);
    const_assert_eq!(count_tts!(-----------), 11);
    const_assert_eq!(count_tts!(------------), 12);
    const_assert_eq!(count_tts!(-------------), 13);
    const_assert_eq!(count_tts!(--------------), 14);
    const_assert_eq!(count_tts!(---------------), 15);
}
