# kustomize-pass

[![crates.io Badge](https://img.shields.io/crates/v/kustomize-pass?style=for-the-badge)](https://crates.io/crates/kustomize-pass)
![Maintenance Status Badge](https://img.shields.io/maintenance/yes/2022?style=for-the-badge)

A kustomize plugin that is able to generate secrets by extracting them from [pass](https://www.passwordstore.org/)
or replace placeholders in other manifests from pass.

## Installation
For installation, this package depends on [gpgme-rs](https://crates.io/crates/gpgme) which requires the gpgme library and its development files (e.g., headers, gpgme-config) to be installed during the build process.
You should install these using your operating systems package manager.

Afterwards, you can install the package either using one of the provided binaries from the [releases page](https://github.com/ftsell/kustomize-pass/releases/) or compile and install it yourself by running
```shell
cargo install kustomize-pass
```

## Usage

Once *kustomize-pass* is installed, you can use the generator by providing kustomize with the following example resource manifests.

A detailed description of the supported input manifest is provided in openapi format in the [schema.openapi.yaml](./schema.openapi.yaml).
It can also be generated and printed on-demand by the application.

```yaml
# generator.yml
apiVersion: ftsell.de/v1beta1
kind: PassSecret
metadata:
  name: example-secret
  annotations:
    config.kubernetes.io/function: |
      exec:
        path: kustomize-pass
data:
  example-key: example-pass-name
```

```yaml
# kustomization.yml
apiVersion: kustomize.config.k8s.io/v1beta1
generators:
  - generator.yml
```

When running the shown example and if you have a password named `example-pass-name` in your password store, the following
resulting resource will be produced:

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: example-secret
data:
  example-key: foobar123
```
