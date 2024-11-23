use crate::Program;
use chia::clvm_traits::ToClvm;
use clvmr::Allocator;
use clvmr::NodePtr;

pub struct ClvmValueArray {
    values: Vec<ClvmValue>,
}

#[derive(Default)]
pub struct ClvmArrayBuilder {
    values: Vec<ClvmValue>,
}

impl ClvmArrayBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_value(&mut self, value: Box<ClvmValue>) -> &mut Self {
        self.values.push(*value);
        self
    }

    pub fn build(self) -> ClvmValue {
        ClvmValue::Array(ClvmValueArray {
            values: self.values,
        })
    }
}

pub fn build_from_array(array: Box<ClvmArrayBuilder>) -> Box<ClvmValue> {
    Box::new(ClvmValue::Array(ClvmValueArray {
        values: (*array).values,
    }))
}

pub enum ClvmValue {
    Float(f64),
    Integer(u64),
    String(String),
    Bool(bool),
    Program(Program),
    Bytes(Vec<u8>),
    Array(ClvmValueArray), // Use the wrapper here
}

pub(crate) trait Allocate {
    fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String>;
}

impl Allocate for ClvmValue {
    fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
        match self {
            ClvmValue::Float(f) => f.allocate(allocator),
            ClvmValue::Integer(i) => i.allocate(allocator),
            ClvmValue::String(s) => s.allocate(allocator),
            ClvmValue::Bool(b) => b.allocate(allocator),
            ClvmValue::Bytes(b) => b.allocate(allocator),
            ClvmValue::Array(arr) => arr.allocate(allocator),
            ClvmValue::Program(prog) => prog.allocate(allocator),
        }
    }
}

pub fn new_string_value(value: String) -> Box<ClvmValue> {
    Box::new(ClvmValue::String(value))
}

pub fn new_float_value(value: f64) -> Box<ClvmValue> {
    Box::new(ClvmValue::Float(value))
}

pub fn new_int_value(value: u64) -> Box<ClvmValue> {
    Box::new(ClvmValue::Integer(value))
}

pub fn new_bool_value(value: bool) -> Box<ClvmValue> {
    Box::new(ClvmValue::Bool(value))
}

pub fn new_bytes_value(value: Vec<u8>) -> Box<ClvmValue> {
    Box::new(ClvmValue::Bytes(value)) // No need to clone, we own the Vec
}

pub fn new_program_value(program: Box<Program>) -> Box<ClvmValue> {
    Box::new(ClvmValue::Program(*program))
}

pub fn new_array_value(array: Box<ClvmValueArray>) -> Box<ClvmValue> {
    Box::new(ClvmValue::Array(*array))
}

pub fn array_builder() -> Box<ClvmArrayBuilder> {
    Box::new(ClvmArrayBuilder::new())
}

impl Allocate for f64 {
    fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
        if self.is_infinite() {
            return Err("Value is infinite".to_string());
        }

        if self.is_nan() {
            return Err("Value is NaN".to_string());
        }

        if self.fract() != 0.0 {
            return Err("Value has a fractional part".to_string());
        }

        if *self > 9_007_199_254_740_991.0 {
            return Err("Value is larger than MAX_SAFE_INTEGER".to_string());
        }

        if *self < -9_007_199_254_740_991.0 {
            return Err("Value is smaller than MIN_SAFE_INTEGER".to_string());
        }

        let value = *self as i64;

        if (0..=67_108_863).contains(&value) {
            allocator
                .new_small_number(value as u32)
                .map_err(|e| e.to_string())
        } else {
            allocator
                .new_number(value.into())
                .map_err(|e| e.to_string())
        }
    }
}

impl Allocate for u64 {
    fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
        allocator
            .new_number((*self).into())
            .map_err(|e| e.to_string())
    }
}

impl Allocate for String {
    fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
        allocator
            .new_atom(self.as_bytes())
            .map_err(|e| e.to_string())
    }
}

impl Allocate for bool {
    fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
        allocator
            .new_small_number(u32::from(*self))
            .map_err(|e| e.to_string())
    }
}

impl Allocate for Vec<u8> {
    fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
        allocator.new_atom(self).map_err(|e| e.to_string())
    }
}

impl Allocate for ClvmValueArray {
    fn allocate(&self, allocator: &mut Allocator) -> Result<NodePtr, String> {
        let mut items: Vec<NodePtr> = Vec::with_capacity(self.values.len());
        for value in &self.values {
            let node_ptr = value.allocate(allocator)?;
            items.push(node_ptr);
        }
        items.to_clvm(allocator).map_err(|e| e.to_string())
    }
}

impl Allocate for Program {
    fn allocate(&self, _allocator: &mut Allocator) -> Result<NodePtr, String> {
        Ok(self.ptr)
    }
}

pub fn allocate_value(value: &ClvmValue, allocator: &mut Allocator) -> Result<NodePtr, String> {
    value.allocate(allocator)
}
