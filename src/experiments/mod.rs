// Experimental features for Aether-Desk
// These features are not guaranteed to be stable or production-ready

pub mod widgets;
pub mod ai;
pub mod effects;
pub mod performance;
pub mod wallpapers;
pub mod services;

/// Enable all experimental features
pub fn enable_all_experimental_features() {
    widgets::enable();
    ai::enable();
    effects::enable();
    performance::enable();
    wallpapers::enable();
    services::enable();
}

/// Disable all experimental features
pub fn disable_all_experimental_features() {
    widgets::disable();
    ai::disable();
    effects::disable();
    performance::disable();
    wallpapers::disable();
    services::disable();
} 