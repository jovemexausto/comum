use std::collections::BTreeMap;

use frost_ed25519 as frost;
use rand::rngs::OsRng;

pub fn frost_keygen_with_dealer(
    max_signers: u16,
    min_signers: u16,
) -> Result<
    (
        BTreeMap<frost::Identifier, frost::keys::KeyPackage>,
        frost::keys::PublicKeyPackage,
    ),
    frost::Error,
> {
    let mut rng = OsRng;
    let (shares, pubkey_package) = frost::keys::generate_with_dealer(
        max_signers,
        min_signers,
        frost::keys::IdentifierList::Default,
        &mut rng,
    )?;

    let mut key_packages = BTreeMap::new();
    for (identifier, secret_share) in shares {
        let key_package = frost::keys::KeyPackage::try_from(secret_share)?;
        key_packages.insert(identifier, key_package);
    }

    Ok((key_packages, pubkey_package))
}

pub fn frost_sign(
    message: &[u8],
    signing_ids: &[frost::Identifier],
    key_packages: &BTreeMap<frost::Identifier, frost::keys::KeyPackage>,
    pubkey_package: &frost::keys::PublicKeyPackage,
) -> Result<frost::Signature, frost::Error> {
    let mut rng = OsRng;
    let mut nonces_map = BTreeMap::new();
    let mut commitments_map = BTreeMap::new();

    for identifier in signing_ids {
        let key_package = key_packages
            .get(identifier)
            .ok_or(frost::Error::UnknownIdentifier)?;
        let (nonces, commitments) = frost::round1::commit(key_package.signing_share(), &mut rng);
        nonces_map.insert(*identifier, nonces);
        commitments_map.insert(*identifier, commitments);
    }

    let signing_package = frost::SigningPackage::new(commitments_map, message);
    let mut signature_shares = BTreeMap::new();

    for identifier in signing_ids {
        let key_package = &key_packages[identifier];
        let nonces = &nonces_map[identifier];
        let signature_share = frost::round2::sign(&signing_package, nonces, key_package)?;
        signature_shares.insert(*identifier, signature_share);
    }

    frost::aggregate(&signing_package, &signature_shares, pubkey_package)
}

pub fn frost_verify(
    message: &[u8],
    signature: &frost::Signature,
    pubkey_package: &frost::keys::PublicKeyPackage,
) -> bool {
    pubkey_package
        .verifying_key()
        .verify(message, signature)
        .is_ok()
}
