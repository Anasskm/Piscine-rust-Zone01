mod areas_volumes;

pub use areas_volumes::*;

pub fn area_fit(
    x: usize,
    y: usize,
    objects: GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let area = rectangle_area(x, y);
    match objects {
        GeometricalShapes::Square => {
            if square_area(a) * times > area {
                return false;
            }
            true
        }
        GeometricalShapes::Circle => {
            if circle_area(a) * times as f64 > area as f64 {
                return false;
            }
            true
        }
        GeometricalShapes::Rectangle => {
            if rectangle_area(a, b) * times > area {
                return false;
            }
            true
        }
        GeometricalShapes::Triangle => {
            if triangle_area(a, b) * times as f64 > area as f64 {
                return false;
            }
            true
        }
    }
}
pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let box_v = parallelepiped_volume(x, y, z);
    match objects {
        GeometricalVolumes::Cube => {
            if cube_volume(a) * times > box_v {
                return false;
            }
            true
        }
        GeometricalVolumes::Sphere => {
            if sphere_volume(a) * times as f64 > box_v as f64 {
                return false;
            }
            true
        }
        GeometricalVolumes::Cone => {
            if cone_volume(a, b) * times as f64 > box_v as f64 {
                return false;
            }
            true
        }
        GeometricalVolumes::Pyramid => {
            if cone_volume(a, b) * times as f64> box_v as f64 {
                return false;
            }
            true
        }
        GeometricalVolumes::Parallelepiped => {
            if parallelepiped_volume(a, b, c) * times > box_v {
                return false;
            }
            true
        }
    }
}
