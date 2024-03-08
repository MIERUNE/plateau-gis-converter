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
    type SerializationError = bincode::error::EncodeError;
    type DeserializationError = bincode::error::DecodeError;

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
        let bincode_config = bincode::config::standard();
        for item in items.into_iter() {
            bincode::serde::encode_into_std_write(&item, &mut chunk_writer, bincode_config)?;
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
        let bincode_config = bincode::config::standard();
        if self.reader.limit() == 0 {
            None
        } else {
            match bincode::serde::decode_from_std_read(&mut self.reader, bincode_config) {
                Ok(result) => Some(Ok(result)),
                Err(err) => Some(Err(err)),
            }
        }
    }
}
