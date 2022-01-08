use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use frame_system::pallet_prelude::*;
// use super::*;

// ------------ 测试创建存证 --------------

//创建存证的单元测试用例，并查询存证的所有者是否一致
#[test]
fn create_proof_test(){
	new_test_ext().execute_with(||{
		// md5 32位
		// <Test as pallet_poe::Config>::MaxLength::get();
		// let a = <Test as pallet_poe::Config>::MaxLength::get();
		let proof_text = "126a1ea53f9963837532eb5fa0efa5df";
		assert_ok!(PoeModule::create_claim(Origin::signed(1), String::from(proof_text).into_bytes()));
		let sender = ensure_signed(Origin::signed(1)).unwrap();
		assert_eq!(PoeModule::something(String::from(proof_text).into_bytes()).0, sender);
	});
}

/*
重复创建两次,是否会出现Error::<T>::ProofAlreadyClaimed
**/
#[test]
fn create_same_proof_two_times(){
	new_test_ext().execute_with(||{
		let proof_text = "126a1ea53f9963837532eb5fa0efa6df";
		assert_ok!(PoeModule::create_claim(Origin::signed(1), String::from(proof_text).into_bytes()));
		assert_noop!(PoeModule::create_claim(Origin::signed(1), String::from(proof_text).into_bytes()), Error::<Test>::ProofAlreadyClaimed);
	});
}

/**
 * 测试存证过短的场景
 */
#[test]
fn create_short_proof(){
	new_test_ext().execute_with(||{
		let proof_text = "126a1ea";
		assert_noop!(PoeModule::create_claim(Origin::signed(1), String::from(proof_text).into_bytes()), Error::<Test>::TooShort);
	});
}

/**
 * 测试存证过短的场景
 */
#[test]
fn create_long_proof(){
	new_test_ext().execute_with(||{
		let proof_text = "126a1ea53f9963837532eb5fa0efa6df12";
		assert_noop!(PoeModule::create_claim(Origin::signed(1), String::from(proof_text).into_bytes()), Error::<Test>::TooLong);
	});
}

// ----------------- 删除存证测试 -----------

/**
 * 测试删除长度过短的存证
 */
#[test]
fn remove_claim_with_short_proof(){
	new_test_ext().execute_with(||{
		let proof_text = "126a1ea2";
		assert_noop!(PoeModule::revoke_claim(Origin::signed(1), String::from(proof_text).into_bytes()), Error::<Test>::TooShort);
	});
}

/**
 * 测试删除长度过长的存证
 */
#[test]
fn remove_claim_with_long_proof(){
	new_test_ext().execute_with(||{
		let proof_text = "126a1ea53f9963837532eb5fa0efa6df123";
		assert_noop!(PoeModule::revoke_claim(Origin::signed(1), String::from(proof_text).into_bytes()), Error::<Test>::TooLong);
	});
}

/**
 * 测试非存证拥有者删除存证信息
 */
#[test]
fn remove_not_permission_proof(){
	new_test_ext().execute_with(||{
		let proof_text = "126a1ea53f9963837532eb5fa0efa6df";
		assert_ok!(PoeModule::create_claim(Origin::signed(1), String::from(proof_text).into_bytes()));
		assert_noop!(PoeModule::revoke_claim(Origin::signed(2), String::from(proof_text).into_bytes()), Error::<Test>::NotProofOwner);
	});
}

/**
 * 测试存证拥有者删除存证信息
 */
#[test]
fn remove_permission_proof(){
	new_test_ext().execute_with(||{
		let proof_text = "126a1ea53f9963837532eb5fa0efa6df";
		assert_ok!(PoeModule::create_claim(Origin::signed(1), String::from(proof_text).into_bytes()));
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), String::from(proof_text).into_bytes()));
	});
}


// --------------- 转移存证 ------------------

/**
 * 转移过短的存证
 */
#[test]
fn transfer_short_proof(){
	new_test_ext().execute_with(||{
		let proof_text = "126a1ea2";
		assert_noop!(PoeModule::transfer_claim(Origin::signed(1), String::from(proof_text).into_bytes(), 2), Error::<Test>::TooShort);
	});
}

/**
 * 转移过长的存证
 */
#[test]
fn transfer_long_proof(){
	new_test_ext().execute_with(||{
		let proof_text = "126a1ea53f9963837532eb5fa0efa6df123";
		assert_noop!(PoeModule::transfer_claim(Origin::signed(1), String::from(proof_text).into_bytes(), 2), Error::<Test>::TooLong);
	});
}

/**
 * 测试存证转移是否成功
 */
#[test]
fn transfer_to_other(){
	new_test_ext().execute_with(||{
		let proof_text = "126a1ea53f9963837532eb5fa0efa6df";
		assert_ok!(PoeModule::create_claim(Origin::signed(1), String::from(proof_text).into_bytes()));
		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), String::from(proof_text).into_bytes(), 2));
		assert_eq!(PoeModule::something(String::from(proof_text).into_bytes()).0, 2);
	});
}