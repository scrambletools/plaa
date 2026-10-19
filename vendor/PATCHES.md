# Vendored dependencies

## smithay-clipboard 0.7.3

prev's copy (from crates.io, MIT, see `smithay-clipboard/LICENSE`), wired
in with `[patch.crates-io]` in the root `Cargo.toml`.

**Why:** winit 0.30 has no file drag and drop on Wayland, and Hyprland
sends drag events only to the first `wl_data_device` a client creates,
which iced's clipboard makes. The patch handles drag and drop on that
device (`src/dnd.rs`, and the drag hooks in `src/state.rs`), reported to
the handler set with `smithay_clipboard::dnd::set_drag_handler`; plaa
takes dropped file lists from it. The patch also shares one worker among
a display's clipboards, stopped by `smithay_clipboard::shutdown` while the
Wayland connection is still open, as iced makes a clipboard per window.
prev's `vendor/PATCHES.md` has the whole story;
`smithay-clipboard-dnd.patch` is the diff against the crates.io release.

**Updating:** this is the same copy prev has. Both could move to
scramble-ui, beside its patched iced crates, so they change together.
