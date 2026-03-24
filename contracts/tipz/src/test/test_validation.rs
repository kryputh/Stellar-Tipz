//! Tests for username validation (issue #4).

#![cfg(test)]

use soroban_sdk::{Env, String};

use crate::errors::ContractError;
use crate::validation::validate_username;

// ───────────────────────── helpers ──────────────────────────

fn s(env: &Env, v: &str) -> String {
    String::from_str(env, v)
}

// ───────────────────── valid usernames ──────────────────────

#[test]
fn valid_simple_username() {
    let env = Env::default();
    assert_eq!(validate_username(&s(&env, "alice")), Ok(()));
}

#[test]
fn valid_min_length() {
    let env = Env::default();
    assert_eq!(validate_username(&s(&env, "abc")), Ok(()));
}

#[test]
fn valid_max_length() {
    let env = Env::default();
    // 32 characters
    assert_eq!(
        validate_username(&s(&env, "abcdefghijklmnopqrstuvwxyz012345")),
        Ok(())
    );
}

#[test]
fn valid_with_digits() {
    let env = Env::default();
    assert_eq!(validate_username(&s(&env, "user123")), Ok(()));
}

#[test]
fn valid_with_underscores() {
    let env = Env::default();
    assert_eq!(validate_username(&s(&env, "hello_world")), Ok(()));
}

#[test]
fn valid_single_underscore_in_middle() {
    let env = Env::default();
    assert_eq!(validate_username(&s(&env, "a_b")), Ok(()));
}

#[test]
fn valid_multiple_separated_underscores() {
    let env = Env::default();
    assert_eq!(validate_username(&s(&env, "a_b_c_d")), Ok(()));
}

#[test]
fn valid_ends_with_digit() {
    let env = Env::default();
    assert_eq!(validate_username(&s(&env, "user42")), Ok(()));
}

// ──────────────── too short / too long ──────────────────────

#[test]
fn reject_too_short_one_char() {
    let env = Env::default();
    assert_eq!(
        validate_username(&s(&env, "a")),
        Err(ContractError::InvalidUsername)
    );
}

#[test]
fn reject_too_short_two_chars() {
    let env = Env::default();
    assert_eq!(
        validate_username(&s(&env, "ab")),
        Err(ContractError::InvalidUsername)
    );
}

#[test]
fn reject_too_long() {
    let env = Env::default();
    // 33 characters
    assert_eq!(
        validate_username(&s(&env, "abcdefghijklmnopqrstuvwxyz0123456")),
        Err(ContractError::InvalidUsername)
    );
}

#[test]
fn reject_empty() {
    let env = Env::default();
    assert_eq!(
        validate_username(&s(&env, "")),
        Err(ContractError::InvalidUsername)
    );
}

// ──────────────── start character rules ─────────────────────

#[test]
fn reject_starts_with_digit() {
    let env = Env::default();
    assert_eq!(
        validate_username(&s(&env, "1alice")),
        Err(ContractError::InvalidUsername)
    );
}

#[test]
fn reject_starts_with_underscore() {
    let env = Env::default();
    assert_eq!(
        validate_username(&s(&env, "_alice")),
        Err(ContractError::InvalidUsername)
    );
}

// ──────────────── end character rules ───────────────────────

#[test]
fn reject_ends_with_underscore() {
    let env = Env::default();
    assert_eq!(
        validate_username(&s(&env, "alice_")),
        Err(ContractError::InvalidUsername)
    );
}

// ──────────── consecutive underscores ───────────────────────

#[test]
fn reject_consecutive_underscores() {
    let env = Env::default();
    assert_eq!(
        validate_username(&s(&env, "alice__bob")),
        Err(ContractError::InvalidUsername)
    );
}

#[test]
fn reject_triple_underscores() {
    let env = Env::default();
    assert_eq!(
        validate_username(&s(&env, "a___b")),
        Err(ContractError::InvalidUsername)
    );
}

// ──────────── disallowed characters ─────────────────────────

#[test]
fn reject_uppercase() {
    let env = Env::default();
    assert_eq!(
        validate_username(&s(&env, "Alice")),
        Err(ContractError::InvalidUsername)
    );
}

#[test]
fn reject_uppercase_middle() {
    let env = Env::default();
    assert_eq!(
        validate_username(&s(&env, "aLice")),
        Err(ContractError::InvalidUsername)
    );
}

#[test]
fn reject_space() {
    let env = Env::default();
    assert_eq!(
        validate_username(&s(&env, "ali ce")),
        Err(ContractError::InvalidUsername)
    );
}

#[test]
fn reject_hyphen() {
    let env = Env::default();
    assert_eq!(
        validate_username(&s(&env, "ali-ce")),
        Err(ContractError::InvalidUsername)
    );
}

#[test]
fn reject_dot() {
    let env = Env::default();
    assert_eq!(
        validate_username(&s(&env, "ali.ce")),
        Err(ContractError::InvalidUsername)
    );
}

#[test]
fn reject_at_sign() {
    let env = Env::default();
    assert_eq!(
        validate_username(&s(&env, "ali@ce")),
        Err(ContractError::InvalidUsername)
    );
}

#[test]
fn reject_special_chars() {
    let env = Env::default();
    assert_eq!(
        validate_username(&s(&env, "ali$ce")),
        Err(ContractError::InvalidUsername)
    );
}

// ──────────── boundary / edge cases ─────────────────────────

#[test]
fn valid_exactly_three_letters() {
    let env = Env::default();
    assert_eq!(validate_username(&s(&env, "abc")), Ok(()));
}

#[test]
fn valid_letter_digit_letter() {
    let env = Env::default();
    assert_eq!(validate_username(&s(&env, "a1b")), Ok(()));
}

#[test]
fn reject_all_underscores() {
    let env = Env::default();
    assert_eq!(
        validate_username(&s(&env, "___")),
        Err(ContractError::InvalidUsername)
    );
}

#[test]
fn reject_all_digits() {
    let env = Env::default();
    assert_eq!(
        validate_username(&s(&env, "123")),
        Err(ContractError::InvalidUsername)
    );
}
