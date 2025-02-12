use ed25519_dalek_bip32::ChildIndex;
use ed25519_dalek_bip32::DerivationPath;
use ed25519_dalek_bip32::ExtendedSigningKey;
use ed25519_dalek_bip32::SigningKey;
use rustler::Binary;
use rustler::Encoder;
use rustler::Env;
use rustler::NewBinary;
use rustler::Term;

mod atoms {
    rustler::atoms! {
        ok,
        error,
        wrong_seed_size,
        wrong_secret_key_size,
        wrong_signing_key,
        wrong_chain_code_size,
        invalid_path
    }
}

rustler::init!("Elixir.Guarana.DerivationImpl");

#[rustler::nif]
fn master_key_from_seed<'a>(env: Env<'a>, seed: Binary, hmac_key: String) -> Term<'a> {
    let seed_arr: [u8; 32] = match seed.as_slice().try_into() {
        Ok(array) => array,
        Err(_) => return (atoms::error(), atoms::wrong_seed_size()).encode(env),
    };

    let extended_signing_key =
        match ExtendedSigningKey::from_seed_with_custom_hmac_key(&seed_arr, &hmac_key) {
            Ok(signing_key) => signing_key,
            Err(_) => return (atoms::error(), atoms::wrong_signing_key()).encode(env),
        };

    serialize_extended_key(env, extended_signing_key)
}

#[rustler::nif]
fn derive_child_key<'a>(
    env: Env<'a>,
    depth: u8,
    child_index: u32,
    signing_key_bin: Binary,
    chain_code_bin: Binary,
    path: String,
) -> Term<'a> {
    let chain_code: [u8; 32] = match chain_code_bin.as_slice().try_into() {
        Ok(array) => array,
        Err(_) => return (atoms::error(), atoms::wrong_chain_code_size()).encode(env),
    };

    let signing_key_array: [u8; 64] = match signing_key_bin.as_slice().try_into() {
        Ok(array) => array,
        Err(_) => return (atoms::error(), atoms::wrong_signing_key()).encode(env),
    };

    let signing_key = match SigningKey::from_keypair_bytes(&signing_key_array) {
        Ok(key) => key,
        Err(_) => return (atoms::error(), atoms::wrong_signing_key()).encode(env),
    };

    let derivation_path: DerivationPath = match path.parse() {
        Ok(path) => path,
        Err(_) => return (atoms::error(), atoms::invalid_path()).encode(env),
    };

    let signing_key = ExtendedSigningKey {
        depth: depth,
        child_index: ChildIndex::from_u32(child_index),
        chain_code: chain_code,
        signing_key: signing_key,
    };

    match signing_key.derive(&derivation_path) {
        Ok(key) => serialize_extended_key(env, key),
        Err(_) => return (atoms::error(), atoms::invalid_path()).encode(env),
    }
}

fn serialize_extended_key<'a>(env: Env<'a>, extended_signing_key: ExtendedSigningKey) -> Term<'a> {
    (
        atoms::ok(),
        (
            extended_signing_key.depth,
            extended_signing_key.child_index.to_u32_with_info(),
            create_binary_from_slice(env, &extended_signing_key.signing_key.to_keypair_bytes()),
            create_binary_from_slice(env, &extended_signing_key.chain_code),
        ),
    )
        .encode(env)
}

fn create_binary_from_slice<'a>(env: Env<'a>, slice: &[u8]) -> Binary<'a> {
    let size = slice.len();
    let mut binary = NewBinary::new(env, size);

    binary.as_mut_slice().copy_from_slice(slice);

    Binary::from(binary)
}
