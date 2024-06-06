
# `serde_tran`: helper to encode/decode your JSON data

## Background
When we use the HTTP protocol (or other transmission protocols),
it is convenient and common to use the Json data format to store
it in the HTTP request body or response body.

However, when we transmit a large amount of data (for example,
a large array, and the key in the Map type data is relatively
long), it is not efficient to use Json data directly.

Therefore, you can consider using this library. This library will
help you encapsulate Json type data (that is, `impl Serialize`)
with an extra layer (the result is still Json, but the data is more compact).

## Encoding Example
This library will encode:

```json
{
  "username": "serde_tran:username",
  "password": "serde_tran:password:123456"
}
```

to something like:
```json
{
  "f": "base64",
  "v": "NgAAAAAAAAATAAAAAAAAAHNlcmRlX3RyYW46dXNlcm5hbWUTAAAAAAAAAHNlcmRlX3RyYW46cGFzc3dvcmRnmz7nMG94SA"
}
```

## Usage

First, we define a struct in our example, which `impl Serialize`:

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MyStruct {
    // fields...    
}
```

```rust
pub fn example() {
    let s = MyStruct {};
    
    // 1. convert it to serde_tran::Json
    let json = serde_tran::to_json(&s).unwrap();
    
    // json is something like: {"f": "base64", "v": "(base64 encoded string)"}
    // you can use json.to_string() to get the json string.
    
    // 2. convert it back to MyStruct
    let ds: MyStruct = json.to_value().unwrap();
    
    assert_eq!(s, ds); // they are the same (remember to derive PartialEq to use macro assert_eq!)
}
```

More examples, see the folder `examples`.

## How it works

This picture shows all the public function from `serde_tran`:

![](./structure.jpeg)
 
## Features

+ `serde_json (default)`: enable `Json`. If you use `serde_json`, `base58` or `base64` must be enabled (at least one).
+ `base64 (default)`: enable base64 encoding, using crate (base64)[https://docs.rs/base64/latest/base64].
+ `bs58`: enable base58 encoding, using crate (bs58)[https://docs.rs/bs58]. Warning: this encoding is slow.
+ `flate2`: enable gzip, about 10+ times slower (compared to `serde_json`).

## Wasm
use `wasm` in your web client.

> Work in progress

## Benchmark

+ `n`: the length of array
+ `sl`: `string_length`, the length of the string of the elements in the array

Summary: It saves 35%~50% of the space.

```text
n=1, sl=10, 
	json.time=3000ns/op, json.space=816bytes/bench, 
	tran.time=2000ns/op, tran.space=488bytes/bench
	[tran/json] time=-33.33333333333333%, space=-40.19607843137255%
n=1, sl=200, 
	json.time=13000ns/op, json.space=11328bytes/bench, 
	tran.time=13000ns/op, tran.space=7128bytes/bench
	[tran/json] time=0%, space=-37.07627118644068%
n=1, sl=400, 
	json.time=20000ns/op, json.space=22221bytes/bench, 
	tran.time=23000ns/op, tran.space=14017bytes/bench
	[tran/json] time=15%, space=-36.92003060168309%
n=10, sl=10, 
	json.time=10000ns/op, json.space=8276bytes/bench, 
	tran.time=7000ns/op, tran.space=4652bytes/bench
	[tran/json] time=-30%, space=-43.78927017883035%
n=10, sl=200, 
	json.time=118000ns/op, json.space=112964bytes/bench, 
	tran.time=117000ns/op, tran.space=70636bytes/bench
	[tran/json] time=-0.847457627118644%, space=-37.47034453454198%
n=10, sl=400, 
	json.time=217000ns/op, json.space=223148bytes/bench, 
	tran.time=227000ns/op, tran.space=140019bytes/bench
	[tran/json] time=4.6082949308755765%, space=-37.25285460770431%
n=100, sl=10, 
	json.time=83000ns/op, json.space=81750bytes/bench, 
	tran.time=72000ns/op, tran.space=45460bytes/bench
	[tran/json] time=-13.253012048192772%, space=-44.391437308868504%
n=100, sl=200, 
	json.time=1004000ns/op, json.space=1128094bytes/bench, 
	tran.time=2115000ns/op, tran.space=705407bytes/bench
	[tran/json] time=110.65737051792827%, space=-37.46912934560418%
n=100, sl=400, 
	json.time=3192000ns/op, json.space=2231179bytes/bench, 
	tran.time=3775000ns/op, tran.space=1402308bytes/bench
	[tran/json] time=18.264411027568922%, space=-37.14946223498877%
n=1024, sl=10, 
	json.time=1169000ns/op, json.space=837345bytes/bench, 
	tran.time=1086000ns/op, tran.space=466460bytes/bench
	[tran/json] time=-7.100085543199315%, space=-44.29297362496939%
n=1024, sl=200, 
	json.time=11045000ns/op, json.space=11554178bytes/bench, 
	tran.time=13268000ns/op, tran.space=7228320bytes/bench
	[tran/json] time=20.12675418741512%, space=-37.4397728683079%
n=1024, sl=400, 
	json.time=20444000ns/op, json.space=22834571bytes/bench, 
	tran.time=25655000ns/op, tran.space=14344660bytes/bench
	[tran/json] time=25.489141068284095%, space=-37.18007664781616%
n=2048, sl=10, 
	json.time=1811000ns/op, json.space=1674880bytes/bench, 
	tran.time=1890000ns/op, tran.space=932763bytes/bench
	[tran/json] time=4.362230811706239%, space=-44.30866688956821%
n=2048, sl=200, 
	json.time=20665000ns/op, json.space=23114902bytes/bench, 
	tran.time=25878000ns/op, tran.space=14454992bytes/bench
	[tran/json] time=25.22622792160658%, space=-37.464619144827005%
n=2048, sl=400, 
	json.time=42064000ns/op, json.space=45670596bytes/bench, 
	tran.time=51634000ns/op, tran.space=28690505bytes/bench
	[tran/json] time=22.7510460251046%, space=-37.17948195815093%
n=5120, sl=10, 
	json.time=4211000ns/op, json.space=4189273bytes/bench, 
	tran.time=4124000ns/op, tran.space=2331513bytes/bench
	[tran/json] time=-2.066017573023035%, space=-44.345641833320485%
n=5120, sl=200, 
	json.time=52865000ns/op, json.space=57772057bytes/bench, 
	tran.time=63971000ns/op, tran.space=36140128bytes/bench
	[tran/json] time=21.00822850657335%, space=-37.44358453430176%
n=5120, sl=400, 
	json.time=102073000ns/op, json.space=114183541bytes/bench, 
	tran.time=129650000ns/op, tran.space=71728965bytes/bench
	[tran/json] time=27.01693885748435%, space=-37.18099441319656%
n=10240, sl=10, 
	json.time=9128000ns/op, json.space=8376513bytes/bench, 
	tran.time=8968000ns/op, tran.space=4665019bytes/bench
	[tran/json] time=-1.7528483786152498%, space=-44.308341669140844%
n=10240, sl=200, 
	json.time=104960000ns/op, json.space=115546296bytes/bench, 
	tran.time=138643000ns/op, tran.space=72273849bytes/bench
	[tran/json] time=32.09127286585366%, space=-37.45031082606058%
n=10240, sl=400, 
	json.time=208326000ns/op, json.space=228338452bytes/bench, 
	tran.time=296174000ns/op, tran.space=143440585bytes/bench
	[tran/json] time=42.16852433205649%, space=-37.180714091904235%
```
