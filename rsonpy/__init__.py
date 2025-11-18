from rsonpy import rsonpy as rsonpath
import json
from pathlib import Path


def loads(s: str, query: str, json_loader=json.loads):
    """
    Parse the input string that is assumed to be a valid JSON-encoding and return an iterator
    over solutions of the query over the document.
    """
    for start, end in rsonpath.loads(s, query):
        yield json.loads(s[start:end])


def load(input_file: str, query: str, json_loader=json.loads):
    """
    Parse the input string that is assumed to be a valid JSON-encoding and return an iterator
    over solutions of the query over the document.
    """
    if not Path(input_file).exists():
        raise FileNotFoundError(input_file)

    for u in rsonpath.load(input_file, query):
        yield json.loads(u.decode())


__doc__ = rsonpath.__doc__
if hasattr(rsonpath, "__all__"):
    __all__ = rsonpath.__all__
