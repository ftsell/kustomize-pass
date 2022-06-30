<!--
This changelog file is intended to be updated during development and is automatically cleared after
a release.

Uncomment any of the following sections when they become relevant.
-->

## Notable Changes
Below are some of the most relevant changes that are introduced with this release.
You should read at least the *Breaking Changes* section.

### Bugfixes

- Fixed a bug that prevented decrypting pass secrets in environments where not all keys of the password store were known.
    This behavior has been changed so that now only one private key is required while all other public keys need not be known.

<!--
### Breaking Changes

### Additions
-->
