mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
use base64::{engine::general_purpose, Engine as _};

use rsa::{
  pkcs1::DecodeRsaPrivateKey, pkcs8::DecodePublicKey, Oaep, PublicKey, RsaPrivateKey, RsaPublicKey,
};
/*
-----BEGIN RSA PRIVATE KEY-----
MIIEogIBAAKCAQBtzfjInEd3k5w5K2/bZ8QyC+2brq3E64/Zjl8oTLP8n75Ub1W7
CGTdpyXbwR1EstHex5SaXrfHNHNSoBabqSINHWcFzGTxMqTV1tHAgXi75a9WPST2
qg/LeZZ4uqL7mgRtvu8Hur0kdv3phoPZLNGokPvzEGijb6XhvLSU0/H4i9l7MWcT
2FTvwBuKxWSxaeICH2QckBrMFoyrWtCNwJUIMOWJcI/bFQfgwOVIyX+gz8i3dWcj
wCfsizo3AEtyPa953CjFDwqAs4Xtz0HMQugusLKOjwuFC4l3l4+l3PZMOUqZKFTS
bEMnlsg2Z9yw1AigeDLqr+hUlEySr2iTt33vAgMBAAECggEAH2V8lEfzO2Kukb40
zqR9+d1lRFWPBDZivE80MD62XC6E6nmnJsKArMwlTA+Mwd7rmWqxt+4LWF9L9j0m
Gb4jqWJV2deWqUzUyvwQn52lmvxSxTLYhdschw6uErky7wQb7hFhIXGosWuAsKww
uQFStYM3N0ni0ZCT+tbNP5zWnD1owGWvB/Mybk9ieU76NO0mAlvBEe1hqLUIUMva
7TdsbyhFybhBBmDeVeDdBztMteTEQg5fwoZc63eExb0MrU0f1bW/ghEJVXV1Zftu
Q3GLVoNiUYlGhC8JxYo3lyAQdHvtIGJa374FTFqmdcVc1ZPAtF/gbJjPqXHJI7De
VsxLYQKBgQCpfVM1Xel6cxVaip8y2LeoDiOUCttCN3vJYpp7xAus26vME+m4lWTs
WCjOrCQLuoJhxkIpTnyLqbnNQL5dsZp637t9GiT+iq2leDeVf+eQG53t/cwFZBMS
adnwMpVVRhpNoK3np2xY2TyH+SLMQlwKK5MpztoNzMHBob3rzv/5cQKBgQCl2cvJ
V82ftXyw1+kNmq63XvgJUIdff634al6aCUujKZmzliZCN+C0ctCwRgBlqW89PIwq
b3HEGY4H60o3Ke2hcbyZJTeDxydZWzN6NC3JU4gGhgeP7K5W1RLXLxZKOdL0VRh4
lmgYnOfvcLwIWEZRRRztSap9Lde54ARMc+o9XwKBgBhfPSffaUOCaP+sD1hwcXhp
EBSpuv26nMJIu44wkr04mWuvMVypnumdg8C0YDqPh1AoAjuOXbp3nZA2TymJ8Hza
z0seB1PS/UVeFCAbVGEUl5ExfbUIvvVW40/29iJdAktHX7qACMR9+IZU/PwCTtnn
ijHo3NI/L114tKbEDWsBAoGBAIzWf9SSCF54etOd9h2EXpv/PLiENwEk5rJOedlq
zV1YRVqYxhJzaxExcwBN4aZzFLC2yvx6OOzjGpak+xgPNelkMVkHnrX8F/EsnnFH
AB64HYoUpTVWMtIwDNjI9q+/nOG5pZc5elp0XA0b+cFIXSZEf4UNiobUuB3zGxNl
V+QXAoGAI/S5xJnhc40lldgOVYNMNaShL42YwjJO1/DnrZoyqjgOkjvxuyRQybsk
YXb4PV1oH9FnT10+Pj7JYOCgncZtDNM9vG5d5AqP2hdk0vCTT+ddhfxysH+HMiXD
JYaT5H2Aw1BOaoQXBIvCJch7xyaSYHwEEMPhFsaiiGaZqIItcFo=
-----END RSA PRIVATE KEY-----
*/

