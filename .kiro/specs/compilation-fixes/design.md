# Design Document

## Overview

This design addresses the systematic resolution of 61 compilation errors in the Aether-Desk project to restore it to a buildable state while maintaining all existing functionality. The errors span eight critical areas: compilation success, async/await handling, error variant completeness, trait implementation completeness, UI framework compatibility, import/type management, memory management and borrowing, and platform-specific code handling. The solution involves a phased approach that prioritizes core infrastructure fixes before addressing higher-level functionality, ensuring each change maintains backward compatibility and doesn't introduce new issues.

## Architecture

### Error Handling Architecture

The current `AppError` enum in `src/core/error.rs` needs to be updated to include proper error variants for platform-specific and wallpaper-related errors. The design decision is to ensure all error types have dedicated variants (`Platform`, `Wallpaper`) and that all error creation throughout the codebase uses the correct variant names. This provides consistent error reporting and proper error categorization as required by Requirement 3.

**Design Rationale:** Centralized error handling with specific variants allows for better error categorization, debugging, and user-facing error messages while maintaining type safety.

### Async/Await Architecture

The wallpaper trait system has a fundamental async/sync mismatch that needs systematic resolution:
- `WallpaperManager` trait methods are async
- `Wallpaper` trait methods are sync but try to call async methods
- Mutex handling in async contexts requires proper async-compatible mutexes

**Design Decision:** Convert the entire `Wallpaper` trait to async and update all implementations to use `async_trait`. This ensures proper async/await handling throughout the system as required by Requirement 2.

**Design Rationale:** Making the trait consistently async prevents blocking operations and allows for proper resource management in concurrent scenarios.

### Platform Manager Architecture

Platform-specific implementations require completion and standardization:
- Missing `get_current_wallpaper` implementations across platforms
- Incorrect lifetime annotations on async trait methods
- Type conversion issues with command arguments
- Platform-specific API handling differences

**Design Decision:** Implement all missing trait methods with appropriate placeholder implementations and fix type conversions to ensure cross-platform compatibility as required by Requirement 4 and 8.

## Components and Interfaces

### 1. Error System Updates

**Component:** `src/core/error.rs`
- **Interface:** No changes to enum structure needed
- **Implementation:** Error variants already exist with correct names

**Component:** Error usage throughout codebase
- **Interface:** Update all `AppError::Platform` to `AppError::PlatformError`
- **Interface:** Update all `AppError::Wallpaper` to `AppError::WallpaperError`

### 2. Async Wallpaper System

**Component:** `src/wallpapers/mod.rs` - Wallpaper trait
- **Interface:** Convert all trait methods to async
- **Implementation:** Add `async` keyword and `AppResult<()>` return types

**Component:** All wallpaper implementations
- **Interface:** Update method signatures to async
- **Implementation:** Add `.await` calls to wallpaper manager methods
- **Implementation:** Use `async_trait` annotation

### 3. Platform Manager Completions

**Component:** `src/platform/windows/mod.rs`
- **Interface:** Add missing `get_current_wallpaper` method
- **Implementation:** Return `Ok(None)` as placeholder

**Component:** `src/platform/linux/mod.rs`
- **Interface:** Fix lifetime annotations on async methods
- **Interface:** Fix return type of `get_current_wallpaper` to `PathBuf`
- **Implementation:** Add proper `async_trait` usage

**Component:** `src/platform/hyprland.rs`
- **Interface:** Add missing `get_current_wallpaper` method
- **Implementation:** Return `Ok(None)` as placeholder

### 4. UI System Updates

**Component:** `src/ui/app.rs`
- **Interface:** Replace deprecated egui methods
- **Implementation:** Remove `.text_color()` method calls
- **Implementation:** Add missing fields to `WidgetConfig` initialization

**Component:** `src/main.rs`
- **Interface:** Update `NativeOptions` field names
- **Implementation:** Replace `initial_window_size` with `viewport.inner_size`

### 5. Type and Import Management

**Component:** Widget system
- **Interface:** Add missing trait imports (`chrono::Datelike`)
- **Implementation:** Fix temporary value lifetimes
- **Implementation:** Resolve borrowing conflicts
- **Implementation:** Remove unused imports to satisfy Requirement 6

**Component:** Command execution
- **Interface:** Fix `Cow<str>` to `OsStr` conversion
- **Implementation:** Use `.to_string()` before passing to command args
- **Implementation:** Ensure proper type bounds for generic parameters

### 6. Memory Management and Borrowing

**Component:** Mutex handling in async contexts
- **Interface:** Replace `std::sync::Mutex` with `tokio::sync::Mutex` where needed
- **Implementation:** Use separate scopes for mutex guards to avoid borrow conflicts
- **Implementation:** Clone values when necessary to prevent lifetime issues

**Component:** Temporary value management
- **Interface:** Ensure all temporary values have appropriate lifetimes
- **Implementation:** Use explicit variable bindings for complex expressions
- **Implementation:** Apply proper Clone implementations where needed

## Data Models

### Async Wallpaper Trait
```rust
#[async_trait]
pub trait Wallpaper {
    fn get_type(&self) -> WallpaperType;
    fn get_path(&self) -> Option<&Path>;
    async fn start(&self) -> AppResult<()>;
    async fn stop(&self) -> AppResult<()>;
    async fn pause(&self) -> AppResult<()>;
    async fn resume(&self) -> AppResult<()>;
}
```

### Updated Widget Config
```rust
pub struct WidgetConfig {
    // existing fields...
    pub background_color: egui::Color32,
    pub opacity: f32,
}
```

### Platform Manager Trait (Complete)
```rust
#[async_trait]
pub trait WallpaperManager: Send + Sync {
    // existing methods...
    async fn get_current_wallpaper(&self) -> AppResult<Option<std::path::PathBuf>>;
}
```

## Error Handling

### Error Variant Mapping
- All `AppError::Platform(...)` → `AppError::PlatformError(...)`
- All `AppError::Wallpaper(...)` → `AppError::WallpaperError(...)`

### Async Error Propagation
- Wallpaper methods will properly propagate async errors with `.await?`
- Platform manager errors will be handled consistently

### Borrowing Conflict Resolution
- Use separate scopes for mutex guards
- Clone values when needed to avoid borrow conflicts
- Use proper lifetime annotations for temporary values

## Testing Strategy

### Compilation Testing
1. **Unit Test:** Verify each module compiles independently
2. **Integration Test:** Verify full project builds without errors
3. **Warning Test:** Ensure warning count is reduced to acceptable levels

### Functionality Testing
1. **Async Test:** Verify wallpaper operations work with async/await
2. **Platform Test:** Verify platform managers implement all required methods
3. **UI Test:** Verify UI renders without egui compatibility issues

### Error Handling Testing
1. **Error Variant Test:** Verify correct error types are used
2. **Error Propagation Test:** Verify async errors propagate correctly
3. **Platform Error Test:** Verify platform-specific errors are handled

## Implementation Phases

### Phase 1: Core Infrastructure
- Fix error variant names throughout codebase
- Add missing error variants if needed
- Update import statements

### Phase 2: Async System
- Convert Wallpaper trait to async
- Update all wallpaper implementations
- Fix async/await usage patterns

### Phase 3: Platform Completions
- Implement missing trait methods
- Fix lifetime annotations
- Resolve type conversion issues

### Phase 4: UI Compatibility
- Update egui method calls
- Fix NativeOptions usage
- Add missing struct fields

### Phase 5: Final Cleanup
- Resolve borrowing conflicts
- Fix remaining type issues
- Clean up unused imports