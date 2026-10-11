//! Property suites for the coordinate contract (coordinate-system.md §9).
//!
//! Deterministic generation: a built-in splitmix64 PRNG with fixed seeds. Rationale
//! (task summary 009): dev-dependencies are subject to the same registry + zero-dependency
//! law as runtime dependencies; a 20-line generator gives reproducible failures and keeps
//! the crate dependency-free. If a future failure needs input shrinking, that is a
//! tooling decision to revisit — not a reason to pre-load a framework.
//!
//! Magnitudes are chosen per the doc laws: world coordinates up to 1e5, zoom across the
//! full 2%–4096% range, DPR 1/1.5/2/3, fractional and negative values throughout.

use gip_coords::viewport::{
    ViewportRect, device_to_viewport, screen_to_viewport, screen_to_world, viewport_to_device,
    viewport_to_screen, world_to_screen,
};
use gip_coords::{
    Camera, EPS_ANGLE, EPS_MATRIX, EPS_ROUNDTRIP, MAX_ZOOM, MIN_ZOOM, Mat2, Trs, Vec2,
    normalize_deg,
};

/// Deterministic splitmix64 — seedable, reproducible, no dependency.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.unit()
    }

    fn signed(&mut self, magnitude: f64) -> f64 {
        self.range(-magnitude, magnitude)
    }

    fn point(&mut self, magnitude: f64) -> Vec2 {
        Vec2::new(self.signed(magnitude), self.signed(magnitude))
    }

    fn boolean(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    fn camera(&mut self) -> Camera {
        Camera::new(self.signed(1e5), self.signed(1e5), self.zoom())
    }

    fn zoom(&mut self) -> f64 {
        // Full legal range, biased toward extremes so clamps/limits are exercised.
        match self.next_u64() % 4 {
            0 => MIN_ZOOM,
            1 => MAX_ZOOM,
            _ => self.range(MIN_ZOOM, MAX_ZOOM),
        }
    }

    fn rect(&mut self) -> ViewportRect {
        ViewportRect::new(
            self.range(0.0, 800.0),
            self.range(0.0, 400.0),
            self.range(100.0, 2000.0),
            self.range(100.0, 1200.0),
        )
    }

    fn trs(&mut self, magnitude: f64) -> Trs {
        let t = Trs::new(
            self.signed(magnitude),
            self.signed(magnitude),
            self.range(1e-3, magnitude),
            self.range(1e-3, magnitude),
            self.signed(1080.0),
            self.boolean(),
            self.boolean(),
        );
        t.normalized()
    }
}

const CASES: usize = 2_000;

#[test]
fn screen_viewport_roundtrips_across_offsets_and_rects() {
    let mut rng = Rng::new(0x5EED_0000);
    for _ in 0..CASES {
        let rect = rng.rect();
        let p = rng.point(5_000.0);
        let back = viewport_to_screen(screen_to_viewport(p, rect), rect);
        assert!(
            back.approx_eq(p, EPS_ROUNDTRIP),
            "screen↔viewport drifted: {back:?}"
        );
    }
}

#[test]
fn world_viewport_roundtrips_across_zoom_rotation_and_zero_size_edges() {
    let mut rng = Rng::new(0x5EED_0001);
    for _ in 0..CASES {
        let cam = rng.camera();
        let world = rng.point(1e5);
        let back = cam.viewport_to_world(cam.world_to_viewport(world));
        // CAN-COORD invariant: within EPS_ROUNDTRIP world units across zoom 2%–4096%.
        assert!(
            back.approx_eq(world, EPS_ROUNDTRIP),
            "world↔viewport drifted: {back:?}"
        );
        // Zero-size node leg: local→world via a degenerate matrix stays finite.
        let degenerate = Trs::new(1.0, 2.0, 0.0, 5.0, 30.0, false, false).to_matrix();
        assert!(degenerate.local_to_world(world).is_finite());
        assert!(
            degenerate.world_to_local(world).is_none(),
            "singular must not invert"
        );
    }
}

