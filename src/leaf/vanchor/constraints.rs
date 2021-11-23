use super::{Private, Public};
use crate::Vec;
use ark_crypto_primitives::{crh::CRHGadget, CRH};
use ark_ff::fields::PrimeField;
use ark_r1cs_std::{fields::fp::FpVar, prelude::*};
use ark_relations::r1cs::{Namespace, SynthesisError};
use ark_std::marker::PhantomData;
use core::borrow::Borrow;

#[derive(Clone)]
pub struct PrivateVar<F: PrimeField> {
	pub amount: FpVar<F>,
	blinding: FpVar<F>,
}

#[derive(Clone)]
pub struct PublicVar<F: PrimeField> {
	chain_id: FpVar<F>,
}

impl<F: PrimeField> PublicVar<F> {
	pub fn default() -> Self {
		let chain_id = F::zero();

		Self {
			chain_id: FpVar::Constant(chain_id),
		}
	}

	pub fn new(chain_id: FpVar<F>) -> Self {
		Self { chain_id }
	}
}

impl<F: PrimeField> PrivateVar<F> {
	pub fn new(amount: FpVar<F>, blinding: FpVar<F>) -> Self {
		Self { amount, blinding }
	}
}

pub struct VAnchorLeafGadget<
	F: PrimeField,
	H4: CRH,
	HG4: CRHGadget<H4, F>,
	H5: CRH,
	HG5: CRHGadget<H5, F>,
> {
	field: PhantomData<F>,
	hasher4: PhantomData<H4>,
	hasher_gadget4: PhantomData<HG4>,
	hasher5: PhantomData<H5>,
	hasher_gadget5: PhantomData<HG5>,
}

impl<F: PrimeField, H4: CRH, HG4: CRHGadget<H4, F>, H5: CRH, HG5: CRHGadget<H5, F>>
	VAnchorLeafGadget<F, H4, HG4, H5, HG5>
{
	pub fn create_leaf<BG: ToBytesGadget<F>>(
		private: &PrivateVar<F>,
		public_key: &BG,
		public: &PublicVar<F>,
		h_w5: &HG5::ParametersVar,
	) -> Result<HG5::OutputVar, SynthesisError> {
		let pubkey = public_key;

		let mut bytes = Vec::new();
		bytes.extend(public.chain_id.to_bytes()?);
		bytes.extend(private.amount.to_bytes()?);
		bytes.extend(pubkey.to_bytes()?);
		bytes.extend(private.blinding.to_bytes()?);
		HG5::evaluate(h_w5, &bytes)
	}

	pub fn create_signature<BG: ToBytesGadget<F>>(
		private_key: &BG,
		commitment: &HG5::OutputVar,
		index: &FpVar<F>,
		h_w4: &HG4::ParametersVar,
	) -> Result<HG4::OutputVar, SynthesisError> {
		let mut bytes = Vec::new();
		bytes.extend(private_key.to_bytes()?);
		bytes.extend(commitment.to_bytes()?);
		bytes.extend(index.to_bytes()?);
		HG4::evaluate(h_w4, &bytes)
	}
	
	pub fn create_nullifier<BG: ToBytesGadget<F>>(
		signature: &BG,
		commitment: &HG5::OutputVar,
		h_w4: &HG4::ParametersVar,
		index: &FpVar<F>,
	) -> Result<HG4::OutputVar, SynthesisError> {
		let mut bytes = Vec::new();
		bytes.extend(commitment.to_bytes()?);
		bytes.extend(index.to_bytes()?);
		bytes.extend(signature.to_bytes()?);
		HG4::evaluate(h_w4, &bytes)
	}

}

impl<F: PrimeField> AllocVar<Private<F>, F> for PrivateVar<F> {
	fn new_variable<T: Borrow<Private<F>>>(
		into_ns: impl Into<Namespace<F>>,
		f: impl FnOnce() -> Result<T, SynthesisError>,
		mode: AllocationMode,
	) -> Result<Self, SynthesisError> {
		let secrets = f()?.borrow().clone();
		let ns = into_ns.into();
		let cs = ns.cs();

		let amount = secrets.amount;
		let blinding = secrets.blinding;

		let amount_var = FpVar::new_variable(cs.clone(), || Ok(amount), mode)?;
		let blinding_var = FpVar::new_variable(cs.clone(), || Ok(blinding), mode)?;
		Ok(PrivateVar::new(amount_var, blinding_var))
	}
}

