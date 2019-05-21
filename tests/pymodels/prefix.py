import sys
import time
import os


def data(k):
    while True:
        print(k)
        time.sleep(10)
        print(sys.prefix, os.getpid())
