use ::ipfs::repo::mem::{MemBlockStore, MemDataStore, MemLock};

pub struct IpfsTypesRealType;

impl ::ipfs::RepoTypes for IpfsTypesRealType {
	/// Available types: MemBlockStore, FsBlockStore
	type TBlockStore = MemBlockStore;

	/// Available types: MemDataStore, FsDataStore, KvDataStore
	type TDataStore = MemDataStore;

	/// Available types: MemLock, FsLock
	type TLock = MemLock;
}
