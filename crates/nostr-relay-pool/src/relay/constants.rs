// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

//! Relay constants

use core::time::Duration;

/// Default send timeout
// IF CHANGED, REMEMBER TO UPDATE THE DOCS!
pub const DEFAULT_SEND_TIMEOUT: Duration = Duration::from_secs(20);

/// Max relay size
pub const MAX_MESSAGE_SIZE: u32 = 5 * 1024 * 1024; // 5 MB
/// Max event size
pub const MAX_EVENT_SIZE: u32 = 70 * 1024; // 70 kB
/// Max event size for contact list kind
pub const MAX_CONTACT_LIST_EVENT_SIZE: u32 = 840 * 1024; // 840 kB

pub(super) const DEFAULT_RETRY_SEC: u64 = 10;
pub(super) const MIN_RETRY_SEC: u64 = 5;
pub(super) const MAX_ADJ_RETRY_SEC: u64 = 120;

pub(super) const NEGENTROPY_FRAME_SIZE_LIMIT: u64 = 60_000; // Default frame limit is 128k. Halve that (hex encoding) and subtract a bit (JSON msg overhead)
pub(super) const NEGENTROPY_HIGH_WATER_UP: usize = 100;
pub(super) const NEGENTROPY_LOW_WATER_UP: usize = 50;
pub(super) const NEGENTROPY_BATCH_SIZE_DOWN: usize = 50;

pub(super) const MIN_ATTEMPTS: usize = 1;
pub(super) const MIN_UPTIME: f64 = 0.90;

pub(super) const PING_INTERVAL: Duration = Duration::from_secs(50); // Used also for latency calculation

pub(super) const WEBSOCKET_TX_TIMEOUT: Duration = Duration::from_secs(10);

/// Maximum number of reads to be saved in memory to calculate latency
#[cfg(not(target_arch = "wasm32"))]
pub const LATENCY_MAX_VALUES: usize = 50;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) const LATENCY_MIN_READS: usize = 3;
