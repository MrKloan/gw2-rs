/// API v2 models.

mod build;
pub use self::build::Build;

mod cats;
pub use self::cats::Cat;

mod colors;
pub use self::colors::{Color, Material};

mod currencies;
pub use self::currencies::Currency;

mod dungeons;
pub use self::dungeons::*;

mod quaggans;
pub use self::quaggans::Quaggan;