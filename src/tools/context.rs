use crate::tools::tool::{Tool};
use std::collections::HashMap;

pub struct Context{
    pub tools: HashMap<String, Box<dyn Tool>>,
}
