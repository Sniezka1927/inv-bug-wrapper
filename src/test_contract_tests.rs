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
async fn test_update_timestamps() -> Result<()> {
    let (conn, contract) = connect_and_deploy().await?;

    conn.exec(contract.update_timestamp()).await?;
    let timestamps = conn.read(contract.get_timestamps()).await??;
    let mut last_timestamp = timestamps.0;

    for _ in 1..29 {
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
    
    Ok(())
}

#[tokio::test]
async fn test_update_timestamps_no_store() -> Result<()> {
    let (conn, contract) = connect_and_deploy().await?;
    
    for _ in 1..29 {
        conn.exec(contract.update_timestamp_without_store()).await?;
    }

    Ok(())
}


#[tokio::test]
async fn test_update_timestamps_diffrent_operations_add() -> Result<()> {
    let (conn, contract) = connect_and_deploy().await?;
    
    for _ in 1..29 {
        conn.exec(contract.update_timestamp_diffrent_operations_add()).await?;
    }
    
    Ok(())
}

#[tokio::test]
async fn test_update_timestamps_diffrent_operations_sub() -> Result<()> {
    let (conn, contract) = connect_and_deploy().await?;
    
    for _ in 1..29 {
        conn.exec(contract.update_timestamp_diffrent_operations_sub()).await?;
    }
    
    Ok(())
}

#[tokio::test]
async fn test_update_timestamps_diffrent_operations_mul() -> Result<()> {
    let (conn, contract) = connect_and_deploy().await?;
    
    for _ in 1..29 {
        conn.exec(contract.update_timestamp_diffrent_operations_mul()).await?;
    }
    
    Ok(())
}
