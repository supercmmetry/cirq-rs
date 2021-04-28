use std::cmp::Ordering;

use anyhow::Error;

/// Identifies a quantum object such as a qubit, qudit, resonator, etc.
pub trait QId {
    /**
     * Returns the dimension or the number of quantum levels this qid has.
     * E.g. 2 for a qubit, 3 for a qutrit, etc.
     */
    fn dimension(&self) -> u64;

    /// Validates dimension
    fn validate_dimension(&self, dimension: u64) -> Result<(), anyhow::Error>;
}

#[derive(Clone)]
struct QubitAsQid<'a> {
    comparison_key: String,
    pub qubit: &'a dyn QId,
    pub dimension: u64,
}

impl PartialEq for QubitAsQid<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.comparison_key == other.comparison_key
    }
}

impl Eq for QubitAsQid<'_> {}

impl PartialOrd for QubitAsQid<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.comparison_key.partial_cmp(&other.comparison_key)
    }
}

impl Ord for QubitAsQid<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.comparison_key.cmp(&other.comparison_key)
    }
}

impl QId for QubitAsQid<'_> {
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

impl<'a> QubitAsQid<'a> {
    pub fn new<T: QId>(qubit: &'a T, dimension: u64) -> Self {
        let q = Self {
            comparison_key: "".to_string(),
            qubit,
            dimension,
        };

        q.validate_dimension(dimension);
        q
    }

    /// Returns a new QId with a different dimension.
    fn with_dimension(&'a self, dimension: u64) -> Self {
        if self.dimension == dimension {
            self.clone()
        } else {
            QubitAsQid::new(self, dimension)
        }
    }
}
