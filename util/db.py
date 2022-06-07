import this
import leveldb
import hashlib
import pickle


def dbPut(a, b):
    # a key
    # b value
    db = leveldb.LevelDB('./db')
    a = pickle.dumps(a)
    b = pickle.dumps(b)
    db.Put(a, b)
    return


def dbGet(a):
    try:
        db = leveldb.LevelDB('./db')
        a = pickle.dumps(a)
        return pickle.loads(db.Get(a))
    except KeyError:
        return("")


def dbDelete(a):
    db = leveldb.LevelDB('./db')
    a = pickle.dumps(a)
    db.Delete(a)
    return


dict = {
    "hash": "81ddc8d248b2dccdd3fdd5e84f0cad62b08f2d10b57f9a831c13451e5c5c80a5",
    "difficulty": 5,
    "prev": "",
}

dbPut(1, dict)
