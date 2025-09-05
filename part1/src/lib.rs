/// Returns the sqrt of `n`
///
/// Using built-in square root functions is not allowed.
pub fn sqrt(n: u32) -> u32 {
    let mut r = 1;
    while r * r <= n {
        r += 1;
    }
    return r - 1;
}

/// Consumes a sorted list of integers and a query integer. Returns the index of the query integer
///
/// Note that a 3-valued comparison between numbers `a` and `b` can be done easily:
/// ```rust,ignore
/// match a.cmp(&b) {
///    std::cmp::Ordering::Less => { ... },
///    std::cmp::Ordering::Greater => { ... },
///    std::cmp::Ordering::Equal => { ... },
/// }
/// ```
pub fn binary_search(arr: &[i32], query: i32) -> Option<u32> {
    let mut lo = 0;
    let mut hi = arr.len();
    while lo < hi {
        let guess = (lo + hi) / 2;
        match arr[guess].cmp(&query) {
            std::cmp::Ordering::Equal => { return Some(guess.try_into().unwrap()); }
            std::cmp::Ordering::Less => { lo = guess + 1; }
            std::cmp::Ordering::Greater => { hi = guess; }
        }
    }
    None
}

/// Consumes a list of numbers representing daily rainfall. The list may contain -999 signifying
/// the end of data of interest. Returns the average of non-negative values up to the first
/// occurrence of -999 (if it occurs). There may be negative numbers other -999 in the list.
/// Returns None if the average is incomputable.
///
/// example: rainfall([6, 8, -1, 1, -999, 4, 5, 6]) -> Some(5.0)
pub fn rainfall(values: &[i32]) -> Option<f64> {
    let mut total: f64 = 0.0;
    let mut n: f64 = 0.0;
    for x in values {
        if *x >= 0 {
            total += <i32 as TryInto<f64>>::try_into(*x).unwrap();
            n += 1.0;
        }
        if *x == -999 { break; }
    }
    if n > 0.0 { Some(total / n) }
    else { None }
}
