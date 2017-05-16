use itertools::Itertools;
use opencl::LineBuffer;

pub fn dump_poly_lines(file: &str, xs: &LineBuffer, ys: &LineBuffer) {
    let lines = xs.values().into_iter().zip(ys.values().into_iter());
    let lines = lines
        .tuples()
        .filter(|&((x1, y1), (x2, y2))| !(x1.is_nan() || x2.is_nan() || y1.is_nan() || y2.is_nan()))
        .map(|((x1, y1), (x2, y2))| format!("{}, {}, {}, {}", x1, y1, x2, y2));
    ::latin::file::write_lines(file, lines).unwrap();
}
