from time import time

def compile_tx(txs):
    

class Block:
    def __init__(self, prev, version, txs, win):
        //key
        self.previous = prev
        self.version = version
        self.win_attempt = win
        self.merkle_root = compile_tx(txs)

        //value
        self.time = int(time.time())
        self.transacions = txs