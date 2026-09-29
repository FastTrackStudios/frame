//! Suite: **Djent Forge** (`suite-djent-forge`) — a modern progressive
//! metal amp-sim suite recreated unit by unit as frame surfaces, drawn
//! after the real plugin's in-window screenshots.
//!
//! | file | unit | form |
//! |---|---|---|
//! | `01-pre-standard-compressor` | Skyline Compressor: silver powder coat, blue lettering, GR dot meter | Pedal Standard 156 × 286 |
//! | `02-pre-standard-tape-drift` | Tape Drift echo: sage green, LED time readout, MS/NOTE toggle | Pedal Standard |
//! | `03-pre-standard-twin-octave` | Twin Octave: champagne, OCT−1 / OCT+1 / DIRECT | Pedal Standard |
//! | `04-pre-standard-tight-drive` | Tight Drive: navy sparkle, lit GATE dial | Pedal Standard |
//! | `05-pre-micro-havoc` | Havoc: oxblood micro box, one MIX knob | Pedal Micro 92 × 286 |
//! | `06-amp-head-ridge-clean` | Ridge Clean: black head, blue-lit ridge | Head 1200 × 510 |
//! | `07-amp-head-mech-rhythm` | Mech Rhythm: white machined head, orange-lit core | Head 1200 × 510 |
//! | `08-amp-head-slat-lead` | Slat Lead: black slatted cage over lit valves | Head 1200 × 510 |
//! | `09-cab-2x12` | 2×12 cabinet with two mics | Cab 900 × 900 |
//! | `10-eq-tabletop-nine-band` | Ivory Nine-Band graphic EQ | Tabletop 1000 × 262 |
//! | `11-post-treadle-modulator` | Treadle Modulator: rose-gold rocker + range display | Custom (BigBox-width) 404 × 286 |
//! | `12-post-bigbox-stereo-echo` | Stereo Echo: twin 16-segment readouts, ENGAGE / TAP | Pedal BigBox 404 × 286 |
//! | `13-post-standard-shimmer-verb` | Shimmer Verb: midnight blue | Pedal Standard |
//! | `14-fx-treadle-beam` | Beam pitch treadle with a patch list | Custom (BigBox-width) 404 × 286 |
//! | `15-fx-tabletop-grain-glitch` | Grain Glitch: waveform screen, modes, grain dials | Tabletop 1000 × 600 |
//!
//! Real names are never printed: every unit carries an invented name set in
//! the original's lettering style, and the maker's mark is an invented
//! one ("AZIMUTH circuits").

pub mod amps;
pub mod post;
pub mod pre;
pub mod rack;

use crate::face::Face;

/// The suite's plugin id.
pub const PLUGIN: &str = "suite-djent-forge";

/// The blue the pedals' lettering is printed in.
pub const BLUE: &str = "#1f5fa3";
/// The lit blue of the pedals' status LEDs.
pub const LED_BLUE: &str = "#7cc4ff";

/// Every unit of the suite, in signal order.
#[must_use]
pub fn faces() -> Vec<Face> {
    let mut v = pre::faces();
    v.extend(amps::faces());
    v.extend(rack::faces());
    v.extend(post::faces());
    v
}
