use crate::k8s_types::V1Secret;
use crate::V1Beta1PassSecret;
use anyhow::{anyhow, Context};
use libpass::StoreEntry;
use serde_yaml::{Mapping, Value};

/// An value that is encoded so that it cane easily be used as a value for Kubernetes Secrets
#[derive(Debug, Clone, Eq, PartialEq)]
enum SecretValue {
    /// Binary base64 encoded content
    Binary(String),

    /// UTF-8 encoded string content
    String(String),
}

fn convert_value(pass_name: &Value) -> anyhow::Result<SecretValue> {
    // convert arguments to usable format
    let pass_name = match pass_name {
        Value::String(pass_name) => Ok(pass_name),
        _ => Err(anyhow!("Mapping is not a string")),
    }?;

    // retrieve entry from store
    log::debug!("Retrieving {} from pass", &pass_name);
    let pass_entry = match libpass::retrieve(pass_name)
        .context(format!("Could not convert {} to pass secret", pass_name))?
    {
        StoreEntry::Directory(_) => Err(anyhow!("Entry is a directory")),
        StoreEntry::File(file) => Ok(file),
    }?;

    // read and decrypt content from entry
    let bin_result = pass_entry.plain_io()?.as_ref().to_owned();
    Ok(match String::from_utf8(bin_result) {
        Ok(str_result) => SecretValue::String(
            str_result
                .strip_suffix('\n')
                .map(|str_result| str_result.to_string())
                .unwrap_or(str_result),
        ),
        Err(e) => SecretValue::Binary(base64::encode(e.as_bytes())),
    })
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
        let mut str_results = Mapping::new();
        let mut bin_results = Mapping::new();
        for (key, value) in value.data.iter_mut() {
            match convert_value(value)? {
                SecretValue::String(result) => {
                    str_results.insert(key.to_owned(), Value::String(result))
                }
                SecretValue::Binary(result) => {
                    bin_results.insert(key.to_owned(), Value::String(result))
                }
            };
        }

        // construct and return result
        log::debug!(
            "Done converting PassSecret {} to Secret",
            &value.metadata.name
        );
        Ok(V1Secret::new(
            value.metadata,
            value.immutable,
            value.secret_type,
            str_results,
            bin_results,
        ))
    }
}
