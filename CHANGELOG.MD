


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