use hole::encrypting::encryption::*;
use std::collections::HashMap;
use ntru::types::{KeyPair, PrivateKey, PublicKey};


#[test]
fn it_converting_pubic_key () {
    let (public, private, keypair) = generate_kp();

    let initial_pub     = KeyPair::get_public(&keypair);
    let final_pub   =  public_key_from_sring(public);

    let pub_eq = initial_pub  == &final_pub;

    assert_eq!(initial_pub, &final_pub);
    
}

#[test] fn it_converting_private_key () {
    let (public, private, keypair) = generate_kp();

    let initial_priv    = KeyPair::get_private(&keypair);
    let final_priv  =  private_key_from_sring(private);

    let priv_eq = initial_priv == &final_priv;

    assert_eq!(initial_priv, &final_priv);
}

#[test]
fn it_making_same_keypair () {
    let (public, private, keypair) = generate_kp();

    let new_keypair = kp_from_string(public, private);
    assert_eq!(new_keypair, keypair);
    
}

#[test]
fn it_making_correct_decrypt () {
   let (_, _, keypair) = generate_kp();

   let public = KeyPair::get_public(&keypair);


   let msg = "👻: it's a ghost. Ghost is unexpectively bloodthirsty".to_string();
   let my_msg = "👻: it's a ghost. Ghost is unexpectively bloodthirsty".to_string();
 

   let enc_msg = encrypt_message(msg, public);
   let dec_msg = decrypt_message(enc_msg, &keypair);
   assert_eq!(my_msg, dec_msg) 
}

