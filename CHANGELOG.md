# Changelog

## [0.3.4] - 2022-08-17
### Added
- allow to configure file extension

## [0.3.3] - 2022-05-05
### Changed
- Remove debug logging

## [0.3.2] - 2022-05-04
### Changed
- Remove debug logging

## [0.3.1] - 2022-05-03
### Changed
- `FileRotation::rotate` will not unwrap the file name anymore and check if it is pointing to a directory or a file. `FileRotationError::NotAFile` will be returned in that case.

## earlier
### Added
- `FileRotation` and basic interface
