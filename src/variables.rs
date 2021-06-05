use std::collections::HashMap;

pub(crate) struct VariableContainer {
    data: HashMap<String, Vec<u8>>,
}

impl VariableContainer {
    pub(crate) fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    pub(crate) fn set(&mut self, var: &str, val: u8) {
        if self.data.contains_key(var) {
            self.data.get_mut(var).unwrap().push(val);
        } else {
            self.data.insert(var.to_string(), vec![val]);
        }
    }
    pub(crate) fn get(&mut self, var: &str) -> u8 {
        self.data
            .get_mut(var)
            .unwrap_or(&mut vec![])
            .pop()
            .unwrap_or_default()
    }
}
