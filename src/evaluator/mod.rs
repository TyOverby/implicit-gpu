use std::collections::HashMap;

use ::compiler::*;
use ::nodes::*;
use ::opencl::FieldBuffer;


pub struct Evaluator {
    finished: HashMap<GroupId, FieldBuffer>,
}
