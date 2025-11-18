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


``` python
def loads(s: str, query: str, json_loader=json.loads) -> Iterato:
```

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

**Output:**

    The Hobbit
    The Silmarillion
    Unfinished Tales

## How It Works

`rsonpath` returns slices `(start, end)` corresponding to matching
segments of the JSON input string.\
The wrapper simply decodes each slice using `json.loads()` (or a custom
loader if provided).

This approach avoids reparsing the entire document multiple times and
provides excellent performance on large or deeply nested JSON
structures.
