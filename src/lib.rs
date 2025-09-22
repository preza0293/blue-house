#![no_std]
mod common;
#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
mod swap_handler;

#[cfg(feature = "std")]
extern crate std;

pinocchio_pubkey::declare_id!("ENrRns55VechXJiq4bMbdx7idzabctvaEJoYeWxRNe7Y");
