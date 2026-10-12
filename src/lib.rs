//! # DW13 — Wagon Miles
//!
//! ```text
//! Wagon 0 total: 36
//! ```
//!
//! The grid holds miles travelled: one row per wagon, one column per day.
//!
//! Do not edit `src/main.rs`; the bugs are in the library files.

/// Total miles travelled by one wagon (one row of the grid) over all three days.
///
/// ```
/// use dw13::wagon_total;
///
/// let miles = [[12, 15, 9], [10, 11, 14], [8, 13, 16]];
/// assert_eq!(wagon_total(&miles, 0), 36);
/// ```
///
/// ```
/// use dw13::wagon_total;
///
/// let miles = [[12, 15, 9], [10, 11, 14], [8, 13, 16]];
/// assert_eq!(wagon_total(&miles, 2), 37);
/// ```
///
/// ```
/// use dw13::wagon_total;
///
/// let miles = [[12, 15, 9], [10, 11, 14], [8, 13, 16]];
/// assert_eq!(wagon_total(&miles, 1), 35);
/// ```
pub fn wagon_total(miles: &[[u32; 3]; 3], wagon: usize) -> u32 {
    let mut total = 0;
    for day in 0..3 {
        total += miles[wagon][day];
    }
    total
}

