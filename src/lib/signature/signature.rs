use crate::DataOsUri;

#[derive(Clone)]
pub struct Signature {
    signer_app: DataOsUri,
    signature: String,
    signer_fingerprint: String,
}

impl Signature {
    
    pub fn new(signer_app: DataOsUri, signature: String, signer_fingerprint: String) -> Self {
        Signature {
            signer_app,
            signature,
            signer_fingerprint,
        }
    }

    pub fn signer_app(&self) -> &DataOsUri {
        &self.signer_app
    }

    pub fn signature(&self) -> &str {
        self.signature.as_ref()
    }

    pub fn signer_fingerprint(&self) -> &str {
        self.signer_fingerprint.as_ref()
    }
}