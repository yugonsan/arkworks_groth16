use ark_bn254::{Bn254, Fr};
use ark_ff::Field;
use ark_groth16::{generate_random_parameters, prepare_verifying_key, create_random_proof, verify_proof};
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable},
};
use ark_std::rand::{rngs::StdRng, SeedableRng};
use ark_serialize::CanonicalSerialize;
use std::env;


/* define test circuit
a and b are privte inouts that need to equal the public input c.
*/
pub struct Circuit<F: Field> {
    pub a: Option<F>,
    pub b: Option<F>,
    pub c: Option<F>,
}

impl<F: Field> ConstraintSynthesizer<F> for Circuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let a = cs.new_witness_variable(|| self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let b = cs.new_witness_variable(|| self.b.ok_or(SynthesisError::AssignmentMissing))?;
        let c = cs.new_input_variable(|| self.c.ok_or(SynthesisError::AssignmentMissing))?;

        cs.enforce_constraint(lc!() + a + b, lc!() + Variable::One, lc!() + c)?;

        Ok(())
    }
}
/*
Groth16
*/
// fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 4 {
        println!("Usage: {} a b c", args[0]);
        return Ok(());
    }

    let a: Fr = args[1].parse().expect("Failed to parse a");
    let b: Fr = args[2].parse().expect("Failed to parse b");
    let c: Fr = args[3].parse().expect("Failed to parse c");

    let rng = &mut StdRng::seed_from_u64(0u64);

    let pk = {
        let c = Circuit::<Fr> {
            a: None,
            b: None,
            c: None,
        };
        generate_random_parameters::<Bn254, _, _>(c, rng).unwrap()
    };

    let assignment = Circuit {
        a: Some(a),
        b: Some(b),
        c: Some(c),
    };

    let public_input = &[assignment.c.unwrap()];

    let proof = create_random_proof(assignment, &pk, rng).unwrap();

    let mut proof_vec = Vec::new();
    proof.serialize(&mut proof_vec).unwrap();

    let vk = prepare_verifying_key(&pk.vk);

    let result = verify_proof(&vk, &proof, public_input).unwrap();
    println!("verify proof: {:?}", result);

    Ok(())
}