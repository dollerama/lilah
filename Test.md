# Test
### Modules
> - [io](#module-io)
## Module ``io``
### Classes
> - [Fs](#io--k-1)
> - [Serializable](#io--k0)
> - [Json](#io--k1)
> - [JsonStringify](#io--k2)
> - [JsonParser](#io--k3)
### Foreign Class `` <a id='Fs'></a> ``
>

#### Methods
> - [read](#io-0-m-1)
> - [write](#io-0-m0)
##### Foreign Static Method ``read(file: _)`` <a id='io-0-m-1'></a>
``return _``

##### Foreign Static Method ``write(file: _, content: _)`` <a id='io-0-m0'></a>
``return _``

### Class ``Serializable`` <a id='io--k0'></a>
>

#### Methods
> - [//example Serializable.wrapper](#io-1-m-1)
> - [wrapperFn](#io-1-m0)
> - [wrapper](#io-1-m1)
> - [properties](#io-1-m2)
> - [serialize](#io-1-m3)
> - [serialize](#io-1-m4)
> - [deserialize](#io-1-m5)
> - [iterProperties](#io-1-m6)
##### Method ``//example Serializable.wrapper({"math": "Vec2"}: _, "Rect": _, [["pos": _, Vec2]: _, ["size": _, Vec2]]: _)`` <a id='io-1-m-1'></a>
``return _``

##### Static Method ``wrapperFn(imports: _, name: _, values: _)`` <a id='io-1-m0'></a>
``return _``

##### Static Method ``wrapper(imports: _, name: _, values: _)`` <a id='io-1-m1'></a>
``return _``

##### Method ``properties(f: _)`` <a id='io-1-m2'></a>
``return _``

##### Static Method ``serialize(obj: _)`` <a id='io-1-m3'></a>
``return _``

##### Method ``serialize()`` <a id='io-1-m4'></a>
``return _``

##### Method ``deserialize(obj: _)`` <a id='io-1-m5'></a>
``return _``

##### Static Method ``iterProperties(t: _)`` <a id='io-1-m6'></a>
``return _``

### Class ``Json`` <a id='io--k1'></a>
>

#### Methods
> - [parse(string) { parse](#io-2-m-1)
> - [parse](#io-2-m0)
> - [stringify(value) { stringify](#io-2-m1)
> - [stringify](#io-2-m2)
> - [stringify](#io-2-m3)
##### Static Method ``parse(string) { parse(string) { parse("json": _, string: _)`` <a id='io-2-m-1'></a>
``return _``

##### Static Method ``parse(source_id: _, source_string: _)`` <a id='io-2-m0'></a>
``return _``

##### Static Method ``stringify(value) { stringify(value) { stringify(value: _, "  ": _)`` <a id='io-2-m1'></a>
``return _``

##### Static Method ``stringify(value: _, whitespace: _)`` <a id='io-2-m2'></a>
``return _``

##### Static Method ``stringify(value: _, whitespace: _, callback: _)`` <a id='io-2-m3'></a>
``return _``

### Class ``JsonStringify`` <a id='io--k2'></a>
>

#### Methods
> - [stringify](#io-3-m-1)
> - [stringify_map](#io-3-m0)
> - [stringify_primitive](#io-3-m1)
> - [stringify_list](#io-3-m2)
> - [stringify_value](#io-3-m3)
##### Static Method ``stringify(value: _, whitespace: _, out: _)`` <a id='io-3-m-1'></a>
``return _``

##### Static Method ``stringify_map(map: _, whitespace: _, depth: _, out: _)`` <a id='io-3-m0'></a>
``return _``

##### Static Method ``stringify_primitive(value: _, out: _)`` <a id='io-3-m1'></a>
``return _``

##### Static Method ``stringify_list(list: _, whitespace: _, depth: _, out: _)`` <a id='io-3-m2'></a>
``return _``

##### Static Method ``stringify_value(value: _, whitespace: _, depth: _, out: _)`` <a id='io-3-m3'></a>
``return _``

### Class ``JsonParser`` <a id='io--k3'></a>
>

#### Constructors
> - [new](#io-4-c-1)
#### Getters
> - [root](#io-4-g-1)
#### Methods
> - [unexpected](#io-4-m-1)
> - [is_eof](#io-4-m0)
> - [is_whitespace](#io-4-m1)
> - [is_token](#io-4-m2)
> - [next](#io-4-m3)
> - [peek() { peek](#io-4-m4)
> - [peek](#io-4-m5)
> - [peeks() { peeks](#io-4-m6)
> - [peeks](#io-4-m7)
> - [step](#io-4-m8)
> - [skips](#io-4-m9)
> - [parse_key](#io-4-m10)
> - [parse_primitive](#io-4-m11)
> - [read_raw_string](#io-4-m12)
> - [read_string](#io-4-m13)
> - [parse_string](#io-4-m14)
> - [parse_value](#io-4-m15)
> - [parse_list](#io-4-m16)
> - [parse_map](#io-4-m17)
> - [parse_map_value](#io-4-m18)
##### Getter ``root`` <a id='io-4-g-1'></a>
``return _``

##### Constructor ``new(source_id: _, source: _)`` <a id='io-4-c-1'></a>
``return _``

##### Method ``unexpected(point: _)`` <a id='io-4-m-1'></a>
``return _``

##### Method ``is_eof(point: _)`` <a id='io-4-m0'></a>
``return _``

##### Method ``is_whitespace(point: _)`` <a id='io-4-m1'></a>
``return _``

##### Method ``is_token(point: _)`` <a id='io-4-m2'></a>
``return _``

##### Method ``next()`` <a id='io-4-m3'></a>
``return _``

##### Method ``peek() { peek() { peek(1: _)`` <a id='io-4-m4'></a>
``return _``

##### Method ``peek(n: _)`` <a id='io-4-m5'></a>
``return _``

##### Method ``peeks() { peeks() { peeks(1: _)`` <a id='io-4-m6'></a>
``return _``

##### Method ``peeks(n: _)`` <a id='io-4-m7'></a>
``return _``

##### Method ``step(consume: _)`` <a id='io-4-m8'></a>
``return _``

##### Method ``skips(consume: _)`` <a id='io-4-m9'></a>
``return _``

##### Method ``parse_key()`` <a id='io-4-m10'></a>
``return _``

##### Method ``parse_primitive()`` <a id='io-4-m11'></a>
``return _``

##### Method ``read_raw_string()`` <a id='io-4-m12'></a>
``return _``

##### Method ``read_string()`` <a id='io-4-m13'></a>
``return _``

##### Method ``parse_string()`` <a id='io-4-m14'></a>
``return _``

##### Method ``parse_value()`` <a id='io-4-m15'></a>
``return _``

##### Method ``parse_list()`` <a id='io-4-m16'></a>
``return _``

##### Method ``parse_map()`` <a id='io-4-m17'></a>
``return _``

##### Method ``parse_map_value()`` <a id='io-4-m18'></a>
``return _``

