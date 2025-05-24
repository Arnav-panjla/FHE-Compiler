use rand;

const Q: i64 = 4096;
const P: i64 = 17;

pub struct PublicKey {
    pub pk: i64,
}

pub struct SecretKey {
    pub sk: i64,
}

pub fn keygen() -> (PublicKey, SecretKey) {
    let sk = rand::random_range(100..200); 
    (PublicKey { pk: sk }, SecretKey { sk })
}

pub fn encrypt(m: i64, sk: &SecretKey) -> i64 {
    let r = rand::random_range(1..10);
    (m + sk.sk * r) % Q
}

pub fn decrypt(c: &i64, sk: &SecretKey) -> i64 {
    (c % sk.sk) % P
}

pub fn homomorphic_add(c1: &i64, c2: &i64) -> i64 {
    (c1 + c2) % Q
}

pub fn homomorphic_mul(c1: &i64, c2: &i64) -> i64 {
    (c1 * c2) % Q
}

