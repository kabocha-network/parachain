# Kabocha Parachain 

Based on the Cumulus Parachain Template. Big thanks to the Substrate team, and special thanks to Dan Shields. 

👉 Learn more about parachains [here](https://wiki.polkadot.network/docs/learn-parachains), and
parathreads [here](https://wiki.polkadot.network/docs/learn-parathreads).

🧙 Learn about how to use this template and run your own parachain testnet for it in the
[Devhub Cumulus Tutorial](https://docs.substrate.io/tutorials/v3/cumulus/start-relay/).

## Crowdloan process 

We are launching as a featureless parachain template, with only balances and vesting. After an auction win, the sudo will populate balances and vested balances with required amounts both from crowdloan participation and EDG snapshot. Balances will have a filter so that people can transfer balances. A runtime upgrade will then occur to include governance related pallets. Once that happens, anyone can make a proposal to kill the sudo account, once that happens a runtime upgrade to unfilter balances will ensue and the chain will be in its first live state. Thereafter, future runtime upgrades to add custom pallets and features will go through Kabocha on-chain governance. 
## Kabocha pallet ingredients 

We will start of with governance and "NFT" functionality. Thereafter will be customising governance to be more like Liquid Democracy, we will also be adding functionality to the treasury funding process, where new coins can be minted based on successful proposals, instead of just taking from treasury.  
## In the works:
This parachain will have novel governance function. We will be working on Liquid Democracy, which will remove the need for council and instead be more focussed on meta-delegation.