static PEM_PRIVATE_KEY: &str = "-----BEGIN RSA PRIVATE KEY-----
MIIEogIBAAKCAQBtzfjInEd3k5w5K2/bZ8QyC+2brq3E64/Zjl8oTLP8n75Ub1W7
CGTdpyXbwR1EstHex5SaXrfHNHNSoBabqSINHWcFzGTxMqTV1tHAgXi75a9WPST2
qg/LeZZ4uqL7mgRtvu8Hur0kdv3phoPZLNGokPvzEGijb6XhvLSU0/H4i9l7MWcT
2FTvwBuKxWSxaeICH2QckBrMFoyrWtCNwJUIMOWJcI/bFQfgwOVIyX+gz8i3dWcj
wCfsizo3AEtyPa953CjFDwqAs4Xtz0HMQugusLKOjwuFC4l3l4+l3PZMOUqZKFTS
bEMnlsg2Z9yw1AigeDLqr+hUlEySr2iTt33vAgMBAAECggEAH2V8lEfzO2Kukb40
zqR9+d1lRFWPBDZivE80MD62XC6E6nmnJsKArMwlTA+Mwd7rmWqxt+4LWF9L9j0m
Gb4jqWJV2deWqUzUyvwQn52lmvxSxTLYhdschw6uErky7wQb7hFhIXGosWuAsKww
uQFStYM3N0ni0ZCT+tbNP5zWnD1owGWvB/Mybk9ieU76NO0mAlvBEe1hqLUIUMva
7TdsbyhFybhBBmDeVeDdBztMteTEQg5fwoZc63eExb0MrU0f1bW/ghEJVXV1Zftu
Q3GLVoNiUYlGhC8JxYo3lyAQdHvtIGJa374FTFqmdcVc1ZPAtF/gbJjPqXHJI7De
VsxLYQKBgQCpfVM1Xel6cxVaip8y2LeoDiOUCttCN3vJYpp7xAus26vME+m4lWTs
WCjOrCQLuoJhxkIpTnyLqbnNQL5dsZp637t9GiT+iq2leDeVf+eQG53t/cwFZBMS
adnwMpVVRhpNoK3np2xY2TyH+SLMQlwKK5MpztoNzMHBob3rzv/5cQKBgQCl2cvJ
V82ftXyw1+kNmq63XvgJUIdff634al6aCUujKZmzliZCN+C0ctCwRgBlqW89PIwq
b3HEGY4H60o3Ke2hcbyZJTeDxydZWzN6NC3JU4gGhgeP7K5W1RLXLxZKOdL0VRh4
lmgYnOfvcLwIWEZRRRztSap9Lde54ARMc+o9XwKBgBhfPSffaUOCaP+sD1hwcXhp
EBSpuv26nMJIu44wkr04mWuvMVypnumdg8C0YDqPh1AoAjuOXbp3nZA2TymJ8Hza
z0seB1PS/UVeFCAbVGEUl5ExfbUIvvVW40/29iJdAktHX7qACMR9+IZU/PwCTtnn
ijHo3NI/L114tKbEDWsBAoGBAIzWf9SSCF54etOd9h2EXpv/PLiENwEk5rJOedlq
zV1YRVqYxhJzaxExcwBN4aZzFLC2yvx6OOzjGpak+xgPNelkMVkHnrX8F/EsnnFH
AB64HYoUpTVWMtIwDNjI9q+/nOG5pZc5elp0XA0b+cFIXSZEf4UNiobUuB3zGxNl
V+QXAoGAI/S5xJnhc40lldgOVYNMNaShL42YwjJO1/DnrZoyqjgOkjvxuyRQybsk
YXb4PV1oH9FnT10+Pj7JYOCgncZtDNM9vG5d5AqP2hdk0vCTT+ddhfxysH+HMiXD
JYaT5H2Aw1BOaoQXBIvCJch7xyaSYHwEEMPhFsaiiGaZqIItcFo=
-----END RSA PRIVATE KEY-----";

