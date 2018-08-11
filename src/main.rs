
mod lina;

fn main() {
    let v = lina::Vector3::new(1.0, 2.0, 3.0);

    println!("{:?}", lina::Matrix3x3::identity() * v);
}
