use crate::store::Store;
use std::fmt::Debug;
use std::io::Write;

pub trait PersistenceBackend: Debug {
    fn load(&self) -> Option<Vec<u8>>;
    fn save(&self, data: &[u8]);
}

#[derive(Debug, Default)]
pub struct NullBackend;

impl PersistenceBackend for NullBackend {
    fn load(&self) -> Option<Vec<u8>> {
        None
    }

    fn save(&self, _data: &[u8]) {}
}

pub fn load(backend: &dyn PersistenceBackend) -> Store {
    backend
        .load()
        .and_then(|raw| decode(&raw))
        .unwrap_or_default()
}

pub fn save(backend: &dyn PersistenceBackend, store: &Store) {
    if let Some(raw) = encode(store) {
        backend.save(&raw);
    }
}

fn encode(store: &Store) -> Option<Vec<u8>> {
    let packed = rmp_serde::to_vec_named(store).ok()?;
    let mut out = Vec::new();
    {
        let mut writer = brotli::CompressorWriter::new(&mut out, 4096, 5, 22);
        writer.write_all(&packed).ok()?;
    }
    Some(out)
}

fn decode(raw: &[u8]) -> Option<Store> {
    let mut packed = Vec::new();
    {
        let mut writer = brotli::DecompressorWriter::new(&mut packed, 4096);
        writer.write_all(raw).ok()?;
    }
    rmp_serde::from_slice(&packed).ok()
}
