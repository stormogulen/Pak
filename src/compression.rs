
use Error;
use lz4;
use std::io::{self, Read};

pub fn compress<R: Read>(mut input: R) -> Result<(u64, Vec<u8>), Error> {

    let mut encoder = lz4::EncoderBuilder::new().build(Vec::new())
        .map_err(|_| Error::CompressionError)?;

    let size = io::copy(&mut input, &mut encoder).map_err(|_| Error::CompressionError)?;

    let (buf, result) = encoder.finish();
    result.map_err(|_| Error::CompressionError)?;

    Ok((size, buf))
}

pub fn decompress<R: Read>(input: R) -> Result<Vec<u8>, Error> {

    let mut buf = Vec::new();

    let mut decoder = lz4::Decoder::new(input).map_err(|_| Error::DecompressionError)?;

    io::copy(&mut decoder, &mut buf).map_err(|_| Error::DecompressionError)?;

    Ok(buf)
}
