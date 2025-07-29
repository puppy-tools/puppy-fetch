use crate::context::Context;

pub enum ContextFlow { 
    Initialized(Context), 
    HaltOk, 
    HaltErr(Box<dyn std::error::Error>), 
}