# Simple fungible token on SecretNetwork platform (interview task)

## Build
Simple `$ make` should suffice on Linux.

## Test
`$ make test`

## Usage
Optional fields in contract messages are marked with `#`.

* Store the contract
`$ secretcli tx compute store <contract_name> --from <key_address> --gas <gas>`

* Instantiate the contract
`$ secretcli tx compute instantiate <code_id> '{#"balances": [{"address": <key_address>, "amount": <amount>}]}' --from <key_address>`

* Transfer tokens
`$ secretcli tx compute execute <contract_address> '{"transfer": {"to": <key_address>, "amount": <amount>}}' --from <key_address> --gas <gas>`

* Burn tokens
`$ secretcli tx compute execute <contract_address> '{"burn": {"amount": <amount>}}'`

* Query the balance
`$ secretcli query compute query <contract_address> '{"balance": {"address": <key_address>}}'`
