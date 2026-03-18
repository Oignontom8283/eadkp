


0.11 - Revision of imports logic :
    - Removed conditional imports, unnecessary since std already provides alloc when OS=YES.
    - Cleaned up imports in the setup macro.

0.12.0 - Refactor/storage.rs
    - Refactored storage.rs for optimization, speed, and modularity. (Major changes)
    - Added new sub modules for centralized Epsilon memoru/object mapping and management.
    - Added errors.rs sub module for centralized error handling and definitions.
    - Improved README documentation (Header and tutorials).
    - Add Rust CI workflow for automated testing.
    - Synced justfile (from eadkp_template)
    - Updated example/testing code file

0.13.0 - ...
    - Refactor input.rs for better documentation and clarity. 
    - Added to_alphanumeric, is_alphanumeric and documentation in input.rs.
    - Update documentation and comments with French
    - MAJOR: Switched to LGPL-3.0 license for allow non-viral integration in external projects.

0.14.0, 0.14.1 - Wayland support
    - feat: add Wayland support to docker-compose

0.15.0_0.15.2 - Cargo release configuration
    - feat: add Cargo release configuration
    - docs: usage instructions for cargo release in release.toml
    - refactor: reorganize dockerfile
    - feat: add cargo-release installation to dockerfile

0.15.3
    - remove: cargo-release config and docker install
    - add: custom script for dumping version