use super::{quat::QuatStruct, vector::VectorStruct};

#[paramacro::serialized_struct("Transform")]
#[derive(Debug)]
pub struct TransformStruct {
    #[paramacro::serialized_field = "Rotation"]
    rotation: QuatStruct,

    #[paramacro::serialized_field = "Translation"]
    translation: VectorStruct,

    #[paramacro::serialized_field = "Scale3D"]
    scale: VectorStruct,
}
