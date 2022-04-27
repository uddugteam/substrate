use crate as pallet_template;
// use crate::helper_types::extrinsic::ExtrinsicRealType;
// use crate::helper_types::overarching_call::OverarchingCallRealType;
use crate::helper_types::authority_id::AuthorityIdRealType;
use crate::helper_types::public::PublicRealType;
use crate::pallet;
use codec::WrapperTypeEncode;
use core::ops::Deref;
use frame_support::parameter_types;
use frame_system as system;
use frame_system::offchain::{
	AppCrypto, CreateSignedTransaction, SendTransactionTypes, SigningTypes,
};
use parity_util_mem_derive::MallocSizeOf;
use sp_core::H256;
use sp_runtime::traits::{Extrinsic as ExtrinsicT, MaybeMallocSizeOf};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		TemplateModule: pallet_template::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

#[derive(parity_util_mem_derive::MallocSizeOf)]
pub struct ExtrinsicRealType;
impl ExtrinsicT for ExtrinsicRealType {
	type Call = OverarchingCallRealType;
	type SignaturePayload = ();
}
impl Deref for ExtrinsicRealType {
	type Target = ();

	fn deref(&self) -> &Self::Target {
		&()
	}
}
impl WrapperTypeEncode for ExtrinsicRealType {}

pub struct OverarchingCallRealType;
impl From<pallet::Call<Test>> for OverarchingCallRealType {
	fn from(_: pallet::Call<Test>) -> Self {
		Self
	}
}
impl Deref for OverarchingCallRealType {
	type Target = ();

	fn deref(&self) -> &Self::Target {
		&()
	}
}
impl WrapperTypeEncode for OverarchingCallRealType {}

impl SendTransactionTypes<pallet::Call<Self>> for Test {
	type Extrinsic = ExtrinsicRealType;
	type OverarchingCall = OverarchingCallRealType;
}

impl SigningTypes for Test {
	type Public = PublicRealType;
	type Signature = ();
}

impl CreateSignedTransaction<pallet::Call<Self>> for Test {
	fn create_transaction<C: AppCrypto<Self::Public, Self::Signature>>(
		call: Self::OverarchingCall,
		public: Self::Public,
		account: Self::AccountId,
		nonce: Self::Index,
	) -> Option<(Self::OverarchingCall, <Self::Extrinsic as ExtrinsicT>::SignaturePayload)> {
		todo!()
	}
}

pub struct CallRealType;
impl From<pallet::Call<Test>> for CallRealType {
	fn from(_: pallet::Call<Test>) -> Self {
		Self
	}
}

impl pallet_template::Config for Test {
	type AuthorityId = AuthorityIdRealType;
	type Event = Event;
	type Call = CallRealType;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
