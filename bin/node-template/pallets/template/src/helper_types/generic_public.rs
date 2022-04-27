use crate::helper_types::public::PublicRealType;
use crate::helper_types::runtime_app_public::RuntimeAppPublicRealType;

pub struct GenericPublicRealType;

impl From<RuntimeAppPublicRealType> for GenericPublicRealType {
	fn from(_: RuntimeAppPublicRealType) -> Self {
		Self
	}
}

impl From<PublicRealType> for GenericPublicRealType {
	fn from(_: PublicRealType) -> Self {
		Self
	}
}
