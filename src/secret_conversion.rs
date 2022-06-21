use crate::k8s_types::V1Secret;
use crate::V1Beta1PassSecret;
use anyhow::{anyhow, Context};
use libpass::StoreEntry;
use serde_yaml::Value;

fn convert_value(pass_name: &Value) -> anyhow::Result<Value> {
    let pass_name = match pass_name {
        Value::String(pass_name) => Ok(pass_name),
        _ => Err(anyhow!("Mapping is not a string")),
    }?;

    log::debug!("Retrieving {} from pass", &pass_name);
    let pass_entry = match libpass::retrieve(pass_name)
        .context(format!("Could not convert {} to pass secret", pass_name))?
    {
        StoreEntry::Directory(_) => Err(anyhow!("Entry is a directory")),
        StoreEntry::File(file) => Ok(file),
    }?;

    let result = String::from_utf8(pass_entry.plain_io()?.as_ref().clone())?
        .strip_suffix('\n')
        .ok_or_else(|| anyhow!("Could not strip \\n suffix"))?
        .to_string();

    Ok(Value::String(result))
}

impl TryFrom<V1Beta1PassSecret> for V1Secret {
    type Error = anyhow::Error;

    fn try_from(mut value: V1Beta1PassSecret) -> Result<Self, Self::Error> {
        log::debug!(
            "Trying to convert PassSecret {} to Secret",
            &value.metadata.name
        );

        // remove some internal annotations so that the secret doesn't get stripped out by kustomize
        if let Some(ref mut annotations) = value.metadata.annotations {
            annotations.remove(&Value::String(
                "config.kubernetes.io/local-config".to_string(),
            ));
            annotations.remove(&Value::String("config.kubernetes.io/function".to_string()));
        }

        // resolve all pass secrets
        for (_key, value) in value.data.iter_mut() {
            *value = convert_value(value)?;
        }

        // construct and return result
        log::debug!(
            "Done converting PassSecret {} to Secret",
            &value.metadata.name
        );
        Ok(V1Secret::new(
            value.metadata,
            value.data,
            value.immutable,
            value.secret_type,
        ))
    }
}
