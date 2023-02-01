//! carte_library
//! 

/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = carte_library::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```

pub fn add_one(x: i32) -> i32 {
    x + 1
}
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue
    }
    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green, 
        Purple
    }
}

pub mod utils {
    use crate::kinds::*;
    pub fn mix(c1: PrimaryColor, c2:  PrimaryColor) -> SecondaryColor {
        unimplemented!()
    }
}