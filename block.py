import time
import util.crypto_hash as sha
import util.db as db


def compile_tx(txs):
    a = ""
    for i in txs:
        a += str(i)
    return sha.crypto_hash(a)


class Block:
    def __init__(self, prev, version, txs):
        # key
        self.hash = ""
        self.previous = prev
        self.win_attempt = ""
        self.version = version
        self.merkle_root = compile_tx(txs)

        # value
        self.time = int(time.time())
        self.transacions = txs

    def getPlay(self):
        return sha.crypto_hash(str(self.previous)+str(self.version)+str(self.merkle_root)+str(self.time))

    def addWin(self, win):
        self.win_attempt = win
        self.hash = sha.crypto_hash(
            str(self.previous)+str(self.win_attempt)+str(self.version)+str(self.merkle_root)+str(self.time))

    def toDict(self):
        dict = {
            "hash": self.hash,
            "prev": self.previous,
            "win": self.win_attempt,
            "version": self.version,
            "merkle": self.merkle_root,
            "time": self.time,
            "txs": self.transacions
        }
        return dict

    def storeBlock(self):
        db.dbPut(self.hash, self.toDict())
