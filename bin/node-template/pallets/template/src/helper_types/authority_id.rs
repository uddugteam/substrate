use crate::helper_types::generic_public::GenericPublicRealType;
use crate::helper_types::public::PublicRealType;
use crate::helper_types::runtime_app_public::RuntimeAppPublicRealType;
use frame_system::offchain::AppCrypto;

pub struct AuthorityIdRealType;

impl AppCrypto<PublicRealType, ()> for AuthorityIdRealType {
	type RuntimeAppPublic = RuntimeAppPublicRealType;
	type GenericPublic = GenericPublicRealType;
	type GenericSignature = ();
}
