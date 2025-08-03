#![no_std]

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

#[cfg(feature = "std")]
extern crate std;

pinocchio_pubkey::declare_id!("ENrRns55VechXJiq4bMbdx7idzabctvaEJoYeWxRNe7Y");
