use ed25519_dalek::{Signature, SigningKey, Signer, VerifyingKey};
use hmac::{Hmac, Mac};
use sha3::Sha3_256;
use hkdf::Hkdf;

type HmacSha3_256 = Hmac<Sha3_256>;

/// Sign a message with Ed25519.
pub fn sign_ed25519(message: &[u8], sk_bytes: &[u8; 32]) -> [u8; 64] {
    let sk = SigningKey::from_bytes(sk_bytes);
    let sig: Signature = sk.sign(message);
    sig.to_bytes()
}

/// Verify an Ed25519 signature.
pub fn verify_ed25519(message: &[u8], sig_bytes: &[u8; 64], pk_bytes: &[u8; 32]) -> bool {
    let sig = Signature::from_bytes(sig_bytes);
    let pk = match VerifyingKey::from_bytes(pk_bytes) {
        Ok(k) => k,
        Err(_) => return false,
    };
    pk.verify_strict(message, &sig).is_ok()
}

/// Derive a domain-separated nullifier key from the signing key.
/// This ensures nullifiers cannot leak information about sk.
/// nullifier_key = HKDF-SHA3-256(ikm=sk, info="comum-nullifier-v1")
fn derive_nullifier_key(sk_bytes: &[u8; 32]) -> [u8; 32] {
    let hk = Hkdf::<Sha3_256>::new(None, sk_bytes);
    let mut okm = [0u8; 32];
    hk.expand(b"comum-nullifier-v1", &mut okm)
        .expect("HKDF expand failed");
    okm
}

/// Compute a nullifier for a testimony.
/// nullifier = HMAC-SHA3-256(nullifier_key, testimony_id)
/// where nullifier_key = HKDF(sk, "comum-nullifier-v1")
///
/// This construction ensures:
/// 1. Nullifiers are unique per (sk, testimony_id) pair.
/// 2. Exposing a nullifier does not leak sk.
/// 3. The same sk always produces the same nullifier for a given id (deterministic).
pub fn compute_nullifier(sk_bytes: &[u8; 32], testimony_id: &[u8; 32]) -> [u8; 32] {
    let nk = derive_nullifier_key(sk_bytes);
    let mut mac = HmacSha3_256::new_from_slice(&nk)
        .expect("HMAC key length is valid");
    mac.update(testimony_id);
    let result = mac.finalize().into_bytes();
    let mut out = [0u8; 32];
    out.copy_from_slice(&result[..32]);
    out
}
