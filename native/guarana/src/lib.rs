use hd_wallet::{curves::Ed25519, Edwards, HdWallet};
use rustler::Binary;
use rustler::Encoder;
use rustler::Env;
use rustler::ListIterator;
use rustler::NewBinary;
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

rustler::init!("Elixir.Guarana.Impl");

#[rustler::nif]
fn derive_key<'a>(
    env: Env<'a>,
    secret_key: Binary,
    chain_code: Binary,
    path_list: ListIterator<'a>,
) -> Term<'a> {
    let extended_key = match load_extended_key(env, secret_key, chain_code) {
        Ok(result) => result,
        Err(error) => return error,
    };

    let path = path_list
        .map(|x| x.decode::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let child_key_pair = Edwards::derive_child_key_pair_with_path(&extended_key, path);

    let mut public_key = NewBinary::new(env, 32);
    public_key
        .as_mut_slice()
        .copy_from_slice(&child_key_pair.public_key().public_key.to_bytes(false)[..]);

    let mut public_key_chain_code = NewBinary::new(env, 32);
    public_key_chain_code
        .as_mut_slice()
        .copy_from_slice(&child_key_pair.public_key().chain_code);

    let mut private_key = NewBinary::new(env, 64);
    private_key.as_mut_slice().copy_from_slice(
        &child_key_pair
            .secret_key()
            .secret_key
            .as_ref()
            .to_be_bytes()[..],
    );

    let mut private_key_chain_code = NewBinary::new(env, 32);
    private_key_chain_code
        .as_mut_slice()
        .copy_from_slice(&child_key_pair.secret_key().chain_code);

    (
        Binary::from(public_key),
        Binary::from(private_key_chain_code),
        Binary::from(private_key),
        Binary::from(public_key_chain_code),
    )
        .encode(env)
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
