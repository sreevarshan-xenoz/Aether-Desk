# Design Document

## Overview

This design addresses the systematic resolution of 61 compilation errors in the Aether-Desk project. The errors fall into several categories: async/await handling issues, missing error variants, incomplete trait implementations, egui API compatibility problems, and various import/type mismatches. The solution involves updating error handling, fixing async patterns, completing trait implementations, updating UI code for egui compatibility, and resolving borrowing conflicts.

## Architecture

### Error Handling Architecture

The current `AppError` enum in `src/core/error.rs` already contains the necessary variants (`PlatformError`, `WallpaperError`) but the code is referencing them with incorrect names (`Platform`, `Wallpaper`). The fix involves updating all error usage to match the existing enum variants.

### Async/Await Architecture

The wallpaper trait system has a fundamental async/sync mismatch:
- `WallpaperManager` trait methods are async
- `Wallpaper` trait methods are sync but try to call async methods
- Solution: Convert `Wallpaper` trait methods to async and update all implementations

### Platform Manager Architecture

The platform-specific implementations have several issues:
- Missing `get_current_wallpaper` implementations
- Incorrect lifetime annotations on async trait methods
- Type conversion issues with command arguments

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

### 5. Type and Import Fixes

**Component:** Widget system
- **Interface:** Add missing trait imports (`chrono::Datelike`)
- **Implementation:** Fix temporary value lifetimes
- **Implementation:** Resolve borrowing conflicts

**Component:** Command execution
- **Interface:** Fix `Cow<str>` to `OsStr` conversion
- **Implementation:** Use `.to_string()` before passing to command args

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