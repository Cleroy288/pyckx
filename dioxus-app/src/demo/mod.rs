//! Demo sandbox pages — visual style variants A-H plus a
//! blank UI testing surface. Each variant shares the same
//! layout (see `shared`) and differs only by CSS prefix.

mod shared;

pub mod a;
pub mod b;
pub mod c;
pub mod d;
pub mod e;
pub mod f;
pub mod g;
pub mod h;
pub mod testing_ui;

pub use a::DemoAPage;
pub use b::DemoBPage;
pub use c::DemoCPage;
pub use d::DemoDPage;
pub use e::DemoEPage;
pub use f::DemoFPage;
pub use g::DemoGPage;
pub use h::DemoHPage;
pub use testing_ui::TestingUiPage;
