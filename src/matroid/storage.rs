use std::io::Read;
use std::io::Write;

use std::error::Error;
use std::path::Path;

use super::BasesMatroid;
use super::Matroid;

use crate::set::Set;

use postcard::{from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct StoredMatroid {
    // The number of elements in the matroid.
    pub n: usize,
    // The rank of the matroid
    pub k: usize,
    // The set of bases
    pub bases: Vec<Set>,
}

impl<M: Matroid> From<&M> for StoredMatroid {
    fn from(matroid: &M) -> Self {
        let bases = matroid.bases();
        StoredMatroid {
            n: matroid.n(),
            k: matroid.k(),
            bases,
        }
    }
}

impl From<StoredMatroid> for BasesMatroid {
    fn from(stored: StoredMatroid) -> Self {
        BasesMatroid::new(stored.bases, stored.n, stored.k)
    }
}

impl StoredMatroid {
    /// Store the matroid in a file.
    #[allow(unused)]
    pub fn to_file(&self, filename: &Path) -> Result<(), Box<dyn Error>> {
        // set the correct extension
        let mut path = filename.to_path_buf();
        path.set_extension("matroid");

        let mut file = std::fs::File::create(path)?;
        self.save(&mut file)
    }

    /// Load the matroid from a file.
    #[allow(unused)]
    pub fn from_file(filename: &Path) -> Result<Self, Box<dyn Error>> {
        // set the extension
        let mut path = filename.to_path_buf();
        path.set_extension("matroid");

        let mut file = std::fs::File::open(path)?;
        Self::load(&mut file)
    }

    /// Save the matroid to a writer.
    #[allow(unused)]
    pub fn save<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        // Serialize the matroid
        let bytes = to_allocvec(self)?;
        // Write the bytes to the writer
        writer.write_all(&bytes)?;
        Ok(())
    }

    /// Load a matroid from a reader.
    #[allow(unused)]
    pub fn load<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let mut bytes = Vec::new();
        // read the bytes from the reader
        reader.read_to_end(&mut bytes)?;
        // Deserialize the matroid
        let stored = from_bytes(&bytes)?;

        Ok(stored)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::matroid::UniformMatroid;

    use std::env::temp_dir;
    use uuid::Uuid;

    #[test]
    fn test_save_load() {
        let matroid = UniformMatroid::new(3, 6);
        let stored = StoredMatroid::from(&matroid);
        let mut path = temp_dir();
        path.push(Uuid::new_v4().to_string());
        stored.to_file(&path).unwrap();
        let loaded = StoredMatroid::from_file(&path).unwrap();

        assert_eq!(stored, loaded);
    }
}
