pub mod a {
    pub fn a() {}
}

pub mod prelude {
    pub use crate::a::a;
    pub fn b() {}
}
