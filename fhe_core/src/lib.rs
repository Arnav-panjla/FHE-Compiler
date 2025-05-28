use rand;
// const Q: i64 = 4096;
// const P: i64 = 17;

// implemneting a very trivial FHE scheme
const T: i64 = 100007; // 5 digit prime

pub struct PublicKey {
    pub pk: i64,
}

pub struct SecretKey {
    pub sk: i64,
}

pub fn keygen() -> (PublicKey, SecretKey) {

    // let sk = rand::random_range(100..200); 
    // (PublicKey { pk: sk }, SecretKey { sk })
    
    (PublicKey { pk: T }, SecretKey { sk: T })
}

pub fn encrypt(m: i64, _sk: &SecretKey) -> i64 {
    let r = rand::random_range(1..10);
    //(m + sk.sk * r) % Q
    m + r * T
}

pub fn decrypt(c: &i64, _sk: &SecretKey) -> i64 {
    // (c % sk.sk) % P
    c % T
}

pub fn homomorphic_add(c1: &i64, c2: &i64) -> i64 {
    // (c1 + c2) % Q
    c1 + c2
}

pub fn homomorphic_mul(c1: &i64, c2: &i64) -> i64 {
    // (c1 * c2) % Q
    c1 * c2
}

