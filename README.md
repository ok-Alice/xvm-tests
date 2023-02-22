# xvm-tests

## Ink -> EVM 

**Deploy the Ink Contract**

    $ python3 wasm-contracts/flipper/deploy.py

And store the contract address, e.g: b5qtfzgaRKhFkGE2FFT5b7ShofrUHXrsSkfsgL6T83ERBta

**Convert contract address**

    ./astar-collator key inspect b5qtfzgaRKhFkGE2FFT5b7ShofrUHXrsSkfsgL6T83ERBta
      Public Key URI `b5qtfzgaRKhFkGE2FFT5b7ShofrUHXrsSkfsgL6T83ERBta` is account:
        Network ID/Version: astar
        Public key (hex):   0xe3b1801e8a98b73023fc6ca588c37acd475d9a52f06db0139585757bc03fd62a
        Account ID:         0xe3b1801e8a98b73023fc6ca588c37acd475d9a52f06db0139585757bc03fd62a
        Public key (SS58):  b5qtfzgaRKhFkGE2FFT5b7ShofrUHXrsSkfsgL6T83ERBta
        SS58 Address:       b5qtfzgaRKhFkGE2FFT5b7ShofrUHXrsSkfsgL6T83ERBta


**Deploy and call EVM contract**

    $ python3 evm-contracts/flipper/deploy.py 0xe3b1801e8a98b73023fc6ca588c37acd475d9a52f06db0139585757bc03fd62a