#[test]
fn composite_screen_world_roundtrips() {
    let mut rng = Rng::new(0x5EED_0002);
    for _ in 0..CASES {
        let rect = rng.rect();
        let cam = rng.camera();
        let world = rng.point(1e5);
        let back = screen_to_world(world_to_screen(world, rect, cam), rect, cam);
        assert!(
            back.approx_eq(world, EPS_ROUNDTRIP),
            "screen↔world drifted: {back:?}"
        );
    }
}

#[test]
fn device_roundtrips_at_fractional_dpr() {
    let mut rng = Rng::new(0x5EED_0003);
    for _ in 0..CASES {
        let dpr = [1.0, 1.5, 2.0, 3.0][(rng.next_u64() % 4) as usize];
        let vp = rng.point(2_000.0);
        let back = device_to_viewport(viewport_to_device(vp, dpr), dpr);
        assert!(
            back.approx_eq(vp, EPS_ROUNDTRIP),
            "device conversion drifted at {dpr}"
        );
    }
}

#[test]
fn pointer_zoom_invariance_holds_through_gesture_sequences() {
    // Law (CAN-CAM-2): through any sequence of zoom gestures AT A CURSOR, the world point
    // under that cursor stays under it (≤ 0.5 screen px) — including at the clamp limits,
    // because the camera position is derived from the CLAMPED zoom. Panning is a different
    // gesture: it deliberately moves everything on screen (its law is tested below).
    let mut rng = Rng::new(0x5EED_0004);
    for _ in 0..200 {
        let mut cam = rng.camera();
        let cursor = rng.point(1_500.0);
        let anchor = cam.viewport_to_world(cursor);
        for _ in 0..12 {
            cam.zoom_around_viewport(cursor, rng.range(0.5, 2.0));
        }
        let screen_again = cam.world_to_viewport(anchor);
        assert!(
            screen_again.approx_eq(cursor, 0.5),
            "anchor drifted beyond 0.5px: {screen_again:?}"
        );
        assert!(cam.is_valid());

        // Pan law: a viewport-space pan shifts the screen position of any world point by
        // exactly the delta, at any zoom (screen-constant panning).
        let before = cam.world_to_viewport(anchor);
        let delta = rng.point(300.0);
        cam.pan_by_viewport(delta);
        let after = cam.world_to_viewport(anchor);
        assert!(
            after.approx_eq(before + delta, 1e-6),
            "pan moved world point by {:?} instead of {:?}",
            after - before,
            delta
        );
    }
}

#[test]
fn zoom_clamps_never_exit_range_or_produce_non_finite_cameras() {
    let mut rng = Rng::new(0x5EED_0005);
    for _ in 0..CASES {
        let mut cam = rng.camera();
        let cursor = rng.point(1_000.0);
        let factor = match rng.next_u64() % 3 {
            0 => 1e6,
            1 => 1e-6,
            _ => rng.range(0.1, 10.0),
        };
        cam.zoom_around_viewport(cursor, factor);
        assert!(
            (MIN_ZOOM..=MAX_ZOOM).contains(&cam.zoom),
            "zoom escaped clamp: {}",
            cam.zoom
        );
        assert!(cam.is_valid());
    }
}

#[test]
fn nested_ancestor_chains_roundtrip_through_local_world() {
    let mut rng = Rng::new(0x5EED_0006);
    for _ in 0..CASES / 4 {
        // Accumulate a root→leaf chain (CAN-COORD-3): world = parentWorld * local.
        let depth = rng.next_u64() % 5;
        let mut world = Mat2::IDENTITY;
        for _ in 0..depth {
            world = world * rng.trs(1e4).to_matrix();
        }
        let p = rng.point(1e3);
        let world_p = world.local_to_world(p);
        let back = world.world_to_local(world_p);
        match back {
            Some(local_p) => assert!(
                local_p.approx_eq(p, 1e-6),
                "chain roundtrip drifted (depth {depth}): {local_p:?}"
            ),
            None => panic!("constructed chain must be invertible (sizes ≥ 1e-3)"),
        }
    }
}

