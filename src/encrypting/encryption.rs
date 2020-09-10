use ntru::rand::RNG_DEFAULT;
use ntru::encparams::{DEFAULT_PARAMS_256_BITS};
use ntru::types::{KeyPair, PrivateKey, PublicKey};
use std::str;
extern crate ntru;



pub fn key_to_string(key: &Box<[u8]>) -> String {
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


pub fn generate () -> (String, String, KeyPair) {
    let rand_ctx = ntru::rand::init(&RNG_DEFAULT).unwrap();
    let kp       = ntru::generate_key_pair(&DEFAULT_PARAMS_256_BITS, &rand_ctx).unwrap();
    // extracting public and private key from kp
    let pub_key     = KeyPair::get_public(&kp);
    let private_key = KeyPair::get_private(&kp);
    // getting pub and priv key params for exporting
    let pub_key_params     = KeyPair::get_params(&kp).unwrap();
    let private_key_params = PrivateKey::get_params(&private_key).unwrap();
    // exporting 
    let pub_key_exported:       Box<[u8]>       = PublicKey::export(&pub_key, &pub_key_params);
    let private_key_exported:   Box<[u8]>       = PrivateKey::export(&private_key, &private_key_params);
    // converting to string
    let pub_key_string          = key_to_string(&pub_key_exported);
    let private_key_string         = key_to_string(&private_key_exported);
    /* 
     * It was a test 
     *
    
    let xp = &PublicKey::import(&pub_key_exported);
    let boo = xp == pub_key; 

    let value = from_utf8(&pub_key_exported).unwrap();
    let strVal = String::from(value);

    print!(" without converting \n {}", xp == pub_key );
    assert_eq!(xp, pub_key);

    let newbytes = strVal.into_bytes();
    let newkey = PublicKey::import(&newbytes);

    print!(" with converting \n {}", &newkey == pub_key);
    assert_eq!(&newkey , pub_key);
    */    
    return (pub_key_string, private_key_string, kp )
}


pub fn main () {
    let (public, private, key_pair) = generate();


    let initial_pub     = KeyPair::get_public(&key_pair);
    let initial_priv    = KeyPair::get_private(&key_pair);

    let final_pub   =  public_key_from_sring(public);
    let final_priv  =  private_key_from_sring(private);

    let bo0 = initial_pub  == &final_pub;
    let bo1 = initial_priv == &final_priv;

    print!("Pub : {}, Priv : {}", bo0, bo1);

    assert_eq!(initial_pub, &final_pub);
    assert_eq!(initial_priv, &final_priv);
}
