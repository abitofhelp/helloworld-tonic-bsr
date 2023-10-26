mod gen;

pub mod helloworld {
    pub mod v1 {
        include!("gen/helloworld.v1.rs");
    }
}