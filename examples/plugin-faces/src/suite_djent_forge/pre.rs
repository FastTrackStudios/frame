//! Djent Forge pre-FX: five pedals in one row, all one height. Four are the
//! Standard 125B class (156 × 286), the last a Micro (92 × 286).
//!
//! What they share, as the plugin draws them: a powder-coated lid, glossy
//! black lobed knobs with a white line, small caps legends, the model name
//! in widely tracked light capitals across the middle, a chrome-rimmed blue
//! LED over a knurled stomp switch in the bottom band, and the maker's
//! mark (invented) or a thin printed rule with a "designed by" line under
//! the switch.

use surface_kit::Place;

use super::{BLUE, LED_BLUE, PLUGIN};
use crate::delay::parts::{self, Ink};
use crate::face::{Face, canvas};
use crate::rigkit::{self as rk, Enclosure, KnobStyle, Letter};

pub fn faces() -> Vec<Face> {
    vec![compressor()]
}

const KNOB: KnobStyle<'static> = KnobStyle::new("knob.pedal_lobed", "black", 46.0, "#ffffff");

/// The maker's mark, as the silver and navy boxes print it: a heavy
/// geometric word in blue over a widely spaced second line.
fn maker_mark(b: &mut surface_widgets::Builder<'_>, parent: document_proto::NodeId, cx: f64, cy: f64, w: f64, color: &str) {
    let (_, size) = rk::letter_fit(b, parent, "AZIMUTH", (cx, cy), w, Letter::new(20.0, 780.0, color).stretch(0.9).track(0.02));
    rk::letter_fit(b, parent, "CIRCUITS", (cx, cy + size * 0.66), w, Letter::new(6.0, 500.0, color).track(1.55));
}

/// The bottom band every pre-FX box shares: the chrome-rimmed blue LED
/// over the knurled stomp switch.
fn stomp_band(b: &mut surface_widgets::Builder<'_>, parent: document_proto::NodeId, w: f64, addr: &str) {
    rk::stomp(b, parent, "Bypass", addr, (w / 2.0, 229.0), 42.0, LED_BLUE, (0.0, -29.0), 12.0, "bezel", false);
}

/// The printed rule with a small notch over the "designed by" line (the
/// green and champagne boxes).
fn rule_and_line(b: &mut surface_widgets::Builder<'_>, parent: document_proto::NodeId, w: f64, y: f64, color: &str) {
    let (x0, x1, cx) = (w * 0.2, w * 0.8, w / 2.0);
    rk::line(b, parent, &[(x0, y), (cx - 6.0, y), (cx - 2.0, y + 3.5), (cx + 2.0, y - 3.5), (cx + 6.0, y), (x1, y)], 0.8, color);
    rk::silk(b, parent, "DESIGNED IN THE FORGE", cx, y + 13.0, Ink::new(5.2, color).weight(500.0).track(0.28));
}

// ── 01 · Skyline Compressor ──────────────────────────────────────────────

/// Skyline Compressor — Pedal, Standard (156 × 286). Silver powder coat,
/// blue lettering; DETAIL, MIX, VOLUME, COMP round an INTENSITY dome
/// button; a row of ten blue LEDs reads gain reduction (−1 … −20 dB, lit
/// from the right).
fn compressor() -> Face {
    let ns = "dfcomp";
    let a = |s: &str| format!("{ns}/{s}");
    let scene = canvas("Skyline Compressor", |b, root| {
        let e = Enclosure::new("standard", "#b8babd", "powder").base("#d5d6d8");
        let p = rk::pedal_body(b, root, &e);
        let (w, _) = e.size();
        let legend = Ink::new(6.4, BLUE).weight(600.0).track(0.02);
        let knob_at = [("detail", "Detail", (31.0, 38.0), 64.0), ("mix", "Mix", (w - 31.0, 38.0), 64.0), ("volume", "Volume", (31.0, 92.0), 118.0), ("comp", "Comp", (w - 31.0, 92.0), 118.0)];
        for (addr, name, at, ly) in knob_at {
            rk::knob(b, p, &KNOB, &a(addr), name, at, ly, legend);
        }
        // The GR meter: a black slot with ten lamps, printed thresholds over it.
        let (mx0, mx1, my) = (47.0, 111.0, 16.5);
        let slot = draw_slot(b, p, (mx0 - 2.0, my - 4.2, mx1 - mx0 + 4.0, 8.4));
        let _ = slot;
        rk::gr_dots(b, p, "Compression", &a("gr"), (mx0, mx1, my), 4.6, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 10.0, 15.0, 20.0], LED_BLUE, true);
        for (i, l) in ["-20", "-10", "-7", "-5", "-3", "-1"].iter().enumerate() {
            rk::silk(b, p, l, mx0 + 3.0 + (mx1 - mx0 - 6.0) * i as f64 / 5.0, 8.8, Ink::new(4.6, BLUE).weight(600.0).track(0.0));
        }
        rk::silk(b, p, "Compression", w / 2.0, 27.5, Ink::new(6.4, BLUE).weight(600.0).track(0.02));
        let dome = Place::new("Intensity").size(21.0).variant("chrome").bind(rk::bind(&a("intensity"))).touch(surface_kit::TOUCH_MIN);
        let dome = parts::put(b, p, "button.dome", &dome, w / 2.0, 62.0);
        parts::label_at(b, dome, "Intensity", (w / 2.0, 83.0), legend);

        rk::letter(b, p, "COMPRESSOR", (w / 2.0, 137.0), Letter::new(13.5, 420.0, BLUE).track(0.16));
        stomp_band(b, p, w, &a("on"));
        maker_mark(b, p, w / 2.0, 258.0, w * 0.74, BLUE);
    });
    let params = vec![
        rk::percent(&a("detail"), "Detail", 50.0),
        rk::percent(&a("mix"), "Mix", 100.0).featured(),
        rk::db(&a("volume"), "Volume", -12.0, 12.0, 0.0).featured(),
        rk::percent(&a("comp"), "Comp", 50.0).featured(),
        rk::toggle(&a("intensity"), "Intensity", false),
        rk::toggle(&a("on"), "Bypass", true).featured(),
    ];
    Face { file: "01-pre-standard-compressor", plugin: PLUGIN, scene, params, streams: vec![rk::gr_stream(&a("gr"), "Compression")] }
}

/// A recessed black slot (the GR meter's window).
fn draw_slot(b: &mut surface_widgets::Builder<'_>, parent: document_proto::NodeId, r: (f64, f64, f64, f64)) -> document_proto::NodeId {
    let fill = surface_kit::draw::linear(180.0, &[(0.0, "#050506"), (1.0, "#1c1d20")]);
    parts::well(b, parent, "meter slot", r, fill, r.3 / 2.0)
}
