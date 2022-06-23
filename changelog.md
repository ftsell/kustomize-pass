<!--
This changelog file is intended to be updated during development and is automatically cleared after
a release.

Uncomment any of the following sections when they become relevant.
-->

## Notable Changes
Below are some of the most relevant changes that are introduced with this release.
You should read at least the *Breaking Changes* section.

<!--
### Breaking Changes
-->

### Additions

- Add support for specifying a password store's remote repository source.

    This kustomize plugin will automatically try to clone the repository and then retrieve data from it.

    To use this, you can add the following property to your `PassSecret` manifests:
    ```yaml
    apiVersion: ftsell.de/v1beta1
    kind: PassSecret
    metadata:
      name: something
    source:     # leave empty for local ~/.password-store
    # or
    source:
      url: git@github.com:some-user/some-repo.git   # a git clone url
    ```
