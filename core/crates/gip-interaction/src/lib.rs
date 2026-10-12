//! Canvas interaction state machine (Slice 0b, task 020b).
//!
//! Owns the gesture state (CAN-MODE-1), the selection and hover (CAN-SEL), and the rules that
//! turn pointer input into document commits. The machine never mutates the document and never
//! touches history: each gesture returns [`Effect`]s, and `gip-session` applies them (one history
//! transaction per committed gesture). A cancelled gesture therefore leaves no trace (CAN-MODE-4).
//!
//! All pointer positions arrive in client pixels and are converted here through `gip-coords`
//! (CAN-COORD-2).

#![forbid(unsafe_code)]

pub mod machine;

pub use machine::{
    Commit, DRAG_THRESHOLD_PX, Effect, HANDLE_SLOP_PX, HIT_TOLERANCE_PX, Handle, Interaction,
    Modifiers, MoveItem, Preview, Rect4, Tool, View, handle_points, resize_box, resize_target,
};
