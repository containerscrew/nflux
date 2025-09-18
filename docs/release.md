# Publish a new release

Edit the version number in `nflux/Cargo.toml`:  
> Make sure to follow [Semantic Versioning](https://semver.org/).

```shell
cog commit feat -a "new tag X.X.X"
git push origin main
cog bump --version X.X.X
cog changelog > CHANGELOG.md
cog commit chore -a "update CHANGELOG.md for version 0.12.7"
git push origin main
git push 0.12.7
```