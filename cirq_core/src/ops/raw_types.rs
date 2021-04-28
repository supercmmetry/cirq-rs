use std::cmp::Ordering;
use std::hash::Hash;

use anyhow::Error;

use crate::utils::extra_traits::Hashable;

/// Identifies a quantum object such as a qubit, qudit, resonator, etc.
pub trait QId {
    fn comparison_key(&self) -> String;
    /**
     * Returns the dimension or the number of quantum levels this qid has.
     * E.g. 2 for a qubit, 3 for a qutrit, etc.
     */
    fn dimension(&self) -> u64;

    /// Validates dimension
    fn validate_dimension(&self, dimension: u64) -> Result<(), anyhow::Error>;
}

#[derive(Clone)]
struct QubitAsQId<'a> {
    comparison_key: String,
    pub qubit: &'a dyn QId,
    pub dimension: u64,
}

impl PartialEq for QubitAsQId<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.comparison_key == other.comparison_key
    }
}

impl Eq for QubitAsQId<'_> {}

impl PartialOrd for QubitAsQId<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.comparison_key.partial_cmp(&other.comparison_key)
    }
}

impl Ord for QubitAsQId<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.comparison_key.cmp(&other.comparison_key)
    }
}

impl QId for QubitAsQId<'_> {
    fn comparison_key(&self) -> String {
        self.comparison_key.clone()
    }

    fn dimension(&self) -> u64 {
        self.dimension.clone()
    }

    fn validate_dimension(&self, dimension: u64) -> Result<(), Error> {
        if dimension < 1 {
            Err(anyhow!(format!("Wrong qid dimension. Expected a positive integer but got {}.", dimension)))
        } else {
            Ok(())
        }
    }
}

impl<'a> QubitAsQId<'a> {
    pub fn new<T: QId>(qubit: &'a T, dimension: u64) -> Result<Self, anyhow::Error> {
        let q = Self {
            comparison_key: "".to_string(),
            qubit,
            dimension,
        };

        q.validate_dimension(dimension)?;
        Ok(q)
    }

    /// Returns a new QubitAsQId with a different dimension.
    fn with_dimension(&'a self, dimension: u64) -> Result<Self, anyhow::Error> {
        if self.dimension == dimension {
            Ok(self.clone())
        } else {
            QubitAsQId::new(self, dimension)
        }
    }
}

pub trait QIdShape {
    fn qid_shape(&self) -> Vec<u64>;
}

/**
 *   An operation type that can be applied to a collection of qubits.
 *   Gates can be applied to qubits by calling their on() method with
 *   the qubits to be applied to supplied, or, alternatively, by simply
 *   calling the gate on the qubits.  In other words calling MyGate.on(q1, q2)
 *   to create an Operation on q1 and q2 is equivalent to MyGate(q1,q2).
 *   Gates operate on a certain number of qubits. All implementations of gate
 *   must implement the `num_qubits` method declaring how many qubits they
 *   act on. The gate feature classes `SingleQubitGate` and `TwoQubitGate`
 *   can be used to avoid writing this boilerplate.
 *   Linear combinations of gates can be created by adding gates together and
 *   multiplying them by scalars.
*/
pub trait Gate: QIdShape {
    /**
     * Checks if this gate can be applied to the given qubits.
     * By default checks that:
     * inputs are of type `Qid`
     * len(qubits) == num_qubits()
     * qubit_i.dimension == qid_shape[i] for all qubits
     */
    fn validate_args(&self, qubits: Vec<Box<dyn QId>>) -> Result<(), anyhow::Error> {
        if qubits.len() != self.qid_shape().len() {
            return Err(anyhow!(format!("The gate can't be applied to qubits")));
        }

        let qid_shape = self.qid_shape();
        for i in 0..qubits.len() {
            if qid_shape[i] != qubits[i].dimension() {
                return Err(anyhow!(format!("The gate can't be applied to qubits")));
            }
        }

        Ok(())
    }
}

/** An effect applied to a collection of qubits.
 * The most common kind of Operation is a GateOperation, which separates its
 * effect into a qubit-independent Gate and the qubits it should be applied to.
 */
pub trait Operation: QIdShape {
    fn gate(&self) -> Option<Box<dyn Gate>> {
        None
    }

    fn qubits(&self) -> Vec<Box<dyn QId>>;

    /** Returns the same operation, but applied to different qubits.
     *    Args:
     *      new_qubits: The new qubits to apply the operation to. The order must
     *          exactly match the order of qubits returned from the operation's
     *          `qubits` property.
     */
    fn with_qubits(&self, new_qubits: Vec<Box<dyn QId>>) -> Box<dyn Operation>;

    /// Returns a vector of the operation's tags.
    fn tags(&self) -> Vec<Box<dyn Hashable>>;

    /// Returns the underlying operation without any tags.
    fn untagged(&self) -> Box<dyn Operation>;

    /**
    * Creates a new TaggedOperation, with this op and the specified tags.
    * This method can be used to attach meta-data to specific operations
    * without affecting their functionality.  The intended usage is to
    * attach classes intended for this purpose or strings to mark operations
    * for specific usage that will be recognized by consumers.  Specific
    * examples include ignoring this operation in optimization passes,
    * hardware-specific functionality, or circuit diagram customizability.
    * Tags can be a list of any type of object that is useful to identify
    * this operation as long as the type is hashable.  If you wish the
    * resulting operation to be eventually serialized into JSON, you should
    * also restrict the operation to be JSON serializable.
    * Args:
    *    new_tags: The tags to wrap this operation in.
    */
    fn with_tags(&self, new_tags: Vec<Box<dyn Hashable>>) -> TaggedOperation;



}

pub struct TaggedOperation {

}

impl TaggedOperation {
    pub fn new<T: Operation, H: Hashable>(sub_operation: T, new_tags: Vec<H>) -> Self {
        Self {}
    }
}
