# Changelog

## [0.3.1] - 2022-05-03
### Changed
- `FileRotation::rotate` will not unwrap the file name anymore and check if it is pointing to a directory or a file. `FileRotationError::NotAFile` will be returned in that case.

## earlier
### Added
- `FileRotation` and basic interface
