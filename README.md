The rsonpy Python module is a bridge to the Rust library rsonpath, which allows
for efficient execution of JSONPath queries on JSON data. 
It relies on SIMD-optimization to be blazing fast.

The interface try as much as possible to minimize overhead and to be copyless
and performant.

#  Simple Usage 

To maximize the performance and minimize overhead, provides
a filepath:

```python
import rsonpy
J = rsonpy.load("path/to/some/file.json", "$..only.a.part")
n = len(J) # give the size of the result
for doc in J:
    # do something with it
```

The return object is a Sized Iterable. Each operation (`iter` and `len`)
will trigger the rsonpath-lib code and go through the whole file.

The set of results will be stored in RAM before being emitted so if its too big
some memory error can be thrown.  The `len` is mostly memoryless and could
always be used to check nothing wrong is happening.

It is possible to provide a string already in RAM:

```python
J = rsonpy.loads(some_string, "$..only.a.part")
n = len(J) # give the size of the result
for doc in J:
    # do something with it
```

The main difference is that `some_string` will be copied in RAM once
to avoid alignement issue. This behaviour will hopefully be removed
in the future.

# Advance usage

## Preparsing queries

It is possible to avoid parsing the query (which avoid a bit of overhead):

```python
q = rsonpy.Query("$..only.a.part")
q.loads(some_string)
q.load("path/to/some/file.json")
``` 

The compilation and optimization of the query is for now still performed
at the execution. This behaviour will hopefully change in the future and the gain
more interesting.


## Changing the python parser

The rsonpath engine select part of a string matching some results but it
isn't parsing the document. Those selected substring are then parsed
using the standard `json` module. For large results set, this parsing
can become the main bottleneck. You can change it to more efficient
parsing tools or even replace it by `lambda e:e` to get the string
instead.

## Quick efficency show off (WIP)

Let say you want to check the number of keys at the root
of a document.

In python you would do:

```python
def doclen(path):
    with open(path) as f:
        return len(json.load(f))
```

With our rsonpy simply:

```python
def doclen_rsonpy(path):
    return len(rsonpy.load(path, "$.*"))
``` 

With a synthetic document generated as follows:

```python
import json
from functools import lru_cache
@lru_cache # to avoid useless exponential blowup
def rec(i, verbosity=10):
    if not i:
        return " some long value "*verbosity
    return {f"label_{j}": rec(i-1, verbosity=verbosity) for j in range(i)}
    
with open("/tmp/test.json", "w") as f:
    json.dump(rec(20), f)
```

We get a json document of 865Mb.

The function `doclen` take 2s on my laptop while `doclen_rsonpy`
only 4ms.

Loading part of the document can be much faster as well and it isn't really
related to the efficiency of the json module. For instance, simply
loading the file into memory take 15 ms.

