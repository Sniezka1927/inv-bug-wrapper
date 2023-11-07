use aleph_client::SignedConnection;
use anyhow::Result;
use assert2::assert;
use ink_primitives::AccountId;
use ink_wrapper_types::{Connection as _, SignedConnection as _, TxStatus, UploadConnection as _};
use rand::RngCore as _;

use crate::{helpers::connect_as_test_account, test_contract};

fn random_salt() -> Vec<u8> {
    let mut salt = vec![0; 32];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

async fn connect_and_deploy() -> Result<(SignedConnection, test_contract::Instance)> {
    let conn = connect_as_test_account().await?;
    let contract = conn
        .instantiate(test_contract::Instance::default().with_salt(random_salt()))
        .await?;

    Ok((conn, contract))
}

#[tokio::test]
async fn test_simple_integer_messages() -> Result<()> {
    let (conn, contract) = connect_and_deploy().await?;

    let old_val = conn.read(contract.get_u32()).await??;
    println!("Val = {:?}", old_val);
    let new_val = old_val + 42;
    conn.exec(contract.set_u32(new_val)).await?;
    let val = conn.read(contract.get_u32()).await??;
    println!("Val = {:?}", val);

    let timestamps = conn.read(contract.get_timestamps()).await??;
    let mut last_timestamp = timestamps.0;

    for _ in 1..10 {
        let timestamps = conn.read(contract.get_timestamps()).await??;
        println!(
            "timestamp diff = {:?}  [{:?} - {:?}]",
            timestamps.0 - last_timestamp,
            timestamps.0,
            last_timestamp
        );    

        last_timestamp = timestamps.0;

        conn.exec(contract.update_timestamp()).await?;
    }
    
    // assert!(conn.read(contract.get_u32()).await?? == new_val);

    Ok(())
}