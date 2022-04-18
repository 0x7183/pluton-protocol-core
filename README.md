# Pluton Protocol

## Disclaimer

This protocol was designed and implemented for a computer science thesis, thanks to [toran777](https://github.com/toran777) for implementing the user interface.
This software is released without any guarantee, use it at your own risk.

## Functionality

Pluton Protocol can be used to redirect interests from your Terra assets, in that way you can donate/pay using your yield, also, users would be able to create profiles with image and name, so that people can easy find and fund them.

No fees are collected from the Smart Contract and we are NOT going to launch our own token.

At the moment Pluton works only with Anchor, but we are going to add support for Mars and LunaX too.

During the beta launch we will open two pools for these important projects:
- Planting trees with [Treedom](https://www.treedom.net/en/plant-a-tree), 
- Donations to [Angel Protocol](https://www.angelprotocol.io/) 

### For Developers:

#### Integration Testing

Compile, upload, initialize and test the smart contracts

```
git clone https://github.com/0x7183/pluton-protocol-core
```
```
cd pluton-protocol-core/contracts/<path>
```
```
python3 ../../test/test.py <path> <file_name.wasm> <option>
```

#### E. g.
```
cd pluton-protocol-core/contracts/deposits
```
```
python3 ../../test/test.py deposits pluton.wasm all
```

### Testnet

* Deposits address: `terra1wpyx7crglfu2w886y7wf9gkk3ak6l5layqf6zh`
* Profiles address `terra1hz26vrex2j4j0gy585fl95guasrk0mru3u7cqe`


#### Execute Deposits:

Deposit:
```
{
  "deposit": {
    "denom": "uusd",
    "beneficiary": "input_beneficiary_address",
    "beneficiary_amount": "beneficiary_amount"
  }
}
```
Withdraw outgoing donations/payments, only depositor can execute it:
```
{"withdrawal":{"id": "input_id"}}
```
Withdraw incoming donations/payments, only beneficiary can execute it:
``` 
{"withdraw_interest":{"id": "input_id"}}  
```

#### Execure Profiles

 Register a new profile, only one profile per address:
``` 
{"register" : "img_url": "", "name": "0x7183", "description": "this is a test", "github": "https://github.com/", "linkedin": "https://linkedin.com/", "twitter": "https://twitter.com"}} 
```
 Modify an existing profile, only the owner can modify it:
 ``` 
 {"register" : "img_url": "", "name": "0x7183", "description": "this is a modify", "github": "https://github.com/", "linkedin": "https://linkedin.com/", "twitter": "https://twitter.com"}}  
```
 Delete an existing profile, only the owner can execute it:
``` 
{"delete": { }}	
```

#### Deposits Query

Returns all outgoing donations/payments with id:
```
{"depositor_balance":{"address": "input_address"}}
```
Returns all incoming donations/payments with id:
 ```
{"beneficiary_balance":{"address": "input_address"}}
 ```
Return a specific incoming donation/payment, it may panic:
```
{"incoming":{"address": "input_address", "id":"input_id"}}
```
Return a specific outgoing donation/payment, it may panic:
```
{"outgoing" :{"address": "input_address", "id":"input_id"}}
```

#### Profiles Query

Return information for a specific profile: 
```
{"get_profile": {"address":"input_address"}}
```

