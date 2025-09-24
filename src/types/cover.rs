//! Types and traits for "cover" values used by the rasterizer.
//!
//! Background / intent
//! -------------------
//! The AA rasterizer accumulates per-cell signed integer quantities called
//! `cover` and `area` at subpixel resolution. These values are used to
//! compute the final alpha (coverage) for a pixel. The accumulation must be
//! signed because edges can contribute positive or negative amounts (winding
//! rules, holes, etc.). For safety and to avoid intermediate overflows we use
//! wide integer types in the rasterization code; the crate-level constants
//! `POLY_SUBPIXEL_SHIFT` and `POLY_SUBPIXEL_SCALE` control the subpixel grid
//! (the repository defaults are `POLY_SUBPIXEL_SHIFT = 8`, `POLY_SUBPIXEL_SCALE = 256`).
//!
//! What `cover` and `area` mean
//! -----------------------------
//! - `cover` is the signed sum of vertical edge contributions for a logical
//!   cell (pixel) at subpixel resolution.
//! - `area` is a signed subpixel-area accumulator that represents partial
//!   coverage inside that cell with S*S resolution (S = `POLY_SUBPIXEL_SCALE`).
//!
//! Both are kept as integers during rasterization so all geometry is exact
//! in the chosen subpixel grid.
//!
//! How the final 8-bit alpha is computed
//! -------------------------------------
//! The scanline code in this crate converts the pair `(cover, area)` into an
//! 8-bit alpha value with (roughly) the following steps (the exact code is
//! in `rasters::scanline::calculate_alpha` and the call-site in
//! `rasters::scanline::sweep_scanline`):
//!
//! 1. form an intermediate area parameter:
//!
//!    ```text
//!    area_param = (cover << (POLY_SUBPIXEL_SHIFT + 1)) - area
//!    ```
//!
//! 2. normalize to AA precision (the crate uses `aa_shift = 8`):
//!
//!    ```text
//!    coverage_index = abs(area_param) >> (POLY_SUBPIXEL_SHIFT*2 + 1 - aa_shift)
//!    ```
//!
//!    With the default `POLY_SUBPIXEL_SHIFT = aa_shift = 8` both shifts are 9,
//!    so the operation simplifies to `coverage_index = abs(area_param) >> 9`.
//!
//! 3. optionally apply the Even-Odd filling rule adjustment, clamp to
//!    [0..255], and map through the `gamma` table (identity by default). The
//!    resulting value is the 8-bit alpha that is written into the scanline.
//!
//! Relationship to `delta`
//! ------------------------
//! In the rasterization routines (see `rasters::cell`), the variable named
//! `delta` is a temporary signed increment computed while walking an edge
//! through cells. `delta` describes how much a particular fragment of an
//! edge contributes to the `cover` (and relatedly to `area`) of the current
//! cell. The implementation repeatedly does e.g.:
//!
//! ```ignore
//! m_curr_cell.cover += delta;
//! m_curr_cell.area  += some_term * delta;
//! ```
//!
//! So `cover` is the accumulation of many such `delta` contributions produced
//! by geometry. The `area` accumulator stores the subpixel area-weighted
//! contributions needed for precise coverage computation.
//!
//! Example (conceptual)
//! ---------------------
//! Given a `Cell { cover, area }` you can compute the final byte alpha as:
//!
//! ```rust
//! # const POLY_SUBPIXEL_SHIFT: i64 = 8;
//! # let cover = 0; let area = 0i64;
//! // conceptual, mirrors scanline::calculate_alpha behaviour
//! let area_param = (cover << (POLY_SUBPIXEL_SHIFT + 1)) - area;
//! let mut coverage = (area_param >> (POLY_SUBPIXEL_SHIFT * 2 + 1 - 8)).abs();
//! coverage = coverage.clamp(0, 255);
//! // gamma[coverage] is applied in the real code; by default it is identity
//! let alpha_u8 = coverage as u8;
//! ```
//!
//! Notes
//! -----
//! - The choice of `i64` for the runtime `cover`/`area` fields is deliberate
//!   to avoid overflow during intermediate arithmetic while keeping the logic
//!   straightforward. Extremely large coordinates can still overflow, but
//!   typical rendering sizes are safe.
//! - The gamma table in `RasterizerScanline` can remap the linear coverage to
//!   any transfer curve; by default it's the identity mapping 0..255.
use crate::RealLike;

trait CoverLike: RealLike {
  /// Create from a u64 value.
  fn from_u64(v: u64) -> Self;
  fn to_u64(self) -> u64;

  fn is_full(self) -> bool;
}
