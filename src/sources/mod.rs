pub mod clip;
pub mod paths;
pub mod stroke;
pub mod text;
pub mod transform;

pub use clip::*;
pub use paths::*;
pub use stroke::*;
pub use text::*;
pub use transform::*;

/// Source of vertex points
pub trait VertexSource {
  /// Rewind the vertex source (unused)
  fn rewind(&self) {}
  /// Get values from the source
  ///
  /// This could be turned into an iterator
  fn xconvert(&self) -> Vec<Vertex<f64>>;
}
