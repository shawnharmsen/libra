//# init --validators Alice Bob Carol Dave
// Module to test bulk validator updates function in DiemSystem.move

// Test to check the current validator list . Then trigger update to the list 
// of validators, then re-run it. 
// This test is run with the function passing in the wrong current block on purpose.
// This avoids an error when a reconfig function happens before the first epoch 
// is completed.

//# run --admin-script --signers DiemRoot DiemRoot
script {
    use DiemFramework::DiemSystem;
    use Std::Vector;
    use DiemFramework::ValidatorUniverse;

    fun main(vm: signer, _: signer) {
        // Tests on initial size of validators 
        assert!(DiemSystem::validator_set_size() == 4, 73570080010001);
        assert!(DiemSystem::is_validator(@Alice), 73570080010002);
        assert!(DiemSystem::is_validator(@Bob), 73570080010003);
        assert!(DiemSystem::is_validator(@Carol), 73570080010004);
        assert!(DiemSystem::is_validator(@Dave), 73570080010005);

        let old_vec = ValidatorUniverse::get_eligible_validators();
        assert!(Vector::length<address>(&old_vec) == 4, 73570080010006);
        
        //Create vector of validators and func call
        let vec = Vector::empty();
        Vector::push_back<address>(&mut vec, @Alice);
        Vector::push_back<address>(&mut vec, @Bob);
        Vector::push_back<address>(&mut vec, @Carol);
        assert!(Vector::length<address>(&vec) == 3, 73570080010007);

        DiemSystem::bulk_update_validators(&vm, vec);

        // Check if updates are done
        assert!(DiemSystem::validator_set_size() == 3, 73570080010008);
        assert!(DiemSystem::is_validator(@Alice), 73570080010009);
        assert!(DiemSystem::is_validator(@Bob), 73570080010010);
        assert!(DiemSystem::is_validator(@Carol), 73570080010011);
        assert!(DiemSystem::is_validator(@Dave) == false, 73570080010012);
    }
}
// check: EXECUTED