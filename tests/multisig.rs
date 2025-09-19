use tokio;


#[tokio::test]
async fn test_complete_bank_workflow() {
    let program = setup_program().await;
    let system = CombinedBankSystem::new(program);

    let setup = system.setup_system().await.unwrap();
    let bank = system.register_bank_through_multisig(
        setup.admin_multisig,
        setup.admin_multisig_signer,
        setup.gateway,
        "BANK001",
        kyc_authority,
        treasury_vault,
        admin_owners,
    ).await.unwrap();

    let sender_kyc = system.add_kyc_entity(bank, "SENDER001", kyc_authority).await.unwrap();
    let recipient_kyc = system.add_kyc_entity(bank, "RECIPIENT001", kyc_authority).await.unwrap();

    let transfer = system.create_transfer_request(
        setup.gateway, bank, sender_kyc, recipient_kyc,
        "TXN001", 150_000, bank
    ).await.unwrap();

    system.approve_large_transfer(
        setup.compliance_multisig,
        setup.compliance_multisig_signer,
        transfer,
        compliance_owners,
    ).await.unwrap();

    system.execute_transfer(
        transfer, bank, sender_kyc, recipient_kyc,
        sender_vault, recipient_vault
    ).await.unwrap();

    println!("✅ Complete workflow successful!");
}
