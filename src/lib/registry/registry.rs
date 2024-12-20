use crate::DataOsUri;
use super::{RegistryInput, RegistryReadResult, RegistryWriteResult};

pub trait Registry {
    fn new() -> Self;
    fn get(&self, uri: DataOsUri) -> Option<RegistryReadResult>;
    fn add(&mut self, input: RegistryInput) -> RegistryWriteResult;
}
