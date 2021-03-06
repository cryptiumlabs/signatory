//! ledger-tm provider: Ledger Tendermint Validator app (Ed25519 signatures for amino votes)

#![deny(warnings, missing_docs, trivial_casts, trivial_numeric_casts)]
#![deny(unsafe_code, unused_import_braces, unused_qualifications)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/tendermint/signatory/master/img/signatory-rustacean.png",
    html_root_url = "https://docs.rs/signatory-ledger-tm/0.11.0"
)]

use std::sync::{Arc, Mutex};

use signatory::{
    ed25519::{PublicKey, Signature},
    error::{Error, ErrorKind},
    PublicKeyed, Signer,
};

/// ed25519 signature provider for the Ledger Tendermint Validator app
pub struct Ed25519LedgerTmAppSigner {
    app: Arc<Mutex<ledger_tendermint::TendermintValidatorApp>>,
}

impl Ed25519LedgerTmAppSigner {
    /// Create a new Ed25519 signer based on Ledger Nano S - Tendermint Validator app
    pub fn connect() -> Result<Self, Error> {
        match ledger_tendermint::TendermintValidatorApp::connect() {
            Ok(validator_app) => {
                let app = Arc::new(Mutex::new(validator_app));
                let signer = Ed25519LedgerTmAppSigner { app };
                let _pk = signer.public_key().unwrap();
                Ok(signer)
            }
            Err(err) => Err(Error::new(ErrorKind::ProviderError, Some(&err.to_string()))),
        }
    }
}

impl PublicKeyed<PublicKey> for Ed25519LedgerTmAppSigner {
    /// Returns the public key that corresponds to the Tendermint Validator app connected to this signer
    fn public_key(&self) -> Result<PublicKey, Error> {
        let app = self.app.lock().unwrap();

        match app.public_key() {
            Ok(pk) => Ok(PublicKey(pk)),
            Err(err) => Err(Error::new(ErrorKind::ProviderError, Some(&err.to_string()))),
        }
    }
}

impl Signer<Signature> for Ed25519LedgerTmAppSigner {
    /// c: Compute a compact, fixed-sized signature of the given amino/json vote
    fn sign(&self, msg: &[u8]) -> Result<Signature, Error> {
        let app = self.app.lock().unwrap();

        match app.sign(&msg) {
            Ok(sig) => Ok(Signature(sig)),
            Err(err) => Err(Error::new(ErrorKind::ProviderError, Some(&err.to_string()))),
        }
    }
}

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use crate::Ed25519LedgerTmAppSigner;

    lazy_static! {
        static ref SIGNER: Mutex<Ed25519LedgerTmAppSigner> =
            Mutex::new(Ed25519LedgerTmAppSigner::connect().unwrap());
    }

    #[test]

    fn public_key() {        
        use signatory::PublicKeyed;

        let signer = SIGNER.lock().unwrap();

        let _pk = signer.public_key().unwrap();
        println!("PK {:0X?}", _pk);
    }

    #[test]
    #[ignore]
    fn sign() {
        use signatory::Signer;

        let signer = SIGNER.lock().unwrap();

        // Sign message1
        let some_message1 = [
            33, 0x8,  // (field_number << 3) | wire_type
            0x1,  // PrevoteType
            0x11, // (field_number << 3) | wire_type
            0x10, 0x00, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,  // height
            0x19, // (field_number << 3) | wire_type
            0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,  // round
            0x22, // (field_number << 3) | wire_type
            // remaining fields (timestamp):
            0xb, 0x8, 0x80, 0x92, 0xb8, 0xc3, 0x98, 0xfe, 0xff, 0xff, 0xff, 0x1,
        ];

        signer.sign(&some_message1).unwrap();
    }

    #[test]
    fn sign2() {
        use signatory::Signer;

        let signer = SIGNER.lock().unwrap();

        // Sign message1
        let some_message1 = [
            33, 0x8,  // (field_number << 3) | wire_type
            0x1,  // PrevoteType
            0x11, // (field_number << 3) | wire_type
            0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,  // height
            0x19, // (field_number << 3) | wire_type
            0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,  // round
            0x22, // (field_number << 3) | wire_type
            // remaining fields (timestamp):
            0xb, 0x8, 0x80, 0x92, 0xb8, 0xc3, 0x98, 0xfe, 0xff, 0xff, 0xff, 0x1,
        ];

        signer.sign(&some_message1).unwrap();

        // Sign message2
        let some_message2 = [
            33, 0x8,  // (field_number << 3) | wire_type
            0x1,  // PrevoteType
            0x11, // (field_number << 3) | wire_type
            0x10, 0x00, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,  // height
            0x19, // (field_number << 3) | wire_type
            0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,  // round
            0x22, // (field_number << 3) | wire_type
            // remaining fields (timestamp):
            0xb, 0x8, 0x80, 0x92, 0xb8, 0xc3, 0x98, 0xfe, 0xff, 0xff, 0xff, 0x1,
        ];

        signer.sign(&some_message2).unwrap();
    }

    #[test]
    #[ignore]
    fn sign_many() {
        use signatory::PublicKeyed;
        use signatory::Signer;

        let signer = SIGNER.lock().unwrap();

        // Get public key to initialize
        let _pk = signer.public_key().unwrap();
        println!("PK {:0X?}", _pk);

        for index in 50u8..254u8 {
            // Sign message1
            let some_message = [
                33, 0x8,  // (field_number << 3) | wire_type
                0x1,  // PrevoteType
                0x11, // (field_number << 3) | wire_type
                0x40, 0x00, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,  // height
                0x19, // (field_number << 3) | wire_type
                index, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,  // round
                0x22, // (field_number << 3) | wire_type
                // remaining fields (timestamp):
                0xb, 0x8, 0x80, 0x92, 0xb8, 0xc3, 0x98, 0xfe, 0xff, 0xff, 0xff, 0x1,
            ];

            signer.sign(&some_message).unwrap();
        }
    }
}
