# The `dcbor` Command Line Tool

The `dcbor` command line tool is a simple utility that allows you to encode and decode CBOR data using the `dCBOR` library. It is designed to be easy to use and provides a convenient way to work with CBOR data from the command line.

> ✅ **TIP:** This chapter assumes some familiarity with the command line, also called the _shell_. If you're not comfortable using the shell, you may want to refer to the [Command Line Basics](https://www.codecademy.com/learn/learn-the-command-line) course on Codecademy or similar resources.


## Installation

To install the `dcbor` CLI tool, you need to have Rust and Cargo installed on your system. If you don't have them installed, you can follow the instructions on the [Rust website](https://www.rust-lang.org/tools/install) to get started.

Once you have Rust and Cargo installed, you can install the `dcbor` tool by running the following command:

```bash
$ cargo install dcbor-cli
```

You can verify that the installation was successful by running:

```bash
$ dcbor --version
```

You can get help on how to use the `dcbor` tool by running:

```bash
$ dcbor --help
```

This will display the available options and usage information.


## Getting Started

The basic function of the `dcbor` tool is to transform CBOR between various formats and validate it against the deterministic encoding rules.

The three input formats are:

- `diag` - CBOR diagnostic notation
- `hex` - hexadecimal encoding
- `bin` - binary encoding— this is CBOR's native format, but not human-readable

The three output formats are:

- `diag` - CBOR diagnostic notation
- `hex` - hexadecimal encoding
- `bin` - binary encoding— this is CBOR's native format, but not human-readable
- `none` - no output, just validate the input

The default input format is `diag`, and the default output format is `hex`. The hex format is useful for sending to other tools like the `envelope` command line tool we'll discuss in Part III. So by default you put in some human-readable diagnostic notation and get out some CBOR hex:

```bash
$ dcbor 42
182a
```

In the above example, `42` is the string input, and `182a` is the hexadecimal output.

Here's an example with a floating-point number:

```bash
$ dcbor 3.14
fb40091eb851eb851f
```


## Quoting Input

For simple cases like the ones above, you don't need to do anything special with quoting the input. But in many cases you'll need to understand how the shell's use of quotes interacts with CBOR diagnostic notation.

For example, in CBOR diagnostic notation, a string is quoted with double quotes, so for the `dcbor` tool to recognize it as a string, you need to include the double quotes in the input. But in the shell, double quotes are already used to quote the entire argument. If you naively run this command you'll get an error:

```bash
$ dcbor "Hello"
Error: line 1: Unrecognized token
Hello
^
```

This is because the shell is interpreting the double quotes as something to be stripped off. To get around this, you can use single quotes to quote the entire argument:

```bash
$ dcbor '"Hello"'
6548656c6c6f
```

Another option is to escape the inner double quotes with a backslash:

```bash
$ dcbor "\"Hello\""
6548656c6c6f
```

But the most general and flexible way is to use the _here document_ ("_heredoc_") shell feature. This allows you to pass a block of text to the `dcbor` tool without worrying about quoting:

```bash
$ dcbor <<EOF
"Hello"
EOF
6548656c6c6f
```

Notice the first line uses `<<` to indicate the start of a heredoc, which is immediately followed by the delimiter `EOF`. The last line is the same delimiter, which indicates the end of the here document. You can use any string as a delimiter, but `EOF` is a common convention. The here document can also be used to pass as many lines of input as you want. Here is an example of an array of strings spread across multiple lines:

```bash
$ dcbor <<EOF
[
  "Hello",
  "World"
]
EOF
826548656c6c6f65576f726c64
```


## Comments

CBOR diagnostic notation supports two different types of comments: _inline comments_ delimited by `/` and _end-of-line comments_ delimited by `#`. The `dcbor` tool ignores comments in the input, so you can use them freely to annotate your input. Here is an example of both types of comments:

