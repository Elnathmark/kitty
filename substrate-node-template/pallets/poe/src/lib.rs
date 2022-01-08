#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	use sp_std::if_std;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		
		#[pallet::constant]
		type MaxLength: Get<u32>;

		#[pallet::constant]
		type MinLength: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        /// Event emitted when a proof has been claimed. [who, claim]
        ClaimCreated(T::AccountId, Vec<u8>),
        /// Event emitted when a claim is revoked by the owner. [who, claim]
        ClaimRevoked(T::AccountId, Vec<u8>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		    /// The proof has already been claimed.
			ProofAlreadyClaimed,
			/// The proof does not exist, so it cannot be revoked.
			NoSuchProof,
			/// The proof is claimed by another account, so caller can't revoke it.
			NotProofOwner,
			//存证过长
			TooLong,
			//存证过短
			TooShort,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

    #[pallet::weight(0)]
    pub fn create_claim(
        origin: OriginFor<T>,
        proof: Vec<u8>,
    ) -> DispatchResult {
		if_std! {
			println!("MinLength:{:#?}",T::MinLength::get());
			println!("MaxLength:{:#?}",T::MaxLength::get())
		}
		//判断存证是否符合长度限制
		ensure!(proof.len() >= T::MinLength::get() as usize, Error::<T>::TooShort);
		ensure!(proof.len() <= T::MaxLength::get() as usize, Error::<T>::TooLong);

        // Check that the extrinsic was signed and get the signer.
        // This function will return an error if the extrinsic is not signed.
        // https://docs.substrate.io/v3/runtime/origins
        let sender = ensure_signed(origin)?;

        // Verify that the specified proof has not already been claimed.
        ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

        // Get the block number from the FRAME System pallet.
        let current_block = <frame_system::Pallet<T>>::block_number();

        // Store the proof with the sender and block number.
        Proofs::<T>::insert(&proof, (&sender, current_block));

        // Emit an event that the claim was created.
        Self::deposit_event(Event::ClaimCreated(sender, proof));

        Ok(())
    }

    #[pallet::weight(0)]
    pub fn revoke_claim(
        origin: OriginFor<T>,
        proof: Vec<u8>,
    ) -> DispatchResult {
		//判断存证是否符合长度限制
		ensure!(proof.len() >= T::MinLength::get() as usize, Error::<T>::TooShort);
		ensure!(proof.len() <= T::MaxLength::get() as usize, Error::<T>::TooLong);
        // Check that the extrinsic was signed and get the signer.
        // This function will return an error if the extrinsic is not signed.
        // https://docs.substrate.io/v3/runtime/origins
        let sender = ensure_signed(origin)?;
        // Verify that the specified proof has been claimed.
        ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

        // Get owner of the claim.
        let (owner, _) = Proofs::<T>::get(&proof);

        // Verify that sender of the current call is the claim owner.
        ensure!(sender == owner, Error::<T>::NotProofOwner);

        // Remove claim from storage.
        Proofs::<T>::remove(&proof);

        // Emit an event that the claim was erased.
        Self::deposit_event(Event::ClaimRevoked(sender, proof));
        Ok(())
    }

	#[pallet::weight(0)]
	pub fn transfer_claim(origin: OriginFor<T>, proof: Vec<u8>, dest: T::AccountId) -> DispatchResult {
		//判断存证是否符合长度限制
		ensure!(proof.len() >= T::MinLength::get() as usize, Error::<T>::TooShort);
		ensure!(proof.len() <= T::MaxLength::get() as usize, Error::<T>::TooLong);

		let sender = ensure_signed(origin)?;
		let (owner, block_number) = Proofs::<T>::get(&proof);
		
		ensure!(sender == owner, Error::<T>::NotProofOwner); //确实是否存证的拥有者
		Proofs::<T>::insert(&proof, (dest, block_number)); //更新存证拥有者信息
		Ok(().into())
	}

	}
}
