# Changelog
All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.10.0] - 2025-05-18
### Added
- Documentation for `QuantorKind`.
- `QuantorError::EmptyInput`.
### Changed
- `QuantorError` to include the kind of quantifier via the `kind` field for easier diagnostics.
### Fixed
- Logic in `exactly_one`, where an empty input was previously not differentiated from a faulty input.