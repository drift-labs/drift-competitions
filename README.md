# drift-competitions
a sybil-resistant randomized onchain giveaway program

- see the wiki for indepth discussion on design of program


### quick start
```
yarn
export ANCHOR_WALLET=<PATH_TO_WALLET>
export RPC_OVERRIDE=<URL>
ts-node -T scripts/mineAdditionalEntries.ts --authority=<PUBLIC_KEY_OF_AUTHORIY> --n=<NUMBER_OF_ENTRIES>
```