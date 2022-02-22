// use crate::helper_types::overarching_call::OverarchingCallRealType;
// use crate::mock::OverarchingCallRealType;
// use codec::WrapperTypeEncode;
// use core::ops::Deref;
// use sp_runtime::traits::Extrinsic as ExtrinsicT;
//
// #[derive(parity_util_mem_derive::MallocSizeOf)]
// pub struct ExtrinsicRealType;
//
// impl ExtrinsicT for ExtrinsicRealType {
// 	type Call = OverarchingCallRealType;
// 	type SignaturePayload = ();
// }
//
// impl Deref for ExtrinsicRealType {
// 	type Target = ();
//
// 	fn deref(&self) -> &Self::Target {
// 		&()
// 	}
// }
//
// impl WrapperTypeEncode for ExtrinsicRealType {}
