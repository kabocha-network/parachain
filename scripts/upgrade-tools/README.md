
# ToDo Automated Upgrade Tools

These are tools to automate the workflows for Pop-Art relay chain testnet. Register, run and upgrade parachains. 


This is the workflow related to:
## 1 registering para-id
 - import relevant polkadot utils/tools/api methods
 - add variables into config which will be:
   - the ${RPC} node.
 - psuedocode: await sudo.tx.api.register-para-id();
 - get para-id from the event block and update the RPC variable of the config.js

## 2 Check config is preparared
 - ${PARAID}
 - ${SUDO}
 - ${SUDOBALANCE}
 - ${COLLATORS}
 -  ...
 - Catch errors such as empty variables in config. 

## 3 create a build spec and add variables from config:
Then run through build spec update build spec config await console.log(build spec updated), then await build raw spec, then await build genesis and wasm files. 
 Then... 

## 4 Register parathread:
 - psuedocode: await sudo.tx.api.register-parathread(add parameters such as the account, the ${WASM}, and ${GENESIS});
 - success

## 5 Launch service file 
 - add service file with required variables from service-congig.js 


