use super::*;
use compiler::{CompilationContext, Stage};

pub struct Circle {
    pub position: (f32, f32),
    pub radius: f32
}


impl Node for Circle {
    fn compile(&self, cc: &mut CompilationContext) -> (Stage, InputInfo) {
        let res = cc.get_id("circle");
        let dx = cc.get_id("dx");
        let dy = cc.get_id("dy");

        let mut stage = Stage::new(res.clone());

        stage.add_line(format!("float {dx} = {x} - {cx};", dx = dx, x = cc.get_x(), cx = self.position.0));
        stage.add_line(format!("float {dy} = {y} - {cy};", dy = dy, y = cc.get_y(), cy = self.position.1));
        stage.add_line(format!("{result} = sqrt({dx} * {dx} + {dy} * {dy}) - {radius};", result = res, dx = dx, dy = dy, radius = self.radius));

        (stage, InputInfo)
    }
}

unsafe impl Trace for Circle {
    unsafe_empty_trace!();
}

#[test]
fn basic_test() {
    let mut cc = CompilationContext::new();
    let c = Circle { position: (100.0, 200.0), radius: 30.0 };
    let (stage, _) = c.compile(&mut cc);

    assert_eq!(stage.result, "circle_0");
    assert_eq!(stage.lines, vec![
        "float dx_1 = x_s - 100;".to_string(),
        "float dy_2 = y_s - 200;".to_string(),
        "circle_0 = sqrt(dx_1 * dx_1 + dy_2 * dy_2) - 30;".to_string(),
    ]);
}
