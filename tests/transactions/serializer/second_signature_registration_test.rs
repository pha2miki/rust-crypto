use arkecosystem_crypto::configuration::network;
use arkecosystem_crypto::enums::Network;
use arkecosystem_crypto::transactions::{deserializer, serializer};
use *;

#[test]
fn test_signed_with_a_passphrase() {
    network::set(Network::Devnet);

    let fixture = json_transaction("second_signature_registration", "second-passphrase");
    let serialized = fixture["serialized"].as_str().unwrap();
    let transaction = deserializer::deserialize(&serialized);

    let actual = serializer::serialize(&transaction);
    assert_eq!(actual, serialized);
}
