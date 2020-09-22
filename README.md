# Subswap 🌊

<p align="center">
  <img src="/docs/media/subswap.jpg" width="300">
</p>

[![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](LICENSE) 
[![Twitter](https://img.shields.io/twitter/follow/SubstrateSwap?label=Follow&style=social)](https://twitter.com/SubstrateSwap)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](docs/CONTRIBUTING.adoc)


Subswap is a permissionless decentralized value exchange protocol which focuses in laying the foundation of the next interoperable cryptoeconomic activities 🚢.

## Trying it out

### For developers
Subswap is developed in a way that it is compatible to the latest [substrate](www.github.com/paritytech/substrate) project dependencies.
To contribute to this project, Simply go to [substrate.dev](https://substrate.dev) and follow the 
[installation](https://substrate.dev/docs/en/knowledgebase/getting-started/) instructions. You can 
also try out one of the [tutorials](https://substrate.dev/en/tutorials).

For MacOS devs, the current rust nightly compiler crashes so adjust rustc version to compile this project
```bash
rustup install nightly-2020-09-11
rustup default nightly-2020-09-11
rustup target add wasm32-unknown-unknown
```

### For users

Subswap is a permissionless decentralized finance and exchange protocol where holders are rewarded for their activities such as providing liquidity, creating values through decentralization, connecting two different assets with exchanging, and acting in on-chain governance.

### Create values

Create values with [asset module](./frame/asset/lib.rs)'s `issue` function. Users can issue their own asset for their personal use or propose to the public to issue with [Democracy](./frame/democracy/lib.rs) module's propose function to get agreement from holders that the asset has value. When the proposal passes, one can claim his asset's legitmacy by showing proposal hash which is permanently recorded in the blockchain.

### Exchange values

Create pairs between two assets to exchange. Liquidity providers are rewarded as liquidity provider tokens(a.k.a lp token), and users can claim their rewards by burning it or stake to pools to get SUB, the base currency of subswap protocol. Pairs are developed to create the least impermanent loss 

### Yield farming

SUB is the native currency of Subswap network, and it is not distributed easily. SUB is for liquidity providers and degens who creates cryptocurrency ecosystem. To reward providing liquidity. SUB pools are created to stake lptokens and get SUB. 

### Democracy
*For the liquidity providers, By the liquidity providers, of the liquidity providers* 

SUB is used to determine the future of Subswap protocol. SUB council and holders will usually determine:
- To issue cryto asset with proven utility
- To set up exchanges between one asset to the another
- Set reward rate of SUB pools
- Add new features (e.g. synthetic assets, derivatives, insurance)

### Treasury

SUB funds can be made to reward open source developers who bridge other heteregeneous crypto assets and Subswap protocol assets. 

### Contract

To provide utility to tokens registered in the subswap protocol, developers can setup business logic with [ink!]() to interact with SUB and other registered tokens. 

## Documentation

For more research and details, check out the documentation link below.

<a href=""><img src="https://github.com/terra-project/houston/blob/master/assets/gitbook.png" width="300"></a>

## Contributions & Code of Conduct

Please follow the contributions guidelines as outlined in [`docs/CONTRIBUTING.adoc`](docs/CONTRIBUTING.adoc). In all communications and contributions, this project follows the [Contributor Covenant Code of Conduct](docs/CODE_OF_CONDUCT.md).

## Security

The security policy and procedures can be found in [`docs/SECURITY.md`](docs/SECURITY.md).

## ChangeLogs

### Plastic Beach v0.0.1

[Pool](), [Market](), [Asset]() modules are being implemented.

**.
.
.**
***For More Changes read [CHANGELOG.md](CHANGELOG.md)***

## License

- Substrate Primitives (`sp-*`), Frame (`frame-*`) and the pallets (`pallets-*`), binaries (`/bin`) and all other utilities are licensed under [Apache 2.0](LICENSE-APACHE2).
- Subswap Primitives(`swp-*`), Frame(`subswap-*`) and the pallets (`subswap-*`) are licensed under [Apache 2.0](LICENSE-APACHE2).
- Substrate Client (`/client/*` / `sc-*`) is licensed under [GPL v3.0 with a classpath linking exception](LICENSE-GPL3).

The reason for the split-licensing is to ensure that for the vast majority of teams using Substrate to create feature-chains, then all changes can be made entirely in Apache2-licensed code, allowing teams full freedom over what and how they release and giving licensing clarity to commercial teams.

In the interests of the community, we require any deeper improvements made to Substrate's core logic (e.g. Substrate's internal consensus, crypto or database code) to be contributed back so everyone can benefit.
