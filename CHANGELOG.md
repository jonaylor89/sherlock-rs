# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [3.0.0](https://github.com/jonaylor89/sherlock-rs/compare/v2.0.0...v3.0.0) - 2024-09-11

### Other

- prefer &[T] over &Vec<T> for params
- cargo clippy fix
- use fs helpers
- use include_str! and a real json file
- misc changes
- pull out an async function to check for a user at a site
- unneeded clone
- no need for lazy init of simple static arrays
- Convert timeout to a Duration sooner rather than later
- Use Arc<str> over Arc<String>
- Use enum to better represent target error types

## [2.0.0](https://github.com/jonaylor89/sherlock-rs/compare/v1.0.0...v2.0.0) - 2024-08-26

### Other
- refactor save results
- release plz
