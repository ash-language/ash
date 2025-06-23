Ash is a dynamic typed language.

Data-types:
- number
- string
- boolean
- null
- undefined
- symbol
- function
- prototype
- object
- array

To getting data-type of an expression, we can use typeof keyword:

```
var example = "A"
var type_ = typeof example //string
```

When data-types is being used as expressions, the data-type of the expressions is symbol.

```
var example = "A"
var type_ = typeof example //string
var type__ = typeof type_ //symbol
```


Data-types can be used as expression in runtime.

```
var type_ = number //symbol
var condition = type_ == typeof 1 //true
```