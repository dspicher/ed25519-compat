use ed25519_dalek::Signer;

fn main() {
    let keypair_dalek = ed25519_dalek::Keypair::generate(&mut rand::thread_rng());
    let sig_dalek = keypair_dalek.sign(b"hello".as_ref()).to_bytes();
    let keypair_compact = ed25519_compact::KeyPair::from_slice(&keypair_dalek.to_bytes()).unwrap();
    let sig_compact = keypair_compact.sk.sign(b"hello", None);
    assert_eq!(sig_dalek, *sig_compact);
}
