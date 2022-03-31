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

### Testnet address

```
terra1w73aaax36u9hqn4xqjvh3mhy3hygllztzu9emj
```
#### Query

Returns all outgoing donations/payments with id:
```
{"depositor_balance":{"address": "input_address"}}
```
Returns all incoming donations/payments with id:
 ```
{"beneficiary_balance":{"address": "input_address"}}
 ```
Return a specific incoming donation/payment):
```
{"incoming":{"address": "input_address", "id":"input_id"}}
```
Return a specific outgoing donation/payment:
```
{"outgoing" :{"address": "input_address", "id":"input_id"}}
```
Return information for a specific profile: 
```
{"get_profile": {"address":"input_address"}}
```
#### Execute:

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
{"withdrawal":{"id": "1"}}
```
Withdraw incoming donations/payments, only beneficiary can execute it:
``` 
{"withdraw_interest":{"id": "1"}}  
```
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
