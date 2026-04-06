use ed25519_dalek::{Signature, SigningKey, VerifyingKey, Signer};

pub fn sign_ed25519(message: &[u8], sk_bytes: &[u8; 32]) -> [u8; 64] {
    let sk = SigningKey::from_bytes(sk_bytes);
    let sig: Signature = sk.sign(message);
    sig.to_bytes()
}

pub fn verify_ed25519(message: &[u8], sig_bytes: &[u8; 64], pk_bytes: &[u8; 32]) -> bool {
    let sig = Signature::from_bytes(sig_bytes);
    let pk = match VerifyingKey::from_bytes(pk_bytes) {
        Ok(k) => k,
        Err(_) => return false,
    };
    pk.verify_strict(message, &sig).is_ok()
}
