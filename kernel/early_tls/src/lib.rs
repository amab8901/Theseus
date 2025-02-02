//! Handles initialization of TLS data images during early OS initialization.
//!
//! This is only useful before the tasking subsystem is initialized, i.e.,
//! in the `nano_core`, `captain`, or `ap_start` crates.

#![no_std]

use tls_initializer::TlsDataImage;
use spin::Mutex;

static EARLY_TLS_IMAGE: Mutex<TlsDataImage> = Mutex::new(TlsDataImage::empty());

/// Insert the current early TLS image with the given `new_tls_image`,
/// and loads the new image on this CPU.
///
/// If an early TLS image already exists, it is removed and dropped.
pub fn insert(new_tls_image: TlsDataImage) {
    new_tls_image.set_as_current_tls_base();
    *EARLY_TLS_IMAGE.lock() = new_tls_image;
}

/// Loads the existing (previously-initialized) early TLS image on this CPU.
pub fn reload() {
    EARLY_TLS_IMAGE.lock().set_as_current_tls_base();
}

/// Clears the early TLS image
///
/// This should only be invoked after the task subsystem is initialized on all CPUs.
pub fn drop() {
    *EARLY_TLS_IMAGE.lock() = TlsDataImage::empty();
}
