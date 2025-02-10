use hd_wallet::{curves::Ed25519, Edwards, HdWallet};
use rustler::Binary;
use rustler::Encoder;
use rustler::Env;
use rustler::ListIterator;
use rustler::NewBinary;
use rustler::NifResult;
use rustler::Term;

mod atoms {
    rustler::atoms! {
        ok,
        error,
        wrong_secret_key_size,
        wrong_chain_code_size,
        invalid_path
    }
}

fn derive_key(
    env: Env<'a>,
    secret_key: Binary,
    chain_code: Binary,
    path_list: ListIterator<'a>,
) -> Term<'a> {
    let extended_key = load_extended_key(env, secret_key, chain_code)?;

    let path = path_list
        .map(|x| match x.decode::<i64>() {
            Ok(number) => number,
            Err(_) => return Err((atoms::error(), atoms::invalid_path()).encode(env)),
        })
        .collect::<NifResult<Vec<i64>>>();

    let child_key_pair = Edwards::derive_child_key_pair_with_path(&extended_key, path);

    (
        Binary::from(child_key_pair.secret_key().secret_key),
        child_key_pair.secret_key().chain_code,
        Binary::from(child_key_pair.public_key().public_key),
        child_key_pair.public_key().chain_code,
    )
}

fn load_extended_key<'a>(
    env: Env<'a>,
    secret_key: Binary,
    chain_code: Binary,
) -> Result<hd_wallet::ExtendedKeyPair<Ed25519>, Term<'a>> {
    let parsed_secret_key: [u8; 32] = match secret_key.as_slice().try_into() {
        Ok(array) => array,
        Err(_) => return Err((atoms::error(), atoms::wrong_secret_key_size()).encode(env)),
    };

    let parsed_chain_code: [u8; 32] = match chain_code.as_slice().try_into() {
        Ok(array) => array,
        Err(_) => return Err((atoms::error(), atoms::wrong_secret_key_size()).encode(env)),
    };

    Ok(hd_wallet::ExtendedSecretKey {
        secret_key: generic_ec::SecretScalar::from_be_bytes(&parsed_secret_key).unwrap(),
        chain_code: parsed_chain_code,
    }
    .into())
}
