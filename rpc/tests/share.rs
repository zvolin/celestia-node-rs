use celestia_rpc::prelude::*;
use celestia_types::consts::appconsts::{
    CONTINUATION_SPARSE_SHARE_CONTENT_SIZE, FIRST_SPARSE_SHARE_CONTENT_SIZE, SEQUENCE_LEN_BYTES,
    SHARE_INFO_BYTES,
};
use celestia_types::Blob;
use jsonrpsee::http_client::HttpClient;

pub mod utils;

use utils::{random_bytes, random_ns, test_client, AuthLevel};

async fn test_get_shares_by_namespace(client: &HttpClient) {
    let namespace = random_ns();
    let data = random_bytes(1024);
    let blob = Blob::new(namespace, data.clone()).unwrap();

    let submitted_height = client.blob_submit(&[blob.clone()]).await.unwrap();

    let dah = client
        .header_get_by_height(submitted_height)
        .await
        .unwrap()
        .dah;

    let ns_shares = client
        .share_get_shares_by_namespace(&dah, namespace)
        .await
        .unwrap()
        .unwrap();

    let seq_len =
        &ns_shares.rows[0].shares[0].data[SHARE_INFO_BYTES..SHARE_INFO_BYTES + SEQUENCE_LEN_BYTES];
    let seq_len = u32::from_be_bytes(seq_len.try_into().unwrap());
    assert_eq!(seq_len as usize, data.len());

    let reconstructed_data = ns_shares
        .rows
        .into_iter()
        .flat_map(|row| row.shares.into_iter())
        .fold(vec![], |mut acc, share| {
            let data = if acc.is_empty() {
                &share.data[share.data.len() - FIRST_SPARSE_SHARE_CONTENT_SIZE..]
            } else {
                &share.data[share.data.len() - CONTINUATION_SPARSE_SHARE_CONTENT_SIZE..]
            };
            acc.extend_from_slice(data);
            acc
        });

    assert_eq!(&reconstructed_data[..seq_len as usize], &data[..]);
}

async fn test_get_shares_by_namespace_wrong_ns(client: &HttpClient) {
    let namespace = random_ns();
    let data = random_bytes(1024);
    let blob = Blob::new(namespace, data.clone()).unwrap();

    let submitted_height = client.blob_submit(&[blob.clone()]).await.unwrap();

    let dah = client
        .header_get_by_height(submitted_height)
        .await
        .unwrap()
        .dah;

    println!("height: {submitted_height}");
    println!("dah: {}", serde_json::to_string_pretty(&dah).unwrap());

    for _ in 0..10 {
        let random_ns = random_ns();
        println!("ns: {}", serde_json::to_string_pretty(&random_ns).unwrap());

        let ns_shares = client
            .share_get_shares_by_namespace(&dah, random_ns)
            .await
            .unwrap();

        if ns_shares.is_none() {
            println!("Is none");
            continue;
        }

        assert!(ns_shares
            .unwrap()
            .rows
            .iter()
            .all(|row| row.proof.is_of_absence()));
        println!("Is of absence");
        continue;
    }

    panic!()
}

async fn test_get_shares_by_namespace_wrong_roots(client: &HttpClient) {
    let namespace = random_ns();
    let data = random_bytes(1024);
    let blob = Blob::new(namespace, data.clone()).unwrap();

    client.blob_submit(&[blob.clone()]).await.unwrap();

    let genesis_dah = client.header_get_by_height(1).await.unwrap().dah;

    client
        .share_get_shares_by_namespace(&genesis_dah, namespace)
        .await
        .unwrap_err();
}

#[tokio::test]
async fn share_api() {
    let client = test_client(AuthLevel::Write).unwrap();

    // minimum 2 blocks
    client.header_wait_for_height(2).await.unwrap();

    test_get_shares_by_namespace(&client).await;
    test_get_shares_by_namespace_wrong_ns(&client).await;
    test_get_shares_by_namespace_wrong_roots(&client).await;
}
