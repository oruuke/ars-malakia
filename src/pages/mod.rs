use crate::view::Page;
pub mod ch0_pg0;
pub mod ch1_pg0;
pub mod ch1_pg1;
pub mod ch0_pg1;
pub mod ch0_pg2;

pub const ALL_PAGES: &[fn(&u16, u16) -> Page<'static>] = &[
    ch0_pg0::create_page,
    ch0_pg1::create_page,
    ch0_pg2::create_page,
    ch1_pg0::create_page,
    ch1_pg1::create_page,
];
