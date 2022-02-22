use crate::helper_types::generic_public::GenericPublicRealType;
use codec::{WrapperTypeDecode, WrapperTypeEncode};
use core::ops::Deref;
use sp_runtime::{
	app_crypto::RuntimeAppPublic,
	traits::{Extrinsic as ExtrinsicT, IdentifyAccount, One},
};

#[derive(Debug, Clone, Ord, Eq, PartialOrd, PartialEq)]
pub struct PublicRealType(u64);

impl IdentifyAccount for PublicRealType {
	type AccountId = u64;

	fn into_account(self) -> Self::AccountId {
		self.0
	}
}

impl From<u64> for PublicRealType {
	fn from(account_id: u64) -> Self {
		Self(account_id)
	}
}

impl WrapperTypeEncode for PublicRealType {}

impl WrapperTypeDecode for PublicRealType {
	type Wrapped = u64;
}

impl Deref for PublicRealType {
	type Target = ();

	fn deref(&self) -> &Self::Target {
		&()
	}
}

impl From<GenericPublicRealType> for PublicRealType {
	fn from(_: GenericPublicRealType) -> Self {
		Self(0)
	}
}
