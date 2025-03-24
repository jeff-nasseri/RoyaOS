//! Test module for the RoyaOS kernel
//!
//! This module contains comprehensive unit tests for the kernel functionality,
//! following Test-Driven Development (TDD) principles. The tests are organized
//! into logical categories to ensure complete coverage of kernel features.

mod kernel_tests;
mod syscall_tests;
mod subsystem_tests;

// Re-export test utilities for use in other test modules
pub(crate) mod test_utils;
