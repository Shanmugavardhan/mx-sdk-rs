use multiversx_sc::imports::*;
use crate::abi_test_type::{AbiEnvelope, AbiWithManagedBufferReadToEnd};

#[multiversx_sc::module]
pub trait AbiExternalModule: crate::abi_storage::AbiStorageModule {
    #[view]
    fn envelope_like_result(&self) -> AbiEnvelope<Self::Api> {
        AbiEnvelope {
            domain: crate::abi_test_type::AbiEnvelopeDomain::Alpha,
            payload_hash: ManagedBuffer::from(&[9u8; 32]),
            operations: self.item_for_managed_complex_vec(),
        }
    }

    #[endpoint]
    fn validate_token_id_and_return_envelope(
        &self,
        token_id: ManagedBuffer,
    ) -> AbiEnvelope<Self::Api> {
        self.require_valid_token_id(&token_id);
        self.envelope_like_result()
    }

    #[view]
    fn validate_constant_token_id_and_return_envelope(&self) -> AbiEnvelope<Self::Api> {
        let token_id = ManagedBuffer::from(b"CARBON-ab12cd");
        self.require_valid_token_id(&token_id);
        self.envelope_like_result()
    }

    #[endpoint]
    fn set_token_scoped_value_and_return_envelope(
        &self,
        token_id: ManagedBuffer,
        value: ManagedBuffer,
    ) -> AbiEnvelope<Self::Api> {
        self.require_valid_token_id(&token_id);
        self.token_scoped_value(&token_id).set(value);
        self.envelope_like_result()
    }

    #[endpoint]
    fn time_types(
        &self,
    ) -> MultiValue4<TimestampMillis, TimestampSeconds, DurationMillis, DurationSeconds> {
        (
            TimestampMillis::new(0),
            TimestampSeconds::new(0),
            DurationMillis::new(0),
            DurationSeconds::new(0),
        ).into()
    }

    #[view]
    fn operation_completion_status(&self) -> OperationCompletionStatus {
        OperationCompletionStatus::Completed
    }

    #[view]
    fn takes_object_with_managed_buffer_read_to_end(
        &self,
        arg: AbiWithManagedBufferReadToEnd<Self::Api>,
    ) -> ManagedBuffer {
        arg.flush.into_managed_buffer()
    }

    #[endpoint]
    #[label("test-external-view")]
    fn external_view(&self) {}

    #[endpoint]
    #[label("label1")]
    fn label_a(&self) {}

    #[endpoint]
    #[label("label1")]
    #[label("label2")]
    fn label_b(&self) {}

    #[storage_mapper("tokenScopedValue")]
    fn token_scoped_value(&self, token_id: &ManagedBuffer) -> SingleValueMapper<ManagedBuffer>;

    /// Internalized validation logic from drwa-common.
    /// This ensures the ABI tester remains a "torture test" for complex logic 
    /// but without depending on external project crates.
    fn require_valid_token_id(&self, token_id: &ManagedBuffer) {
        if token_id.is_empty() {
            sc_panic!("token_id must not be empty");
        }

        let len = token_id.len();
        if len < 8 {
            sc_panic!("token_id is too short");
        }
        if len > 17 {
            sc_panic!("token_id is too long");
        }

        let mut bytes = [0u8; 17];
        let _ = token_id.load_slice(0, &mut bytes[..len]);
        let token_id_bytes = &bytes[..len];

        if token_id_bytes.contains(&0) {
            sc_panic!("token_id must not contain null bytes");
        }
        let hyphen_pos = token_id_bytes
            .iter()
            .position(|b| *b == b'-')
            .unwrap_or(token_id_bytes.len());
        if token_id_bytes.iter().filter(|b| **b == b'-').count() != 1 {
            sc_panic!("token_id must contain exactly one hyphen");
        }
        if hyphen_pos < 3 {
            sc_panic!("token_id ticker is too short");
        }
        if hyphen_pos > 10 {
            sc_panic!("token_id ticker is too long (max 10 chars)");
        }
        if hyphen_pos + 7 != token_id_bytes.len() {
            sc_panic!("token_id suffix must be 6 characters");
        }

        for (index, byte) in token_id_bytes.iter().enumerate() {
            if index < hyphen_pos {
                if !(byte.is_ascii_uppercase() || byte.is_ascii_digit()) {
                    sc_panic!("token_id ticker must be uppercase alphanumeric");
                }
            } else if index > hyphen_pos && !(byte.is_ascii_digit() || (b'a'..=b'f').contains(byte)) {
                sc_panic!("token_id suffix must be lowercase hex");
            }
        }
    }
}
