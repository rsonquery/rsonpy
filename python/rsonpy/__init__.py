from rsonpy import rsonpy as rust_rson
import json
from collections.abc import Iterable, Sized
from pathlib import Path
from typing import Union

class Query:
    def __init__(self, query: str, python_parser=json.loads):
        self._query_str = query
        self._query = rust_rson.PyQuery(query)
        self.python_parser = python_parser

    def load(self, file: Path):
        return JsonResults(self, file=file, python_parser=self.python_parser)

    def loads(self, input_str: str):
        return JsonResults(self, input_str=input_str, python_parser=self.python_parser)

    def __repr__(self):
        return f"Query<{self._query_str}>"

class JsonResults(Iterable, Sized):
    def __init__(self, query:Query, file:Path=None, input_str: str=None, python_parser=json.loads):
        if not (file or input_str):
            raise ValueError("A JsonResult should be provided either a file or an input string")
        if file:
            file = Path(file)

        self.file = file
        self.input_str = input_str
        self.query = query
        self.python_parser = python_parser

    def __iter__(self):
        if self.file:
            if not self.file.exists():
                raise FileNotFoundError(self.file)
            for es in self.query._query.load(str(self.file)):
                yield self.python_parser(es)
        else:
            for start, end in self.query._query.loads(self.input_str):
                yield self.python_parser(self.input_str[start:end])

    def __len__(self):
        if self.file:
            return self.query._query.count_f(self.file)
        else:
            return self.query._query.count_s(self.input_str)
    

def load(file: Path, query: Union[Query, str]):
    if isinstance(query, str):
        query = Query(query)
    return query.load(file)

def loads(input_str: str, query: Union[Query, str]):
    if isinstance(query, str):
        query = Query(query)
    return query.loads(input_str)


__doc__ = rsonpy.__doc__
if hasattr(rsonpy, "__all__"):
    __all__ = rsonpy.__all__
