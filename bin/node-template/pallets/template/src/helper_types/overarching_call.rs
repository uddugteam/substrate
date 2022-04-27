// use crate::mock::Test;
// use crate::pallet;
// use codec::WrapperTypeEncode;
// use frame_benchmarking::frame_support::sp_runtime::app_crypto::Deref;
//
// pub struct OverarchingCallRealType;
//
// impl From<pallet::Call<crate::mock::Test>> for OverarchingCallRealType {
// 	fn from(_: pallet::Call<crate::mock::Test>) -> Self {
// 		Self()
// 	}
// }
//
// impl Deref for OverarchingCallRealType {
// 	type Target = ();
//
// 	fn deref(&self) -> &Self::Target {
// 		&()
// 	}
// }
//
// impl WrapperTypeEncode for OverarchingCallRealType {}
