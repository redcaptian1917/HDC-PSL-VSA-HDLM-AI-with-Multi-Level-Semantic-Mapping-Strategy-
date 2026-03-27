pub mod vector;
pub mod error;
pub mod compute;
pub mod hdlm;
pub mod adaptive;

pub use vector::{BipolarVector, HD_DIMENSIONS};
pub use error::HdcError;
pub use compute::{ComputeBackend, LocalBackend};
pub use hdlm::{ForensicNode, SemanticMap};
pub use adaptive::{UiAttributes, UiElement};
