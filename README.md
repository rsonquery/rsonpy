# rsonpy

`rsonpy` is a lightweight Python wrapper around the Rust library
**rsonpath**, a high-performance engine for evaluating JSONPath queries
over large JSON documents.

The goal of this package is to expose the core functionality of
`rsonpath` to Python with a minimal, easy-to-use API.


## Installation

``` bash
pip install rsonpy
```

## Usage

The wrapper exposes a single function that execute the query 
on the input string and return an iterator of parsed document
selected by the query.


- `loads(s: str, query: str) -> Iterator` Run a Rsonpath query on a JSON string.
- `load(input_file: str, query: str) -> Iterator`: Run a Rsonpath query on a JSON file.


### Basic Example

``` python
import json
from rsonpy import loads

doc = {
  "store": {
    "book": [
      {"title": "The Hobbit", "price": 10.0},
      {"title": "The Silmarillion", "price": 12.0},
      {"title": "Unfinished Tales", "price": 9.0}
    ]
  }
}

query = "$.store.book[*].title"

for item in loads(json.dumps(doc), query):
    print(item)
```


Or assuming the file is in `test.json`:

```python3
for item in load("test.json", query):
    print(item)
``` 

**Output:**

    The Hobbit
    The Silmarillion
    Unfinished Tales

## How It Works

Rsonpy exposes two functions, loads and load, which both return Python
iterators over results of a Rsonpath query:

- loads(s, query) operates on a JSON string already in memory. It returns a
  list of (start, end) slices pointing into the string, so the memory footprint
  is minimal regardless of the input size.

- load(input_file, query) operates on a JSON file using memory-mapped I/O
  (mmap). While this avoids copying the full file initially, the entire result
  set is loaded into main memory as a list of Python objects, which can be
  problematic if the query matches many elements or the file is very large.

Internally, rsonpath returns slices (start, end) corresponding to matching
segments of the JSON input. The wrapper decodes each slice using json.loads()
(or a custom loader if provided).

With loads, parsing is fast and predictable since the input string is already
in memory.

With load, performance depends on the system’s mmap efficiency, but memory
usage grows with the size of the result set.

This approach avoids reparsing the entire document multiple times while
delivering high performance on large or deeply nested JSON structures, as long
as the result set can fit in RAM.

Adaptation to expose the iterator is possible but would require more work.
