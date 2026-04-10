use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

#[test]
fn exported_verifier_program_ids_stay_in_sync() {
    assert_eq!(
        verifier_program_id::VERIFIER_PROGRAM_ID,
        verifier_program_id::solana::VERIFIER_PROGRAM_ID
    );
}

#[test]
fn exported_verifier_program_id_matches_staging_keypair() {
    let keypair_bytes: Vec<u8> = serde_json::from_str(include_str!(
        "../../../test-keypairs/verifier-keypair.json"
    ))
    .expect("staging verifier keypair should be valid JSON");
    let keypair = Keypair::from_bytes(&keypair_bytes)
        .expect("staging verifier keypair should decode");

    assert_eq!(
        verifier_program_id::VERIFIER_PROGRAM_ID,
        keypair.pubkey().to_string()
    );
}
