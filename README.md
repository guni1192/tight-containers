## Tight-Containers Version2

![stable channel](https://github.com/guni1192/tight-containers-v2/workflows/stable%20channel/badge.svg)
![nightly channel](https://github.com/guni1192/tight-containers-v2/workflows/nightly%20channel/badge.svg)
[![Coverage Status](https://coveralls.io/repos/github/guni1192/tight-containers-v2/badge.svg?branch=citest&t=SLnAHd)](https://coveralls.io/github/guni1192/tight-containers-v2?branch=citest)

## Changes from v1 to v2

### Previous Probrem 

- Multi-Binary => Inpossible to manage by crate.io
- Multi Package => CI workflow can be time consuming
- Dependence libraries duty is become bloated.

### This Solution 

- Single-Binary => Can be release to crate.io
- Single-Package => CI workflow became very simple.
- Dependence libraries are replaced to rust module.