impl<F: PrimeField> AllocVar<Public<F>, F> for PublicVar<F> {
	fn new_variable<T: Borrow<Public<F>>>(
		into_ns: impl Into<Namespace<F>>,
		f: impl FnOnce() -> Result<T, SynthesisError>,
		mode: AllocationMode,
	) -> Result<Self, SynthesisError> {
		let public = f()?.borrow().clone();
		let ns = into_ns.into();
		let cs = ns.cs();
		let chain_id = FpVar::new_variable(cs.clone(), || Ok(public.chain_id), mode)?;
		Ok(PublicVar::new(chain_id))
	}
}

#[cfg(feature = "default_poseidon")]
#[cfg(test)]
mod test {
	use super::*;
	use crate::{leaf::vanchor::VAnchorLeaf, poseidon::{
			constraints::{CRHGadget, PoseidonParametersVar},
			sbox::PoseidonSbox,
			PoseidonParameters, Rounds, CRH,
		}, utils::{
			get_mds_poseidon_bn254_x5_2, get_mds_poseidon_bn254_x5_4,
			get_mds_poseidon_bn254_x5_5, get_rounds_poseidon_bn254_x5_2, get_rounds_poseidon_bn254_x5_4,
			get_rounds_poseidon_bn254_x5_5,
		}};
	use ark_bn254::Fq;

	use ark_crypto_primitives::crh::{CRHGadget as CRHGadgetTrait, CRH as CRHTrait};
	use ark_ff::to_bytes;
	use ark_relations::r1cs::ConstraintSystem;
	use ark_std::test_rng;

	#[derive(Default, Clone)]
	struct PoseidonRounds2;

	impl Rounds for PoseidonRounds2 {
		const FULL_ROUNDS: usize = 8;
		const PARTIAL_ROUNDS: usize = 56;
		const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(5);
		const WIDTH: usize = 2;
	}

	#[derive(Default, Clone)]
	struct PoseidonRounds4;

	impl Rounds for PoseidonRounds4 {
		const FULL_ROUNDS: usize = 8;
		const PARTIAL_ROUNDS: usize = 56;
		const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(5);
		const WIDTH: usize = 4;
	}

	#[derive(Default, Clone)]
	struct PoseidonRounds5;

	impl Rounds for PoseidonRounds5 {
		const FULL_ROUNDS: usize = 8;
		const PARTIAL_ROUNDS: usize = 60;
		const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(5);
		const WIDTH: usize = 5;
	}

	type PoseidonCRH2 = CRH<Fq, PoseidonRounds2>;
	type PoseidonCRH4 = CRH<Fq, PoseidonRounds4>;
	type PoseidonCRH5 = CRH<Fq, PoseidonRounds5>;

	type PoseidonCRH2Gadget = CRHGadget<Fq, PoseidonRounds2>;
	type PoseidonCRH4Gadget = CRHGadget<Fq, PoseidonRounds4>;
	type PoseidonCRH5Gadget = CRHGadget<Fq, PoseidonRounds5>;

