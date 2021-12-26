mod line;
use line::*;

mod mesh;
use mesh::*;

use crystal::Crystal;
use lattice::Lattice;
use vector3::Vector3f64;

pub trait KPTS {
    fn get_k_frac(&self, k_index: usize) -> Vector3f64;
    fn get_k_degeneracy(&self, k_index: usize) -> usize;
    fn get_k_weight(&self, k_index: usize) -> f64;
    fn get_n_kpts(&self) -> usize;
    fn frac_to_cart(&self, k_frac: &Vector3f64, blatt: &Lattice) -> Vector3f64;
    fn get_k_mesh(&self) -> [i32; 3];
    fn display(&self);
}

pub fn new(scheme: &str, crystal: &Crystal, _symmetry: bool) -> Box<dyn KPTS> {
    let kpts: Box<dyn KPTS>;

    match scheme {
        "kmesh" => {
            kpts = Box::new(KptsMesh::new(crystal));
        }

        "kline" => {
            kpts = Box::new(KptsLine::new());
        }

        &_ => {
            println!("{} not implemented", scheme);
            std::process::exit(1);
        }
    }

    kpts
}
