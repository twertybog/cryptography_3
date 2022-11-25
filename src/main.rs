#![allow(dead_code)]

use ec_algebra::ECPoint;
use rand::{thread_rng, Rng};

const P: i128 = 7722442867629163;

const A: i128 = 7722442867629162;

const B: i128 = 3575470469711650;

const G: ECPoint = ECPoint{
    x: 326616277527371,
    y: 5932538299212952
};

fn main() {    
    let (alice_sec, alice_pub) = alice();
    println!("alice secret {}\nalice_pub: {}", alice_sec, ECPoint::ECPointToString(alice_pub));

    let (bob_sec, bob_pub) = bob();
    println!("bob secret: {}\nbob pub: {}", bob_sec, ECPoint::ECPointToString(bob_pub));
   
    println!("Shared secret: {:?}", ECPoint::ScalarMult(bob_pub, alice_sec, A));

    println!("Shared secret equal: {}", 
    ECPoint::ScalarMult(bob_pub, alice_sec, A) == ECPoint::ScalarMult(alice_pub, bob_sec, A))
}
fn alice() -> (i128, ECPoint){
    let secret = generate_secret_value();
    let mut h_a = ECPoint::ScalarMult(G, secret, A);
    h_a = ECPoint{x: h_a.x%P, y: h_a.y%P};
    (secret, h_a)
}
fn bob() -> (i128, ECPoint){
    let secret = generate_secret_value();
    let mut h_b = ECPoint::ScalarMult(G, secret, A);
    h_b = ECPoint{x: h_b.x%P, y: h_b.y%P};
    (secret, h_b)
}

fn generate_secret_value() -> i128{
    let mut rng = thread_rng();
    rng.gen_range(2_i128.pow(0)..2_i128.pow(24))
}
