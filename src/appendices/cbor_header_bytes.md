# CBOR Header Bytes

This table shows all possible CBOR header byte values and their meanings.

|      | `_0`      | `_1`     | `_2`      | `_3`      | `_4`      | `_5`      | `_6`      | `_7`      | `_8`        | `_9`        | `_a`        | `_b`        | `_c`      | `_d`      | `_e`      | `_f`         |
|------|----------:|---------:|----------:|----------:|----------:|----------:|----------:|----------:|------------:|------------:|------------:|------------:|----------:|----------:|----------:|-------------:|
| `0_` | `0`       | `1`      | `2`       | `3`       | `4`       | `5`       | `6`       | `7`       | `8`         | `9`         | `10`        | `11`        | `12`      | `13`      | `14`      | `15`         |
| `1_` | `16`      | `17`     | `18`      | `19`      | `20`      | `21`      | `22`      | `23`      | `int 1+1`   | `int 1+2`   | `int 1+4`   | `int 1+8`   |           |           |           |              |
| `2_` | `-1`      | `-2`     | `-3`      | `-4`      | `-5`      | `-6`      | `-7`      | `-8`      | `-9`        | `-10`       | `-11`       | `-12`       | `-13`     | `-14`     | `-15`     | `-16`        |
| `3_` | `-17`     | `-18`    | `-19`     | `-20`     | `-21`     | `-22`     | `-23`     | `-24`     | `neg 1+1`   | `neg 1+2`   | `neg 1+4`   | `neg 1+8`   |           |           |           |              |
| `4_` | `bstr 0`  | `bstr 1` | `bstr 2`  | `bstr 3`  | `bstr 4`  | `bstr 5`  | `bstr 6`  | `bstr 7`  | `bstr 8`    | `bstr 9`    | `bstr 10`   | `bstr 11`   | `bstr 12` | `bstr 13` | `bstr 14` | `bstr 15`    |
| `5_` | `bstr 16` | `bstr 17`| `bstr 18` | `bstr 19` | `bstr 20` | `bstr 21` | `bstr 22` | `bstr 23` | `bstr 1+1`  | `bstr 1+2`  | `bstr 1+4`  | `bstr 1+8`  |           |           |           | `bstr indef` |
| `6_` | `str 0`   | `str 1`  | `str 2`   | `str 3`   | `str 4`   | `str 5`   | `str 6`   | `str 7`   | `str 8`     | `str 9`     | `str 10`    | `str 11`    | `str 12`  | `str 13`  | `str 14`  | `str 15`     |
| `7_` | `str 16`  | `str 17` | `str 18`  | `str 19`  | `str 20`  | `str 21`  | `str 22`  | `str 23`  | `str 1+1`   | `str 1+2`   | `str 1+4`   | `str 1+8`   |           |           |           | `str indef`  |
| `8_` | `arr 0`   | `arr 1`  | `arr 2`   | `arr 3`   | `arr 4`   | `arr 5`   | `arr 6`   | `arr 7`   | `arr 8`     | `arr 9`     | `arr 10`    | `arr 11`    | `arr 12`  | `arr 13`  | `arr 14`  | `arr 15`     |
| `9_` | `arr 16`  | `arr 17` | `arr 18`  | `arr 19`  | `arr 20`  | `arr 21`  | `arr 22`  | `arr 23`  | `arr 1+1`   | `arr 1+2`   | `arr 1+4`   | `arr 1+8`   |           |           |           | `arr indef`  |
| `a_` | `map 0`   | `map 1`  | `map 2`   | `map 3`   | `map 4`   | `map 5`   | `map 6`   | `map 7`   | `map 8`     | `map 9`     | `map 10`    | `map 11`    | `map 12`  | `map 13`  | `map 14`  | `map 15`     |
| `b_` | `map 16`  | `map 17` | `map 18`  | `map 19`  | `map 20`  | `map 21`  | `map 22`  | `map 23`  | `map 1+1`   | `map 1+2`   | `map 1+4`   | `map 1+8`   |           |           |           | `map indef`  |
| `c_` | `tag 0`   | `tag 1`  | `tag 2`   | `tag 3`   | `tag 4`   | `tag 5`   | `tag 6`   | `tag 7`   | `tag 8`     | `tag 9`     | `tag 10`    | `tag 11`    | `tag 12`  | `tag 13`  | `tag 14`  | `tag 15`     |
| `d_` | `tag 16`  | `tag 17` | `tag 18`  | `tag 19`  | `tag 20`  | `tag 21`  | `tag 22`  | `tag 23`  | `tag 1+1`   | `tag 1+2`   | `tag 1+4`   | `tag 1+8`   |           |           |           |              |
| `e_` | `val 0`   | `val 1`  | `val 2`   | `val 3`   | `val 4`   | `val 5`   | `val 6`   | `val 7`   | `val 8`     | `val 9`     | `val 10`    | `val 11`    | `val 12`  | `val 13`  | `val 14`  | `val 15`     |
| `f_` | `val 16`  | `val 17` | `val 18`  | `val 19`  | `false`   | `true`    | `null`    | `undef`   | `val 1+1`   | `float 16`  | `float 32`  | `float 64`  |           |           |           | `break`      |

**Legend:**

- `1+1` = 1 header byte + 1 data byte (24...255)
- `1+2` = 1 header byte + 2 data bytes (256...65535)
- `1+4` = 1 header byte + 4 data bytes
- `1+8` = 1 header byte + 8 data bytes
- `int` = non-negative integer
- `neg` = negative integer
- `bstr` = byte string + length
- `str` = UTF-8 text string + length
- `arr` = array + length
- `map` = map + length
- `tag` = semantic tag + value
- `val` = simple value
- `false` = simple value 20
- `true` = simple value 21
- `null` = simple value 22
- `undef` = simple value 23
- `float 16/32/64` = half/single/double precision float
- `indef` = indefinite length
- `break` = stop code for indefinite items
