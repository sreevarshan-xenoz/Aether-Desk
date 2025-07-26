# Implementation Plan

- [x] 1. Fix core error handling system







  - Update all error variant references from `AppError::Platform` to `AppError::PlatformError`
  - Update all error variant references from `AppError::Wallpaper` to `AppError::WallpaperError`
  - Verify error enum structure in `src/core/error.rs` has correct variant names
  - Test that error handling compiles without variant name conflicts
  - _Requirements: 3.1, 3.2, 3.3, 3.4_

- [x] 2. Convert wallpaper trait system to async





- [x] 2.1 Update Wallpaper trait definition to async


  - Modify `src/wallpapers/mod.rs` to make all Wallpaper trait methods async
  - Add `async_trait` annotation to the trait definition
  - Update method signatures to return `AppResult<()>` for async operations
  - _Requirements: 2.1, 2.2, 2.4_

- [x] 2.2 Update all wallpaper implementations to async


  - Convert all wallpaper struct implementations to use async methods
  - Add `.await` calls to wallpaper manager method invocations
  - Apply `async_trait` annotation to all implementation blocks
  - Fix any async/sync mismatches in wallpaper operation calls
  - _Requirements: 2.1, 2.2, 2.4_

- [x] 2.3 Fix async mutex handling in wallpaper system


  - Replace `std::sync::Mutex` with `tokio::sync::Mutex` in async contexts
  - Update mutex guard usage to prevent borrow conflicts in async functions
  - Implement proper async mutex locking patterns throughout wallpaper code
  - _Requirements: 2.3, 7.3_

- [x] 3. Complete platform manager trait implementations





- [x] 3.1 Implement missing Windows platform methods


  - Add `get_current_wallpaper` method to Windows platform manager
  - Implement placeholder return value `Ok(None)` for initial compilation
  - Ensure all WallpaperManager trait methods are implemented
  - _Requirements: 4.1, 4.3, 8.2_

- [x] 3.2 Fix Linux platform manager implementation


  - Correct lifetime annotations on async trait methods
  - Fix return type of `get_current_wallpaper` to return `PathBuf`
  - Add proper `async_trait` usage to Linux platform implementation
  - _Requirements: 4.1, 4.2, 4.3_

- [x] 3.3 Complete Hyprland platform manager


  - Add missing `get_current_wallpaper` method implementation
  - Implement placeholder return value `Ok(None)` for compilation
  - Ensure all trait methods satisfy the WallpaperManager interface
  - _Requirements: 4.1, 4.3, 8.4_

- [x] 4. Update UI system for egui compatibility





- [x] 4.1 Fix deprecated egui method calls


  - Remove `.text_color()` method calls from UI components in `src/ui/app.rs`
  - Replace with current egui styling methods for text color
  - Update any other deprecated egui API usage found during compilation
  - _Requirements: 5.1, 5.3_

- [x] 4.2 Update NativeOptions configuration


  - Replace `initial_window_size` with `viewport.inner_size` in `src/main.rs`
  - Update any other deprecated NativeOptions field names
  - Ensure window configuration uses valid egui NativeOptions structure
  - _Requirements: 5.2_

- [x] 4.3 Complete WidgetConfig struct initialization


  - Add missing fields to `WidgetConfig` initialization (background_color, opacity)
  - Ensure all required struct fields are properly initialized
  - Fix any incomplete struct literal errors in widget creation
  - _Requirements: 5.4_

- [x] 5. Resolve import and type management issues





- [x] 5.1 Add missing trait imports


  - Add `chrono::Datelike` import to modules that use date functionality
  - Import any other missing traits required for method calls
  - Ensure all trait methods are in scope where used
  - _Requirements: 6.2_

- [x] 5.2 Fix type conversion issues


  - Convert `Cow<str>` to `OsStr` using `.to_string()` before command arguments
  - Fix any other type conversion errors in platform-specific command execution
  - Ensure proper type bounds are satisfied for generic parameters
  - _Requirements: 6.3, 8.1, 8.3_

- [x] 5.3 Clean up unused imports


  - Remove unused import statements to reduce compiler warnings
  - Ensure only necessary imports are included in each module
  - Verify import cleanup doesn't break any functionality
  - _Requirements: 6.1_

- [x] 6. Fix memory management and borrowing issues





- [x] 6.1 Resolve borrow checker conflicts


  - Use separate scopes for mutex guards to avoid mutable/immutable borrow conflicts
  - Clone values when necessary to prevent lifetime issues
  - Fix any temporary value lifetime problems
  - _Requirements: 7.1, 7.2_

- [x] 6.2 Implement proper Clone traits where needed


  - Add Clone implementations or derive Clone for types that need cloning
  - Use explicit variable bindings for complex expressions to manage lifetimes
  - Ensure all cloning operations are appropriate and don't cause performance issues
  - _Requirements: 7.4_

- [x] 6.3 Fix async mutex deadlock prevention


  - Ensure mutex guards are dropped before await points
  - Implement proper async mutex usage patterns to prevent deadlocks
  - Test that async operations don't hold locks across await boundaries
  - _Requirements: 7.3_

- [x] 7. Verify compilation success and warning reduction


  - Run `cargo build` to ensure all compilation errors are resolved
  - Verify warning count is reduced to acceptable levels (currently 17 warnings, down from 61 errors)
  - Test that all existing functionality is preserved after fixes
  - Run basic smoke tests to ensure application still functions correctly
  - _Requirements: 1.1, 1.2, 1.3_

- [x] 8. Address remaining dead code warnings





  - Review and remove unused code or add `#[allow(dead_code)]` attributes where appropriate
  - Focus on unused methods in performance monitoring, plugin system, and widget system
  - Consider if unused code should be removed or is intended for future use
  - _Requirements: 1.2_

- [x] 9. Final verification and cleanup








  - Run comprehensive tests to ensure all functionality works as expected
  - Verify that the application starts and displays the UI correctly
  - Test basic wallpaper setting functionality across different types
  - Document any remaining limitations or known issues
  - _Requirements: 1.1, 1.2, 1.3_