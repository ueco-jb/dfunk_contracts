## About the smart contract
This is a smart contract for distribution of funds from the community pool as per [Prop 4080 on Terra Classic](https://classic-agora.terra.money/t/proposal-distribute-50-transaction-fees-to-the-community-pool-increase-proposer-validator-rewards/44729)

## Distribution Logic
Consider a total transaction fee (TF) of 100 LUNC. The distribution logic is as follows:
- Amount going to the community pool (CP) = 50 LUNC (50% of TF)
- Amount reserved for core dev (CD) = 5 LUNC (5% of TF or 10% of CP)
- Amount to be distributed for burning + airdrop (DA) = 45 LUNC (45% of TF or 90% of CP)
- Burn amount (BA) = 35 LUNC (35% of TF or `77.78%` of DA)
- Airdrop amount (AA) = 10 LUNC (10% of TF or `22.22%` of DA)

## Burn Address
The burn amount will be sent to the following address: ```terra1sk06e3dyexuq4shw77y3dsv480xv42mq73anxu```

## Airdrop Whitelist
If you are a Terra Classic dapp with a [TVL greater than 0](https://defillama.com/chain/Terra%20Classic), you can create a pull request [here](4080-airdrop-whitelist.json) to qualify for the airdrop.