```bash
$ dcbor <<EOF
[ # Start of array
  / First element / "Hello",
  / Second element / "World"
] # End of array
EOF
826548656c6c6f65576f726c64
```

This example produces the same output as the previous example, but it includes comments to explain what each part of the input is doing.


## Round-Trip Conversion

Now that we have an example of hex-encoded CBOR, let's see how to convert it back to diagnostic notation. The `dcbor` tool can do this by specifying the input format as `hex` and the output format as `diag`:

```bash
$ dcbor -i hex -o diag 826548656c6c6f65576f726c64
["Hello", "World"]
```

> ✅ **TIP:** The `-i` and `-o` flags are short for `--in` and `--out`, respectively. You can use the full names if you prefer, but the short names are more convenient for quick commands.

> ⚠️ **WARNING:** CBOR diagnostic notation looks a lot like JSON, but as its name suggests, it is for use during development and debugging, while CBOR is the actual binary encoding format. Diagnostic notation is not optimized for size or speed, and may not be perfectly compatible across implementations. So don't use diagnostic notation in production code.


## Supported Data Types

The `dcbor` tool parses a variety of data types, including the primitives defined in the CBOR specification. It also supports specialized types like Uniform Resources (URs) and Known Values that can be quite useful when working with Gordian Envelope, which we'll discuss in Part III. Here is a summary of the supported data types:

| Type                | Example                                                     |
| ------------------- | ----------------------------------------------------------- |
| Boolean             | `true`<br>`false`                                           |
| Null                | `null`                                                      |
| Integers            | `0`<br>`1`<br>`-1`<br>`42`                                  |
| Floats              | `3.14`<br>`-2.5`<br>`Infinity`<br>`-Infinity`<br>`NAN`      |
| Strings             | `"hello"`<br>`"🌎"`                                          |
| Hex Byte Strings    | `h'68656c6c6f'`                                             |
| Base64 Byte Strings | `b64'AQIDBAUGBwgJCg=='`                                     |
| Tagged Values       | `1234("hello")`<br>`5678(3.14)`                             |
| Name-Tagged Values  | `tag-name("hello")`<br>`tag-name(3.14)`                     |
| Known Values        | `'1'`<br>`'isA'`                                            |
| Unit Known Value    | `Unit`<br>`''`<br>`'0'`                                     |
| URs                 | `ur:date/cyisdadmlasgtapttl`                                |
| Arrays              | `[1, 2, 3]`<br>`["hello", "world"]`<br>`[1, [2, 3]]`        |
| Maps                | `{1: 2, 3: 4}`<br>`{"key": "value"}`<br>`{1: [2, 3], 4: 5}` |

Note these are _input formats_, and not all of them will round-trip to the same output format. For example, the `b64` format can be used to convert existing Base64-encoded data to CBOR, but when output back to diagnostic notation, it will be converted to a `h` format hex byte string.

The `ur` format is a URI encoding of tagged CBOR, and the `dcbor` tool will automatically convert URs it knows about to the appropriate tagged CBOR format. On output, tagged CBOR will appear in the usual `integer(item)` format.

The `dcbor` tool has built-in libraries of name-number correspondences for:

- [CBOR tags/UR types](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2020-006-urtypes.md)
- [Known Values](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2023-002-known-value.md)

Where it can, it will accept these names in place of numeric values.

## Validating Input

Every time the `dcbor` tool is run, it validates the input as valid CBOR, and the dCBOR deterministic encoding rules. If the input is valid, it will produce the output in the specified format. If the input is not valid, it will produce an error message. For example, if we change the first byte of the hex-encoded CBOR from the example above to `0x83` (representing a CBOR array of three elements), we will get an error:

```bash
$ dcbor -i hex -o diag 836548656c6c6f65576f726c64
Error: early end of CBOR data
```

This error message indicates that the input expected a third array element, but it reached the end of the input before finding it. We can "hack" the input to make it valid by adding a third element. Let's just add a `0` to the end of the array:

