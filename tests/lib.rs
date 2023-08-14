use scrypto::prelude::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

// #[test]
// fn test_hello() {
//     // Setup the environment
//     let mut test_runner = TestRunner::builder().build();

//     // Create an account
//     let (public_key, _private_key, account) = test_runner.new_allocated_account();

//     // Publish package
//     let package_address = test_runner.compile_and_publish(this_package!());

//     // Test the `instantiate_hello` function.
//     let manifest = ManifestBuilder::new()
//         .call_function(
//             package_address,
//             "Hello",
//             "instantiate_hello",
//             manifest_args!(),
//         )
//         .build();
//     let receipt = test_runner.execute_manifest_ignoring_fee(
//         manifest,
//         vec![NonFungibleGlobalId::from_public_key(&public_key)],
//     );
//     println!("{:?}\n", receipt);
//     let component = receipt.expect_commit(true).new_component_addresses()[0];

//     // Test the `free_token` method.
//     let manifest = ManifestBuilder::new()
//         .call_method(component, "free_token", manifest_args!())
//         .call_method(
//             account,
//             "deposit_batch",
//             manifest_args!(ManifestExpression::EntireWorktop),
//         )
//         .build();
//     let receipt = test_runner.execute_manifest_ignoring_fee(
//         manifest,
//         vec![NonFungibleGlobalId::from_public_key(&public_key)],
//     );
//     println!("{:?}\n", receipt);
//     receipt.expect_commit_success();
// }

#[test]
fn manifest_creator() {
    let mut test_runner = TestRunner::builder().build();

    // Create an account
    let (public_key, _private_key, account) = test_runner.new_allocated_account();

    // Publish package
    let package_address = test_runner.compile_and_publish(this_package!());

    // Test the `instantiate_hello` function.
    let raw_manifest = ManifestBuilder::new()
        .call_function(
            package_address,
            "Member",
            "instantiate_member",
            manifest_args!(),
        );

        let manifest = raw_manifest
        .build();

        let manifest_copy = manifest.clone();
    let receipt = test_runner.execute_manifest_ignoring_fee(
        manifest,
        vec![NonFungibleGlobalId::from_public_key(&public_key)],
    );

    let component = receipt.expect_commit(true).new_component_addresses()[0];
    println!("{:?}\n", component);

    dump_manifest_to_file_system( 
        &manifest_copy, 
        raw_manifest.object_names(), 
        "./transaction-manifest", 
        Some("sample_dump"),
        &NetworkDefinition::simulator()
    ).err(); 

    // let manifest = ManifestBuilder::new() 
    // .call_method( 
    //     Address("component_tdx_d_1cqdjyq9fyh3fk4ldqkj58g7wghqew476rd7qfvsxnrc4un4fktd7p5"),
    //     "mint_member_card",
    //     manifest_args!()
    // )
    // .build(); 
}