#[test]
fn decompose_recompose_roundtrips_matrix_and_canonical_fields() {
    let mut rng = Rng::new(0x5EED_0007);
    for _ in 0..CASES {
        let t = rng.trs(1e5);
        let m = t.to_matrix();
        let d = m.decompose().expect("non-degenerate TRS must decompose");
        // (1) matrix identity: decompose→recompose returns the same matrix.
        assert!(
            d.to_matrix().approx_eq(m, EPS_MATRIX),
            "matrix roundtrip drifted for {t:?}"
        );
        // (2) canonical field identity: decomposition returns the normalized frame.
        let canon = t.normalized();
        assert!(
            d.approx_eq(canon, EPS_ROUNDTRIP, EPS_ANGLE),
            "field roundtrip drifted:\n got {d:?}\n want {canon:?}"
        );
    }
}

#[test]
fn matrix_cycles_are_stable_over_a_thousand_iterations() {
    // CAN-TR-6 stability law: 1000 apply→revert cycles stay within EPS_MATRIX of the
    // original. Each cycle must apply a delta and its inverse (an accumulating product of
    // 1000 random scales diverges exponentially — that is not a cycle and would overflow).
    let mut rng = Rng::new(0x5EED_0008);
    let start = rng.trs(1e3).to_matrix();
    let mut current = start;
    for _ in 0..1000 {
        let delta = rng.trs(10.0).to_matrix();
        let inv = delta.invert().expect("deltas are non-degenerate");
        current = current * delta * inv;
    }
    assert!(
        current.is_finite(),
        "cycle product must stay finite (deltas are size-bounded)"
    );
    assert!(
        current.approx_eq(start, EPS_MATRIX),
        "1000-cycle drift exceeded EPS_MATRIX:\n{current:?}\nvs\n{start:?}"
    );
}

#[test]
fn rotation_application_matches_trigonometry_across_the_full_range() {
    let mut rng = Rng::new(0x5EED_0009);
    for _ in 0..CASES {
        let angle = rng.signed(720.0);
        let (sin, cos) = angle.to_radians().sin_cos();
        let v = Vec2::new(cos, sin);
        let rotated = Mat2::rotation_deg(angle).apply(Vec2::new(1.0, 0.0));
        assert!(
            rotated.approx_eq(v, 1e-12),
            "rotation({angle}) misplaced the x axis"
        );
        // Normalization does not change where the rotation lands.
        let normalized = normalize_deg(angle);
        let rotated_n = Mat2::rotation_deg(normalized).apply(Vec2::new(1.0, 0.0));
        assert!(rotated_n.approx_eq(v, 1e-12));
    }
}

#[test]
fn angle_normalization_is_total_and_idempotent() {
    let mut rng = Rng::new(0x5EED_000A);
    for _ in 0..CASES {
        let raw = rng.signed(1e9);
        let n = normalize_deg(raw);
        assert!(n > -180.0 && n <= 180.0, "range violated: {raw} → {n}");
        assert_eq!(
            normalize_deg(n).to_bits(),
            n.to_bits(),
            "not idempotent at {raw}"
        );
    }
}

#[test]
fn angle_normalization_preserves_rotation_for_representable_inputs() {
    // Equivalence of `rotation(raw)` and `rotation(normalize(raw))` holds where the
    // degrees→radians conversion itself is precise: at |deg| ≤ 1e6 the radian ulp is
    // ≈ 2e-12. Beyond that the radian ulp exceeds the round-trip epsilon — which is exactly
    // why normalization happens at input edges, before any trig (ADR-002 §5).
    let mut rng = Rng::new(0x5EED_000B);
    for _ in 0..CASES {
        let raw = rng.signed(1e6);
        let a = Mat2::rotation_deg(raw).apply(Vec2::new(1.0, 0.0));
        let b = Mat2::rotation_deg(normalize_deg(raw)).apply(Vec2::new(1.0, 0.0));
        assert!(
            a.approx_eq(b, 1e-9),
            "normalization changed the rotation at {raw}"
        );
    }
}