	type Leaf = VAnchorLeaf<Fq, PoseidonCRH4, PoseidonCRH5>;
	type LeafGadget =
		VAnchorLeafGadget<Fq, PoseidonCRH4, PoseidonCRH4Gadget, PoseidonCRH5, PoseidonCRH5Gadget>;
	use crate::ark_std::{One, UniformRand};
	#[test]
	fn should_crate_new_leaf_constraints() {
		let rng = &mut test_rng();
		let cs = ConstraintSystem::<Fq>::new_ref();

		// Native version
		let rounds5_5 = get_rounds_poseidon_bn254_x5_5::<Fq>();
		let mds5_5 = get_mds_poseidon_bn254_x5_5::<Fq>();
		let params5_5 = PoseidonParameters::<Fq>::new(rounds5_5, mds5_5);
		let rounds5_2 = get_rounds_poseidon_bn254_x5_2::<Fq>();
		let mds5_2 = get_mds_poseidon_bn254_x5_2::<Fq>();
		let params5_2 = PoseidonParameters::<Fq>::new(rounds5_2, mds5_2);

		let chain_id = Fq::one();
		let index = Fq::one();
		let public = Public::new(chain_id);
		let secrets = Private::generate(rng);
		let private_key = Fq::rand(rng);
		let privkey = to_bytes![private_key].unwrap();
		let public_key = PoseidonCRH2::evaluate(&params5_2, &privkey).unwrap();
		let leaf = Leaf::create_leaf(&secrets, &public_key, &public, &params5_5).unwrap();

		// Constraints version
		let index_var = FpVar::<Fq>::new_witness(cs.clone(), || Ok(index)).unwrap();
		let public_var = PublicVar::new_input(cs.clone(), || Ok(&public)).unwrap();
		let secrets_var = PrivateVar::new_witness(cs.clone(), || Ok(&secrets)).unwrap();
		let private_key_var = FpVar::<Fq>::new_witness(cs.clone(), || Ok(&private_key)).unwrap();
		let params_var5_5 = PoseidonParametersVar::new_variable(
			cs.clone(),
			|| Ok(&params5_5),
			AllocationMode::Constant,
		)
		.unwrap();
		let params_var5_2 = PoseidonParametersVar::new_variable(
			cs.clone(),
			|| Ok(&params5_2),
			AllocationMode::Constant,
		)
		.unwrap();

		let mut bytes = Vec::new();
		bytes.extend(private_key_var.to_bytes().unwrap());
		let public_key_var = PoseidonCRH2Gadget::evaluate(&params_var5_2, &bytes).unwrap();

		let leaf_var =
			LeafGadget::create_leaf(&secrets_var, &public_key_var, &public_var, &params_var5_5)
				.unwrap();

		// Check equality
		let leaf_new_var = FpVar::<Fq>::new_witness(cs.clone(), || Ok(leaf)).unwrap();
		let res = leaf_var.is_eq(&leaf_new_var).unwrap();
		assert!(res.value().unwrap());
		assert!(res.cs().is_satisfied().unwrap());

		// Test Signature
		// Native version

		let rounds5_4 = get_rounds_poseidon_bn254_x5_4::<Fq>();
		let mds5_4 = get_mds_poseidon_bn254_x5_4::<Fq>();
		let params5_4 = PoseidonParameters::<Fq>::new(rounds5_4, mds5_4);
		let params_var5_4 = PoseidonParametersVar::new_variable(
			cs.clone(),
			|| Ok(&params5_4),
			AllocationMode::Constant,
		)
		.unwrap();

		let commitment = leaf;
		let inputs_signature = to_bytes![private_key, commitment, index].unwrap();
		let ev_res = PoseidonCRH4::evaluate(&params5_4, &inputs_signature).unwrap();
		let signature =
			Leaf::create_signature(&private_key, &commitment, &index, &params5_4).unwrap();
		assert_eq!(ev_res, signature);
		
		// Constraints version
		let commitment_var = leaf_var.clone();
		let signature_var = LeafGadget::create_signature(
			&private_key_var,
			&commitment_var,
			&index_var,
			&params_var5_4,
		)
		.unwrap();

		let signature_new_var =
			FpVar::<Fq>::new_witness(signature_var.cs(), || Ok(signature)).unwrap();
		let res_sig = signature_var.is_eq(&signature_new_var).unwrap();
		assert!(res_sig.value().unwrap());
		assert!(res_sig.cs().is_satisfied().unwrap());

		// Test Nullifier
		// Native version

		let nullifier =
			Leaf::create_nullifier(&signature, &commitment, &params5_4, &index).unwrap();

		// Constraints version
		let nullifier_var = LeafGadget::create_nullifier(
			&signature_var,
			&commitment_var,
			&params_var5_4,
			&index_var,
		)
		.unwrap();

		// Check equality
		let nullifier_new_var =
			FpVar::<Fq>::new_witness(nullifier_var.cs(), || Ok(nullifier)).unwrap();
		let res_nul = nullifier_var.is_eq(&nullifier_new_var).unwrap();
		assert!(res_nul.value().unwrap());
		assert!(res_nul.cs().is_satisfied().unwrap());
	}

}
