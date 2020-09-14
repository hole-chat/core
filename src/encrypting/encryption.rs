use ntru::rand::RNG_DEFAULT;
use ntru::encparams::{DEFAULT_PARAMS_256_BITS};
pub use ntru::types::{KeyPair, PrivateKey, PublicKey};
use std::str;
use std::u8;
extern crate ntru;



pub fn u8_to_string(key: &Box<[u8]>) -> String {
    let utf = unsafe {str::from_utf8_unchecked(&key)};
    let string = String::from(utf);
    return String::from(string);
}

pub fn public_key_from_sring (pk: String) -> PublicKey {
    let bytes = pk.into_bytes();
    let imported = PublicKey::import(&bytes);
    return imported
}

pub fn private_key_from_sring (pk: String) -> PrivateKey {
    let bytes = pk.into_bytes();
    let imported = PrivateKey::import(&bytes);
    return imported
}


pub fn generate_kp () -> (String, String, KeyPair) {
    let rand_ctx            = ntru::rand::init(&RNG_DEFAULT).unwrap();
    let kp                  = ntru::generate_key_pair(&DEFAULT_PARAMS_256_BITS, &rand_ctx).unwrap();
    // extracting public and private key from kp
    let pub_key             = KeyPair::get_public(&kp);
    let private_key         = KeyPair::get_private(&kp);
    // getting pub and priv key params for exporting
    let pub_key_params      = KeyPair::get_params(&kp).unwrap();
    let private_key_params  = PrivateKey::get_params(&private_key).unwrap();
    // exporting
    let pub_key_exported:       Box<[u8]>       = PublicKey::export(&pub_key, &pub_key_params);
    let private_key_exported:   Box<[u8]>       = PrivateKey::export(&private_key, &private_key_params);
    // converting to string
    let pub_key_string                          = u8_to_string(&pub_key_exported);
    let private_key_string                      = u8_to_string(&private_key_exported);
    return (pub_key_string, private_key_string, kp )
}

pub fn kp_from_string (public: String,private: String) -> KeyPair {
    let pub_key = public_key_from_sring(public);
    let priv_key = private_key_from_sring(private);
    
    let keypair = KeyPair::new(priv_key, pub_key);

    return keypair;

}

pub fn encrypt_message (msg:String, key: &PublicKey ) -> String {

    // to bytes
    let _msg = msg.into_bytes();
    //encrypting
    let rand_ctx = ntru::rand::init(&RNG_DEFAULT).unwrap();
    let encrypted = ntru::encrypt(&_msg, &key, &DEFAULT_PARAMS_256_BITS,
                              &rand_ctx).unwrap();
    //to string
    let message = u8_to_string(&encrypted);
    return message
}

pub fn decrypt_message(msg: String, kp: &KeyPair, final_msg: &mut String) {
    let encrypted_message = msg.into_bytes();
    let decrypted = ntru::decrypt(&encrypted_message, &kp, &DEFAULT_PARAMS_256_BITS);
    match decrypted {
        Ok(res) => {
            *final_msg = u8_to_string(&res)
        },
        Err(e) => {}            
    }
}




