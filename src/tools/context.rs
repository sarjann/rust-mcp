use std::sync::Mutex;
use crate::tools::tool::ToolMeta;

#[derive(Debug)]
pub struct Context {
    pub tools_meta: Mutex<Vec<ToolMeta>>,
}