/*
-----BEGIN PUBLIC KEY-----
MIIBITANBgkqhkiG9w0BAQEFAAOCAQ4AMIIBCQKCAQBtzfjInEd3k5w5K2/bZ8Qy
C+2brq3E64/Zjl8oTLP8n75Ub1W7CGTdpyXbwR1EstHex5SaXrfHNHNSoBabqSIN
HWcFzGTxMqTV1tHAgXi75a9WPST2qg/LeZZ4uqL7mgRtvu8Hur0kdv3phoPZLNGo
kPvzEGijb6XhvLSU0/H4i9l7MWcT2FTvwBuKxWSxaeICH2QckBrMFoyrWtCNwJUI
MOWJcI/bFQfgwOVIyX+gz8i3dWcjwCfsizo3AEtyPa953CjFDwqAs4Xtz0HMQugu
sLKOjwuFC4l3l4+l3PZMOUqZKFTSbEMnlsg2Z9yw1AigeDLqr+hUlEySr2iTt33v
AgMBAAE=
-----END PUBLIC KEY-----
*/

static PEM_PUBLICK_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIIBITANBgkqhkiG9w0BAQEFAAOCAQ4AMIIBCQKCAQBtzfjInEd3k5w5K2/bZ8Qy
C+2brq3E64/Zjl8oTLP8n75Ub1W7CGTdpyXbwR1EstHex5SaXrfHNHNSoBabqSIN
HWcFzGTxMqTV1tHAgXi75a9WPST2qg/LeZZ4uqL7mgRtvu8Hur0kdv3phoPZLNGo
kPvzEGijb6XhvLSU0/H4i9l7MWcT2FTvwBuKxWSxaeICH2QckBrMFoyrWtCNwJUI
MOWJcI/bFQfgwOVIyX+gz8i3dWcjwCfsizo3AEtyPa953CjFDwqAs4Xtz0HMQugu
sLKOjwuFC4l3l4+l3PZMOUqZKFTSbEMnlsg2Z9yw1AigeDLqr+hUlEySr2iTt33v
AgMBAAE=
-----END PUBLIC KEY-----";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn encrypt<S>(plaintext: S) -> Result<String>
where
  S: AsRef<[u8]>,
{
  let public_key = RsaPublicKey::from_public_key_pem(PEM_PUBLICK_KEY)?;

  let mut rng = rand::thread_rng();
  let padding = Oaep::new::<sha2::Sha256>();

  let enc_data = public_key.encrypt(&mut rng, padding, plaintext.as_ref())?;

  Ok(general_purpose::STANDARD_NO_PAD.encode(enc_data))
}

pub fn decrypt<S>(ciphertext: S) -> Result<String>
where
  S: AsRef<[u8]>,
{
  let private_key = RsaPrivateKey::from_pkcs1_pem(PEM_PRIVATE_KEY)?;
  let enc_data = general_purpose::STANDARD_NO_PAD.decode(ciphertext.as_ref())?;

  let padding = Oaep::new::<sha2::Sha256>();
  let dec_data = private_key.decrypt(padding, enc_data.as_slice())?;

  String::from_utf8(dec_data).map_err(Into::into)
}

#[cfg(test)]
mod tests {
  use super::{decrypt, encrypt};

  #[test]
  pub fn enc_and_dec() {
    let plaintext = "abc";

    let enc_str = encrypt(plaintext);

    assert!(enc_str.is_ok());

    let ciphertext = enc_str.unwrap();

    println!("{ciphertext}");

    let dec_str = decrypt(ciphertext);

    assert!(dec_str.is_ok());

    assert_eq!(plaintext, dec_str.unwrap());
  }
}
