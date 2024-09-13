from contextlib import contextmanager
from logging import Logger

import time

@contextmanager
def timed(name: str):
    ts = time.time()
    try:
        yield ts
    finally:
        tf = time.time()
        print(f"{name} runned in {round(tf-ts, 4)}s")
