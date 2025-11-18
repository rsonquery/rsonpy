from rsonpy import rsonpy as rsonpath
import json


def loads(s: str, query: str, json_loader=json.loads):
    """
    Parse the input string that is assumed to be a valid JSON-encoding and return an iterator
    over solutions of the query over the document.
    """
    for start, end in rsonpath.loads(s, query):
        yield json.loads(s[start:end])


__doc__ = rsonpath.__doc__
if hasattr(rsonpath, "__all__"):
    __all__ = rsonpath.__all__
