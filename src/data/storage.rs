//! `memory_storage` module is responsible for managing connection to various
//! memory backends, such as redis and various rust hashmaps. provides a single
//! interchangeable api to easily switch among them.
//! also contains other data structures such as buffer with automatic element
//! expiration.

pub mod expiration_buffer;
pub mod memory_map;

pub mod backend;
