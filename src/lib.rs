#![cfg_attr(not(feature = "std"), no_std)]

pub mod data_protocol;
pub mod macro_protocol;

#[cfg(test)]
mod tests {
    use crate::data_protocol;
    use crate::macro_protocol;
}