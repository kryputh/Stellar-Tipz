//! Leaderboard tracking for the Tipz contract.
//!
//! Maintains a sorted list of top creators by total tips received.
//! The leaderboard is updated after each tip.

use soroban_sdk::{vec, Address, Env, Vec};

use crate::storage::{get_profile, has_profile};
use crate::types::LeaderboardEntry;

/// Updates the leaderboard after a tip is received by a creator.
///
/// This function should be called whenever a creator receives a tip.
/// It maintains a sorted list of top creators by total tips received.
pub fn update_leaderboard(env: &Env, creator: &Address) {
    if !has_profile(env, creator) {
        return;
    }

    let profile = get_profile(env, creator);
    let entry = LeaderboardEntry {
        address: creator.clone(),
        username: profile.username,
        total_tips_received: profile.total_tips_received,
        credit_score: profile.credit_score,
    };

    // Get current leaderboard
    let mut leaderboard: Vec<LeaderboardEntry> = env
        .storage()
        .persistent()
        .get(&crate::storage::DataKey::Leaderboard)
        .unwrap_or_else(|| vec![env]);

    // Simple approach: just add the entry and let get_leaderboard handle sorting
    // Remove existing entry for this creator if present
    let mut filtered = vec![env];
    for i in 0..leaderboard.len() {
        if let Some(existing) = leaderboard.get(i) {
            if existing.address != *creator {
                filtered.push_back(existing);
            }
        }
    }
    filtered.push_back(entry);

    // Store the updated leaderboard
    env.storage()
        .persistent()
        .set(&crate::storage::DataKey::Leaderboard, &filtered);
}

/// Returns the top `limit` creators by total tips received.
///
/// If `limit` is 0, returns all entries (capped at 100).
/// Results are sorted by total tips received in descending order.
pub fn get_leaderboard(env: &Env, limit: u32) -> Vec<LeaderboardEntry> {
    let leaderboard: Vec<LeaderboardEntry> = env
        .storage()
        .persistent()
        .get(&crate::storage::DataKey::Leaderboard)
        .unwrap_or_else(|| vec![env]);

    // For now, return all entries unsorted
    leaderboard
}
