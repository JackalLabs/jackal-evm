## testing commands

### DeployMailbox.s.sol 

Run anvil first, followed by the below command. 

```shell
$ forge script script/DeployMailbox.s.sol --broadcast --rpc-url http://localhost:8545
```

### Mailbox.t.sol

This is just the mailbox unit test 

```shell
$ forge test -vv --via-ir       
```
