use std::fs;
use std::io;
use std::marker::PhantomData;

use ext_sort::chunk::ExternalChunk;

pub struct BincodeExternalChunk<T> {
    reader: io::Take<io::BufReader<fs::File>>,
    item_type: PhantomData<T>,
}

impl<T> ExternalChunk<T> for BincodeExternalChunk<T>
where
    T: serde::ser::Serialize + serde::de::DeserializeOwned,
{
    type SerializationError = bincode::Error;
    type DeserializationError = bincode::Error;

    fn new(reader: io::Take<io::BufReader<fs::File>>) -> Self {
        Self {
            reader,
            item_type: PhantomData,
        }
    }

    fn dump(
        mut chunk_writer: &mut io::BufWriter<fs::File>,
        items: impl IntoIterator<Item = T>,
    ) -> Result<(), Self::SerializationError> {
        for item in items.into_iter() {
            bincode::serialize_into(&mut chunk_writer, &item)?;
        }
        Ok(())
    }
}

impl<T> Iterator for BincodeExternalChunk<T>
where
    T: serde::ser::Serialize + serde::de::DeserializeOwned,
{
    type Item = Result<T, <Self as ExternalChunk<T>>::DeserializationError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reader.limit() == 0 {
            None
        } else {
            match bincode::deserialize_from(&mut self.reader) {
                Ok(result) => Some(Ok(result)),
                Err(err) => Some(Err(err)),
            }
        }
    }
}