```bash
$ dcbor -i hex -o diag 836548656c6c6f65576f726c6400
["Hello", "World", 0]
```

This is a valid CBOR array of three elements, and the `dcbor` tool produces the expected output.


## Annotated Output

When producing either hex or diagnostic notation output, the `--annotate` flag can be used to produce output that is even more human-readable. Let's define a shell variable with a hex-encoded CBOR structure, a cryptographic seed:

```bash
$ CBOR_SEED=d99d6ca4015059f2293a5bce7d4de59e71b4207ac5d202c11a6035970003754461726b20507572706c652041717561204c6f766504787b4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e
```

Now we can pass this variable to the `dcbor` by just referring to it as `$CBOR_SEED`:

```bash
$ dcbor --in hex --out diag $CBOR_SEED
40300({1: h'59f2293a5bce7d4de59e71b4207ac5d2', 2: 1(1614124800), 3: "Dark Purple Aqua Love", 4: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."})
```

The output is "flat" diagnostic notation, entirely on one line. This is useful for passing to other tools, but not very human-readable. The `--annotate` flag will produce a more readable output:

```bash
$ dcbor --in hex --out diag --annotate $CBOR_SEED
40300(   / seed /
    {
        1:
        h'59f2293a5bce7d4de59e71b4207ac5d2',
        2:
        1(1614124800),   / date /
        3:
        "Dark Purple Aqua Love",
        4:
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
    }
)
```

Now it's much easier to see that the entire structure is a map with four keys: the first key is a byte string, the second key is a tagged number representing a date, and so on. The entire structure is tagged with `40300`, which is a registered CBOR tag for a cryptographic seed in this format. You can see the definition of this format in the [Blockchain Commons Registry of UR Types](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2020-006-urtypes.md#cryptographic-seed-seed).

The hex output format also supports the `--annotate` flag:

```bash
$ dcbor --in hex --annotate $CBOR_SEED
d9 9d6c                                 # tag(40300) seed
    a4                                  # map(4)
        01                              # unsigned(1)
        50                              # bytes(16)
            59f2293a5bce7d4de59e71b4207ac5d2
        02                              # unsigned(2)
        c1                              # tag(1) date
            1a60359700                  # unsigned(1614124800)
        03                              # unsigned(3)
        75                              # text(21)
            4461726b20507572706c652041717561204c6f7665 # "Dark Purple Aqua Love"
        04                              # unsigned(4)
        78 7b                           # text(123)
            4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e # "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
```

The annotated hex output also uses indentation and comments to make its structure and semantics more clear.

Recall that the default input format is `diag`, and the default output format is `hex`. So in this case we're using `--in` to specify the input format as `hex`, and leaving the output format as the default `hex`, but adding the `--annotate` flag to produce the annotated output.


## Binary Input and Output

CBOR is, after all, a binary format, and `bin` can be used with `--in` or `--out` to specify binary input or output:

```bash
# Write the binary to a file using stdout.
$ dcbor --in hex --out bin $CBOR_SEED >test.bin

# Show the first 48 bytes of the binary file.
(base) ➜  ~ hexdump -C test.bin | head -n 3
00000000  d9 9d 6c a4 01 50 59 f2  29 3a 5b ce 7d 4d e5 9e  |..l..PY.):[.}M..|
00000010  71 b4 20 7a c5 d2 02 c1  1a 60 35 97 00 03 75 44  |q. z.....`5...uD|
00000020  61 72 6b 20 50 75 72 70  6c 65 20 41 71 75 61 20  |ark Purple Aqua |

# The file is exactly 178 bytes long.
wc -c < test.bin | xargs
178

# Read it back in from the file.
$ dcbor --in bin <test.bin
d99d6ca4015059f2293a5bce7d4de59e71b4207ac5d202c11a6035970003754461726b20507572706c652041717561204c6f766504787b4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e73656374657475722061646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e

