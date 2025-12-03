# Changelog

All notable changes to this project will be documented in this file.

## [0.1.2]

### Changed

* Generated request, response, API enum, and Init structs now derive `PartialEq` instead of `PartialEq, Eq`. This allows using types like `f64` and `Vec<f64>` in method signatures, which implement `PartialEq` but not `Eq`.

## [0.1.1]

* Added comprehensive documentation for the `#[pool_item]` attribute macro

## [0.1.0]

* Initial release of the `#[pool_item]` procedural macro
* Generates `PoolItem` trait implementation from annotated `impl` blocks
* `#[messaging(Request, Response)]` attribute for defining message handlers
* Optional `Init = "CustomType"` parameter for custom initialization types
* Optional `Shutdown = "method_name"` parameter for shutdown hooks
* Support for generic pool item types
