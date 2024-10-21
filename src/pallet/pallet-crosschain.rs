#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn get_messages)]
    pub type Messages<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<(Vec<u8>, T::BlockNumber)>>;

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn store_message_hash(origin: OriginFor<T>, ipfs_hash: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            let current_block = <frame_system::Pallet<T>>::block_number();
            let messages = Messages::<T>::get(&sender).unwrap_or_default();

            Messages::<T>::insert(&sender, [(ipfs_hash.clone(), current_block)].to_vec());

            // Emit an event (optional)
            Self::deposit_event(Event::MessageStored(sender, ipfs_hash));
            Ok(())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MessageStored(T::AccountId, Vec<u8>),
    }
}
