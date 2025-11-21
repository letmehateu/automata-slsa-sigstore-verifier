//! Fetcher utilities for retrieving trust bundles
//!
//! This module provides helper functions for fetching Fulcio certificate chains
//! from external sources. These are utility functions that clients can use to
//! obtain the necessary trust bundles for verification.
//!
//! **Note**: The verification library itself does not fetch data. Clients are
//! responsible for fetching and providing certificate chains to the verifier.

pub mod trust_bundle;
