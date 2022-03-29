use crate::impl_from_and_to_type_option;
use crate::services::field::{BoxTypeOptionBuilder, TypeOptionBuilder};
use crate::services::row::CellDataSerde;
use bytes::Bytes;
use flowy_derive::ProtoBuf;
use flowy_error::FlowyError;
use flowy_grid_data_model::entities::{FieldMeta, FieldType};
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct RichTextTypeOptionBuilder(RichTextTypeOption);
impl_into_box_type_option_builder!(RichTextTypeOptionBuilder);
impl_from_json_str_and_from_bytes!(RichTextTypeOptionBuilder, RichTextTypeOption);

impl TypeOptionBuilder for RichTextTypeOptionBuilder {
    fn field_type(&self) -> FieldType {
        self.0.field_type()
    }

    fn build_type_option_str(&self) -> String {
        self.0.clone().into()
    }

    fn build_type_option_data(&self) -> Bytes {
        self.0.clone().try_into().unwrap()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ProtoBuf)]
pub struct RichTextTypeOption {
    #[pb(index = 1)]
    pub format: String,
}
impl_from_and_to_type_option!(RichTextTypeOption, FieldType::RichText);

impl CellDataSerde for RichTextTypeOption {
    fn deserialize_cell_data(&self, data: String) -> String {
        data
    }

    fn serialize_cell_data(&self, data: &str) -> Result<String, FlowyError> {
        let data = data.to_owned();
        if data.len() > 10000 {
            Err(FlowyError::text_too_long().context("The len of the text should not be more than 10000"))
        } else {
            Ok(data)
        }
    }
}