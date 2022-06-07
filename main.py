from hashlib import new
from util.crypto_hash import crypto_hash
from util.hex_to_binary import hex_to_binary
import block
import util.db as db

block = block.Block(
    '6B51D431DF5D7F141CBECECCF79EDF3DD861C3B4069F0B11661A3EEFACBBA918', 2.0, [3, 4])
block.addWin('6B51D431DF5D7F141CBECECCF79EDF3DD861C3B4069F0B11661A3EEFACBBA918')
block.storeBlock()
print(db.dbGet(block.hash))
