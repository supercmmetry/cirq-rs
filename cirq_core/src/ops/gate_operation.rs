use crate::ops::raw_types::{Gate, Operation, QId, QIdShape, TaggedOperation};
use crate::utils::extra_traits::Hashable;

#[derive(Clone)]
pub struct GateOperation {
    gate: Box<dyn Gate>,
    qubits: Vec<Box<dyn QId>>,
}

impl GateOperation {
    pub fn new(gate: Box<dyn Gate>, qubits: Vec<Box<dyn QId>>) -> Self {
        Self {
            gate,
            qubits,
        }
    }

    pub fn with_gate(&self, new_gate: Box<dyn Gate>) -> Self {
        Self {
            gate: new_gate,
            qubits: self.qubits.clone()
        }
    }
}

impl QIdShape for GateOperation {
    fn qid_shape(&self) -> Vec<u64> {
        self.gate.qid_shape()
    }
}

impl Operation for GateOperation {
    fn qubits(&self) -> Vec<Box<dyn QId>> {
        self.qubits.clone()
    }

    fn with_qubits(&self, new_qubits: Vec<Box<dyn QId>>) -> Box<dyn Operation> {
        Box::new(Self::new(self.gate.clone(), new_qubits))
    }

    fn tags(&self) -> Vec<Box<dyn Hashable>> {
        vec![]
    }

    fn untagged(&self) -> Box<dyn Operation> {
        Box::new(self.clone())
    }

    fn with_tags(&self, new_tags: Vec<Box<dyn Hashable>>) -> TaggedOperation {
        TaggedOperation::new(Box::new(self.clone()), new_tags)
    }
}