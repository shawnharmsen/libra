//# init --validators DummyPreventsGenesisReload --parent-vasps Alice
// DummyPreventsGenesisReload: validator with 10M GAS
// Alice:                  non-validator with  1M GAS

// Alice Submit VDF Proof
//# run --admin-script --signers DiemRoot Alice
script {
    use DiemFramework::TowerState;
    use DiemFramework::TestFixtures;

    // SIMULATES A MINER ONBOARDING PROOF (proof_0.json)
    fun main(_dr: signer, sender: signer) {
        TowerState::test_helper_init_val(
            &sender,
            TestFixtures::alice_0_easy_chal(),
            TestFixtures::alice_0_easy_sol(),
            TestFixtures::easy_difficulty(),
            TestFixtures::security(),
        );

        // check for initialized TowerState
        assert!(TowerState::test_helper_get_height(@Alice) == 0, 10008001);

        // Note: test helper mocks init of a VALIDATOR, not end-user account
        assert!(TowerState::get_epochs_compliant(@Alice) == 1, 735701);
    }
}
// check: EXECUTED


//# run --admin-script --signers DiemRoot Alice
script {
    use DiemFramework::TowerState;
    use DiemFramework::TestFixtures;

    // SIMULATES THE SECOND PROOF OF THE MINER (proof_1.json)
    fun main(_dr: signer, sender: signer) {
        assert!(TowerState::test_helper_get_height(@Alice) == 0, 10008001);
        let height_after = 1;
        
        let proof = TowerState::create_proof_blob(
            TestFixtures::alice_1_easy_chal(),
            TestFixtures::alice_1_easy_sol(),
            TestFixtures::easy_difficulty(),
            TestFixtures::security(),
        );
        TowerState::commit_state(&sender, proof);

        let verified_height = TowerState::test_helper_get_height(@Alice);
        assert!(verified_height == height_after, 10008002);
    }
}
// check: EXECUTED


//# run --admin-script --signers DiemRoot DiemRoot
script {
    use DiemFramework::EpochBoundary;
    use DiemFramework::TowerState;

    // SIMULATES THE SECOND PROOF OF THE MINER (proof_1.json)
    fun main(vm: signer, _: signer) {
      EpochBoundary::reconfigure(&vm, 100);
      
      // no change from before epoch boundary
      assert!(TowerState::get_epochs_compliant(@Alice) == 1, 735701);
    }
}