mod math;

#[no_mangle]
pub fn add_vectors( f1 : f32, f2 : f32, f3: f32) -> f32{
    use math::vector::Vector3;
    let v1 = Vector3 { x: f1, y : -f2, z : f3 + f1};
    let v2 = Vector3 { x: f2, y : 2.0 + f1, z : f1 * f2};
    (v2.cross_product(&v1) + v1).length()
}
