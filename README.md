# xvm-tests

## Ink -> EVM 

**Deploy the Ink Contract**

After compiling the contract deploy it with:

    $ python3 wasm-contracts/flipper/deploy.py

And store the contract address, e.g: b5qtfzgaRKhFkGE2FFT5b7ShofrUHXrsSkfsgL6T83ERBta


**Deploy and call EVM contract**

    $ python3 evm-contracts/flipper/deploy.py b5qtfzgaRKhFkGE2FFT5b7ShofrUHXrsSkfsgL6T83ERBta