# Read it in again, but with output in diagnostic notation.
$ dcbor --in bin --out diag <test.bin
40300({1: h'59f2293a5bce7d4de59e71b4207ac5d2', 2: 1(1614124800), 3: "Dark Purple Aqua Love", 4: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."})
```


## A More Complex Example

In [a previous chapter](../part_1/cbor_tags.html#example-tags-in-action) we saw an example of a CBOR structure in diagnostic notation that used a number of different tags and complex types like arrays and maps. Let's see how to encode that structure using the `dcbor` tool:

```bash
$ dcbor <<EOF
{
  "sensorID": 37(h'f81d4fae7dec11d0a76500a0c91e6bf6'),    # Tag 37 for UUID
  "captureTime": 0("2023-10-27T14:30:15.123Z"),           # Tag 0 for RFC3339 string
  "reading": 4([-3, -12345]),                             # Tag 4 for decimal fraction
  "statusURL": 32("https://example.com/status/f81d4fae"), # Tag 32 for URI
  "alertPayload": h'01020304'                             # Direct byte string
}
EOF
a56772656164696e67c482223930386873656e736f724944d82550f81d4fae7dec11d0a76500a0c91e6bf66973746174757355524cd820782368747470733a2f2f6578616d706c652e636f6d2f7374617475732f66383164346661656b6361707475726554696d65c07818323032332d31302d32375431343a33303a31352e3132335a6c616c6572745061796c6f61644401020304
```


## Composing Arrays and Maps

We've seen how to compose arrays and maps in CBOR diagnostic notation:

```bash
# Compose an array of strings:
$ dcbor <<EOF
[ "Hello", "World" ]
EOF
826548656c6c6f65576f726c64

# Compose a map of with string keys and values:
$ dcbor <<EOF
{ "Hello": "World" }
EOF
a26148656c6c6f576f726c64
```

When working with shell scripts, you can interpolate shell variable into the input. For example, let's define a shell variable with a string value:

```bash
$ HELLO="Hello"
$ dcbor <<EOF
[ "$HELLO", "World" ]
EOF
826548656c6c6f65576f726c64
```

This isn't bad when you know the number of elements in the array, but what if you have a variable number of elements? Here's a little script that generates the first 10 Fibonacci numbers and puts them in an array:

```bash
$ FIB=($(awk 'BEGIN{a=1;b=1;for(i=0;i<10;i++){printf "%d ",a; t=a;a=b;b=t+b}}'))
$ echo "${FIB[@]}"
1 1 2 3 5 8 13 21 34 55
```

To get this into a CBOR array, we can use the `array` subcommand of the `dcbor` tool:

```bash
$ FIB_DIAG=`dcbor array --out diag "${FIB[@]}"`
$ echo $FIB_DIAG
[1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
```

The `array` subcommand takes a strings which can be *any* diagnostic notation (not just atomic values like the numbers we're using here), and produces a CBOR array so you don't have to muck about with brackets and commas.

Now let's say you want to use `$FIB_DIAG` in a CBOR map. You can use the `map` subcommand to do this:

```bash
$ FIB_MAP=`dcbor map --annotate --out diag \
    '"name"' '"Fibonacci Numbers"' \
    '"value"' "$FIB_DIAG"`

$ echo $FIB_MAP
{
    "name":
    "Fibonacci Numbers",
    "value":
    [1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
}
```

The `map` subcommand takes a list alternating keys and values. Note how we're being careful to quote the keys and values, with single-quotes being used for strings (so they don't get interpreted by the shell) and double-quotes used for the `$FIB_DIAG` variable (so it gets interpolated).

When we've got our map composed the way we want it, we can serialized it to binary, or hex for use with the `envelope` tool we'll discuss in Part III:

```bash
$ dcbor --in diag --out hex $FIB_MAP
a2646e616d65714669626f6e61636369204e756d626572736576616c75658a0101020305080d1518221837
```
