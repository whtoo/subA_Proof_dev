#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,pallet_prelude::*
    };
    use frame_system::pallet_prelude::*;
    
    use frame_support::traits::Vec;

     // The struct on which we build all of our Pallet logic.
     #[pallet::pallet]
     #[pallet::generate_store(pub(super) trait Store)]
     pub struct Pallet<T>(_);

     /* Placeholder for defining custom types. */
     #[pallet::storage]
     #[pallet::getter(fn proofs)]
    pub type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        Vec<u8>,
        (T::AccountId, T::BlockNumber)
    >;
     // DONE: Update the `config` block below
     #[pallet::config]
     pub trait Config: frame_system::Config {
         type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

     }

     // DONE: Update the `event` block below
     #[pallet::event]
     #[pallet::generate_deposit(pub(super) fn deposit_event)]
     pub enum Event<T: Config> {
        ClaimedCreated(T::AccountId,Vec<u8>),
        ClaimedReovked(T::AccountId,Vec<u8>),
        ClaimedTransfered(T::AccountId,T::AccountId,Vec<u8>),
     }

     // DONE: Update the `error` block below
     #[pallet::error]
     pub enum Error<T> {
        ProofAlreadyClaimed,
        NoSuchProof,
        NotProofOwner
     }

     // DONE: add #[pallet::storage] block

     // DONE: Update the `call` block below
     #[pallet::call]
     impl<T: Config> Pallet<T> {
         #[pallet::weight(0)]
         pub fn create_claim(
             origin: OriginFor<T>,
             claim: Vec<u8>
         ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofAlreadyClaimed);

            let current_block = <frame_system::Pallet<T>>::block_number();

            Proofs::<T>::insert(&claim,(sender.clone(),current_block));

            Self::deposit_event(Event::ClaimedCreated(sender,claim));

            Ok(().into())
         }

         #[pallet::weight(0)]
         pub fn revoke_claim(
             origin: OriginFor<T>,
             claim: Vec<u8>
         ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has been claimed.
            ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::NoSuchProof);

            // Get owner of the claim.
            // Panic condition: there is no way to set a `None` owner, so this must always unwrap.
            let (owner, _) = Proofs::<T>::get(&claim).expect("All proofs must have an owner!");

            ensure!(owner == sender, Error::<T>::NotProofOwner);

            Proofs::<T>::remove(&claim);

            Self::deposit_event(Event::ClaimedReovked(sender,claim));

            Ok(().into())
         }

         #[pallet::weight(0)]
         pub fn transfer_claim(origin: OriginFor<T>, claim: Vec<u8>, dest: T::AccountId) -> DispatchResultWithPostInfo {
             let sender = ensure_signed(origin)?;
        
             // 检测存证文件是否存在
             ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::NoSuchProof);
 
             let (owner, _) = Proofs::<T>::get(&claim).expect("All proofs must have an owner!");
 
             ensure!(owner == sender, Error::<T>::NotProofOwner);
 
             Proofs::<T>::insert(&claim, (dest.clone(), frame_system::Module::<T>::block_number()));
             // 发送事件，声明权证转移
             Self::deposit_event(Event::ClaimedTransfered(sender,dest,claim));

             Ok(().into())
         }
     }



     #[pallet::hooks]
     impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
}

