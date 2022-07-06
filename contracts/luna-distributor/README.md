## About the contract

This is a smart contract for distribution of funds from the community pool as per [Prop 4080 on Terra Classic](https://classic-agora.terra.money/t/proposal-distribute-50-transaction-fees-to-the-community-pool-increase-proposer-validator-rewards/44729)

## Distribution logic

Consider a total transaction fee (TF) of 100 LUNC. The distribution logic is as follows:

- Amount going to the community pool (CP) = 50 LUNC (50% of TF)
- Amount reserved for core dev (CD) = 5 LUNC (5% of TF or 10% of CP)
- Amount to be distributed for burning + airdrop (DA) = 45 LUNC (45% of TF or 90% of CP)
- Burn amount (BA) = 35 LUNC (35% of TF or `77.78%` of DA)
- Airdrop amount (AA) = 10 LUNC (10% of TF or `22.22%` of DA)

The contract will only handle the distribution of the Burn Amount (BA) and Airdrop Amount (AA) in the ratio of `77.78:2.22`, as per the logic above.

## Burn address

The burn amount will be sent to the following address: 
```
terra1sk06e3dyexuq4shw77y3dsv480xv42mq73anxu
```

## Airdrop whitelist
If you are a Terra Classic dapp with a [TVL greater than 0](https://defillama.com/chain/Terra%20Classic), you can create a pull request [here](whitelist/airdrop.json) to qualify for the airdrop.

## Usage

You can interact with this smart contract using this **CLASSIC MAINNET** address:
```
terra1cml0je7m86tzaptk3y7sfvnkhwuuxt2xwpnax8
```
The contract has been instantiated without the `--set-signer-as-admin` flag, making it immutable (the contract cannot be migrated).

## ExecuteMsg

### Distribute
The following messages need to be sent to distribute the funds from the contract.

To distribute LUNC:
```
{"distribute":{"denom":"uluna"}}
```
To distribute USTC:
```
{"distribute":{"denom":"uusd"}}
```
### UpdateConfig

Config can be updated only by the admin set via `InstantiateMsg`. The administration of the contract is transferable to another account, a multisig wallet, or a governance contract. Setting the `admin` field as empty will make the contract non-updatable.

```
{
    "update_config": {
        "admin": "terra1na854dwyp46698ylzwsdqh7fs2tcvxl9rm4feg",
        "burn_address": "terra1sk06e3dyexuq4shw77y3dsv480xv42mq73anxu",
        "whitelist": [{
                "address": "terra1zw4hdq5zme37a3cvv9ad80deg54zfggxzkh3fu",
                "protocol": "terraswap"
            },
            {
                "address": "terra1f63jhwhcy9zccwwv9hnl8954hdns94krrxvwvs",
                "protocol": "anchor"
            }
        ],
        "weight_per_protocol": [{
                "protocol": "terraswap",
                "weight": "0.5"
            },
            {
                "protocol": "anchor",
                "weight": "0.5"
            }
        ]
    }
}
```


## Query

### Get Config
To get the current configuration of the contract:
```
{"config":{}}
```
