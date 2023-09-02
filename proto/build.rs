use anyhow::Result;

const DEFAULT: &str = r#"#[serde(default)]"#;
const SERIALIZED: &str = r#"#[derive(::serde::Deserialize, ::serde::Serialize)]"#;
const BASE64STRING: &str =
    r#"#[serde(with = "tendermint_proto::serializers::bytes::base64string")]"#;
const QUOTED: &str = r#"#[serde(with = "tendermint_proto::serializers::from_str")]"#;
const VEC_BASE64STRING: &str =
    r#"#[serde(with = "tendermint_proto::serializers::bytes::vec_base64string")]"#;
const PASCAL_CASE: &str = r#"#[serde(rename_all = "PascalCase")]"#;
const OPTION_ANY: &str = r#"#[serde(default, with = "crate::serializers::option_any")]"#;
const OPTION_TIMESTAMP: &str =
    r#"#[serde(default, with = "crate::serializers::option_timestamp")]"#;

#[rustfmt::skip]
pub static CUSTOM_TYPE_ATTRIBUTES: &[(&str, &str)] = &[
    (".celestia.da.DataAvailabilityHeader", SERIALIZED),
    (".cosmos.base.abci.v1beta1.ABCIMessageLog", SERIALIZED),
    (".cosmos.base.abci.v1beta1.Attribute", SERIALIZED),
    (".cosmos.base.abci.v1beta1.StringEvent", SERIALIZED),
    (".cosmos.base.abci.v1beta1.TxResponse", SERIALIZED),
    (".cosmos.base.v1beta1.Coin", SERIALIZED),
    (".cosmos.base.query.v1beta1.PageResponse", SERIALIZED),
    (".cosmos.staking.v1beta1.QueryDelegationResponse", SERIALIZED),
    (".cosmos.staking.v1beta1.DelegationResponse", SERIALIZED),
    (".cosmos.staking.v1beta1.Delegation", SERIALIZED),
    (".cosmos.staking.v1beta1.QueryRedelegationsResponse", SERIALIZED),
    (".cosmos.staking.v1beta1.RedelegationResponse", SERIALIZED),
    (".cosmos.staking.v1beta1.Redelegation", SERIALIZED),
    (".cosmos.staking.v1beta1.RedelegationEntryResponse", SERIALIZED),
    (".cosmos.staking.v1beta1.RedelegationEntry", SERIALIZED),
    (".cosmos.staking.v1beta1.QueryUnbondingDelegationResponse", SERIALIZED),
    (".cosmos.staking.v1beta1.UnbondingDelegation", SERIALIZED),
    (".cosmos.staking.v1beta1.UnbondingDelegationEntry", SERIALIZED),
    (".header.pb.ExtendedHeader", SERIALIZED),
    (".share.eds.byzantine.pb.BadEncoding", SERIALIZED),
    (".share.eds.byzantine.pb.MerkleProof", SERIALIZED),
    (".share.eds.byzantine.pb.Share", SERIALIZED),
    (".share.p2p.shrex.nd.Proof", SERIALIZED),
    (".share.p2p.shrex.nd.Row", SERIALIZED),
    (".share.p2p.shrex.nd.Row", PASCAL_CASE),
];

#[rustfmt::skip]
pub static CUSTOM_FIELD_ATTRIBUTES: &[(&str, &str)] = &[
    (".celestia.da.DataAvailabilityHeader.row_roots", VEC_BASE64STRING),
    (".celestia.da.DataAvailabilityHeader.column_roots", VEC_BASE64STRING),
    (".cosmos.base.abci.v1beta1.TxResponse.tx", OPTION_ANY),
    (".cosmos.base.query.v1beta1.PageResponse.next_key", BASE64STRING),
    (".cosmos.staking.v1beta1.RedelegationEntry.completion_time", OPTION_TIMESTAMP),
    (".cosmos.staking.v1beta1.UnbondingDelegationEntry.completion_time", OPTION_TIMESTAMP),
    (".share.eds.byzantine.pb.MerkleProof.nodes", VEC_BASE64STRING),
    (".share.eds.byzantine.pb.MerkleProof.leaf_hash", DEFAULT),
    (".share.eds.byzantine.pb.MerkleProof.leaf_hash", BASE64STRING),
    (".share.eds.byzantine.pb.BadEncoding.axis", QUOTED),
    (".share.p2p.shrex.nd.Proof.nodes", VEC_BASE64STRING),
    (".share.p2p.shrex.nd.Proof.hashleaf", DEFAULT),
    (".share.p2p.shrex.nd.Proof.hashleaf", BASE64STRING),
    // TODO: remove me  https://github.com/celestiaorg/celestia-node/issues/2427
    (".share.p2p.shrex.nd.Proof.hashleaf", r#"#[serde(rename = "leaf_hash")]"#),
    (".share.p2p.shrex.nd.Row.shares", VEC_BASE64STRING),
];

fn main() -> Result<()> {
    let mut config = prost_build::Config::new();

    for (type_path, attr) in CUSTOM_TYPE_ATTRIBUTES {
        config.type_attribute(type_path, attr);
    }

    for (field_path, attr) in CUSTOM_FIELD_ATTRIBUTES {
        config.field_attribute(field_path, attr);
    }

    config
        .include_file("mod.rs")
        .extern_path(".tendermint", "::tendermint_proto::v0_34")
        .extern_path(
            ".google.protobuf.Timestamp",
            "::tendermint_proto::google::protobuf::Timestamp",
        )
        .extern_path(
            ".google.protobuf.Duration",
            "::tendermint_proto::google::protobuf::Duration",
        )
        // Comments in Google's protobuf are causing issues with cargo-test
        .disable_comments([".google"])
        .compile_protos(
            &[
                "vendor/celestia/da/data_availability_header.proto",
                "vendor/header/pb/extended_header.proto",
                "vendor/share/p2p/shrexnd/pb/share.proto",
                "vendor/share/eds/byzantine/pb/share.proto",
                "vendor/cosmos/base/v1beta1/coin.proto",
                "vendor/cosmos/base/abci/v1beta1/abci.proto",
                "vendor/cosmos/staking/v1beta1/query.proto",
                "vendor/go-header/p2p/pb/header_request.proto",
            ],
            &["vendor"],
        )?;

    Ok(())
}
