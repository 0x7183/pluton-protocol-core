import time
import base64
import sys
from terra_sdk.key.mnemonic import MnemonicKey
from terra_sdk.client.lcd import LCDClient
from terra_sdk.client.lcd.api.tx import CreateTxOptions
from terra_sdk.core.wasm import MsgStoreCode, MsgInstantiateContract, MsgExecuteContract



def initialize(wallet, terra, wasmfile, path):

    file = open(path +"/initialize.txt", "r")
    initializemsg= eval(file.read())[0]
    file.close()

    print("[+] Storing contract")

    contract_file = open(wasmfile, "rb")
    file_bytes = base64.b64encode(contract_file.read()).decode()
    store_code = MsgStoreCode(wallet.key.acc_address, file_bytes)
    store_code_tx = wallet.create_and_sign_tx(CreateTxOptions(msgs=[store_code]))
    store_code_tx_result = terra.tx.broadcast(store_code_tx)
    print("[*] Contract stored")

    code_id = store_code_tx_result.logs[0].events_by_type["store_code"]["code_id"][0]

    time.sleep(10)

    print("[+] Instantiate contract")
    instantiate = MsgInstantiateContract(
        wallet.key.acc_address,
        wallet.key.acc_address,
        code_id,
        initializemsg
    )
    instantiate_tx = wallet.create_and_sign_tx(CreateTxOptions(msgs=[instantiate]))
    instantiate_tx_result = terra.tx.broadcast(instantiate_tx)
    print("[*] Contract instantiated")

    contract_address = instantiate_tx_result.logs[0].events_by_type[
        "instantiate_contract"
        ]["contract_address"][0]

    print("[*] Contract address:" + str(contract_address))

    return contract_address

def debugSC(wallet, terra, contract_address, path):

    file = open(path + "/execute.txt", "r")
    executeFile = eval(file.read())
    file.close()

    file = open(path + "/query.txt", "r")
    queryFile = eval(file.read())
    file.close()
    print("[+] Testing ExecuteMsg")

    for msg in executeFile:

        execute = MsgExecuteContract(
            wallet.key.acc_address,
            contract_address,
            msg.get("msg"),
            msg.get("coin")
        )

        execute_tx = wallet.create_and_sign_tx(CreateTxOptions(msgs=[execute]))

        execute_tx_result = terra.tx.broadcast(execute_tx)
        print("\n\n[*] ExecuteMsg:\n" + str(msg.get("msg")) + "\nResult\n" + str(execute_tx_result))
        time.sleep(10)

    
    print("\n\n[+] Testing QueryMsg")

    for query in queryFile:

        result = terra.wasm.contract_query(contract_address, query)
        time.sleep(10)
        print("\n[*] Query:\n" + str(query) + "\nResult:\n" + str(result))


if __name__ == "__main__":

    wasmfile = sys.argv[1]
    path = sys.argv[2]

    # Create client to communicate with testnet.
    terra = LCDClient(
        url="https://bombay-lcd.terra.dev/",
        chain_id="bombay-12"
    )

    # Initialize wallet with associated mnemonic key.
    mk = MnemonicKey(mnemonic="decline ignore great ostrich piano torch whip scorpion actor hard path riot ancient sleep zero dial present insane vivid embark combine pulse latin tuition")
    wallet = terra.wallet(mk)

    # Upload ad initialize contract
    contract_address = initialize(wallet, terra, wasmfile, path)
    
    # Test contract
    debugSC(wallet, terra, contract_address, path)

    print("[-] Ended check results")
