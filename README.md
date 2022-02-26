# rust-autocomplete-poc
*Fastest autocomplete in rust with O(1) complexity*

## Backstory

On my daily job I'm working with autocomplete and I'm using [WFST](https://lucene.apache.org/core/6_1_0/suggest/org/apache/lucene/search/suggest/fst/WFSTCompletionLookup.html) provided by [Lucene](https://lucene.apache.org/). 

WFSTs are awesome! They are fast and provide really impressive compression! But one day I was thinking how can I make autocomplete even faster?

And I've had this dumb idea:
**What is the fastest thing in the universe?**

Correct! it's a `.get` method from the HashMap! and it also has `O(1)` complexity so it should scale really well. 

**But how to implement prefix-lookup in the HashMap?** 
you might ask. 

So here is the dumb part, you don't need to do it, you can prepare keys that will already contain all possible prefixes.

**Let's follow my example here**
To start with autocomplete you usually have some weighted string, weight is determine how the query is relevant. 

My weighted strings: 
```text
pizza   4
pie     2
```

My HashMap:
```bash
{
    "p": ["pizza", "pie"],
    'pi': ['pizza', 'pie'],
    'piz': ['pizza'],
    'pizz': ['pizza'],
    'pizza': ['pizza'],
    'pie': ['pie']}
}
```

You see that it's not quite your normal HashMap since it contains multiple values for a given key. 
In python you can modify Python's dict like this:
```
class Dictlist(dict):
    def __setitem__(self, key, value):
        try:
            self[key]
        except KeyError:
            super(Dictlist, self).__setitem__(key, [])
        self[key].append(value)

```

In Rust I used [multimap crate](https://crates.io/crates/multimap).

## Running this code:
1. clone it
2. run it with `cargo run` or buid it and run from binary
![gif](demo.gif)
