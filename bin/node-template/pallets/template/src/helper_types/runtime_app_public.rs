use crate::helper_types::generic_public::GenericPublicRealType;
use crate::Vec;
use sp_core::crypto::{CryptoType, CryptoTypeId, IsWrappedBy, KeyTypeId, Public};
use sp_runtime::{
	app_crypto::RuntimeAppPublic,
	traits::{Extrinsic as ExtrinsicT, IdentifyAccount, One},
};
pub struct RuntimeAppPublicRealType;

impl RuntimeAppPublic for RuntimeAppPublicRealType {
	const ID: KeyTypeId = KeyTypeId([0, 0, 0, 0]);
	const CRYPTO_ID: CryptoTypeId = CryptoTypeId([0, 0, 0, 0]);
	type Signature = ();

	fn all() -> Vec<Self> {
		todo!()
	}

	fn generate_pair(seed: Option<Vec<u8>>) -> Self {
		todo!()
	}

	fn sign<M: AsRef<[u8]>>(&self, msg: &M) -> Option<Self::Signature> {
		todo!()
	}

	fn verify<M: AsRef<[u8]>>(&self, msg: &M, signature: &Self::Signature) -> bool {
		todo!()
	}

	fn to_raw_vec(&self) -> Vec<u8> {
		todo!()
	}
}

impl From<GenericPublicRealType> for RuntimeAppPublicRealType {
	fn from(_: GenericPublicRealType) -> Self {
		Self
	}
}
