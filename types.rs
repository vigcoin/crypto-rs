
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub struct ba32 {
data:[u8;32]
}
pub struct ba64 {
data:[u8;64]
}

pub type elliptic_curve_point_t = ba32;
pub type elliptic_curve_scalar_t = ba32;

pub type hash_t = ba32;
pub type public_key_t = ba32;
pub type secret_key_t = ba32;
pub type key_derivation_t = ba32;
pub type key_image_t = ba32;
pub type signature_t = ba64;

pub struct account_public_address_t
{
    spendPublicKey: public_key_t,
    viewPublicKey: public_key_t,
}

pub struct account_keys_t
{
    address: account_public_address_t,
    spendSecretKey: secret_key_t,
    viewSecretKey: secret_key_t,
}

pub struct key_pair_t
{
    publicKey: public_key_t,
    secretKey:secret_key_t,
}