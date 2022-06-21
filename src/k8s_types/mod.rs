//! Type definitions for objects encoded in YAML and used in kubernetes contexts

mod field_ref;
mod file_ref;
mod k8s_secret;
mod krm_result;
mod object_meta;
mod pass_secret;
mod resource_list;
mod resource_ref;
mod type_id;

pub use field_ref::FieldRef;
pub use file_ref::FileRef;
pub use k8s_secret::V1Secret;
pub use object_meta::K8sObjectMeta;
pub use pass_secret::V1Beta1PassSecret;
pub use resource_list::V1ResourceList;
pub use resource_ref::ResourceRef;
pub use type_id::K8sTypeId;

/// Implement `ensure_api_version_kind()` for resources to ensure that they are indeed what they should be
macro_rules! ensure_api_version_kind_impl {
    ($struct:ty, $apiVersion:literal, $kind:literal) => {
        ensure_api_version_kind_impl!($struct, $apiVersion, $kind, k8s_type_id);
    };
    ($struct:ty, $apiVersion:literal, $kind:literal, $field:ident) => {
        impl $struct {
            /// Ensure that the apiVersion and kind field on this object are as they should be
            pub fn ensure_api_version_kind(&self) -> anyhow::Result<()> {
                anyhow::ensure!(
                    self.$field.api_version == $apiVersion,
                    "Unsupported resource apiVersion"
                );
                anyhow::ensure!(self.$field.kind == $kind, "Unsupported resource kind");
                Ok(())
            }
        }
    };
}

ensure_api_version_kind_impl!(V1ResourceList, "config.kubernetes.io/v1", "ResourceList");
ensure_api_version_kind_impl!(V1Beta1PassSecret, "ftsell.de/v1beta1", "PassSecret");
