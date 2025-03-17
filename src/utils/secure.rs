use hmac::{ Hmac, Mac };
use sha2::Sha256;
use hex;

pub fn generate_hash(
    token: &str,
    timestamp: u64,
    node_id: &str,
    is_replica: bool,
    tags: &[String]
) -> String {
    let mut mac = Hmac::<Sha256>
        ::new_from_slice(token.as_bytes())
        .expect("HMAC can take key of any size");
 
    let data = format!("{}|{}|{}|{}", timestamp, node_id, is_replica, tags.join(","));

    mac.update(data.as_bytes());

    return hex::encode(mac.finalize().into_bytes());
}
