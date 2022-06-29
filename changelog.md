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

- Add support for reading ssh keys from ~/.ssh/id_rsa

## Internal Changes

- The code regarding git credentials has been restructured a bit to only supply correct credential types to libgit.
  This means that no SSH key will be given to git to authenticate an HTTPS pull (because that cannot ever work).
