# lilah-scripting
### Modules
> - [app](#module-app)
> - [io](#module-io)
> - [math](#module-math)
> - [Trail](#module-trail)
> - [ParticleSystem](#module-particlesystem)
## Module ``app``
### Classes
> - [GameObjectRef](#class-gameobjectref)
> - [Lilah](#class-lilah)
> - [Audio](#class-audio)
> - [KeycodeLookup](#class-keycodelookup)
> - [Input](#class-input)
> - [UI](#class-ui)
> - [Curve](#class-curve)
> - [Tween](#class-tween)
### Class ``GameObjectRef``
>

#### Constructors
> - [new](#0-c-1)
#### Getters
> - [ref](#0-g-1)
> - [data](#0-g0)
> - [[key]](#0-g1)
#### Setters
> - [data](#0-s-1)
> - [[key]](#0-s0)
#### Methods
> - [create_ref](#0-m-1)
> - [behaviourData](#0-m0)
> - [behaviourData](#0-m1)
> - [behaviourData](#0-m2)
##### Static Method ``create_ref(id: _)`` {#0-m-1}
``return _``

##### Getter ``ref`` {#0-g-1}
``return _``

##### Method ``behaviourData(b: _)`` {#0-m0}
``return _``

##### Method ``behaviourData(b: _, uuid: _)`` {#0-m1}
``return _``

##### Method ``behaviourData(b: _, u: _, mut: _)`` {#0-m2}
``return _``

##### Setter ``data = v: _`` {#0-s-1}

##### Getter ``data`` {#0-g0}
``return _``

##### Getter ``[key]`` {#0-g1}
``return _``

##### Setter ``[key]`` {#0-s0}

##### Constructor ``new(i: _)`` <a id='0-c-1'></a>
``return _``

### Class ``Lilah``
>

#### Getters
> - [camera](#1-g-1)
> - [destroy](#1-g0)
> - [destroy_internal](#1-g1)
> - [gameobjects](#1-g2)
> - [gameobjects_values](#1-g3)
> - [data](#1-g4)
> - [delta_time](#1-g5)
> - [time](#1-g6)
> - [fps](#1-g7)
> - [fullscreen](#1-g8)
> - [screen_size](#1-g9)
> - [fiberCount](#1-g10)
#### Setters
> - [gameobjects](#1-s-1)
> - [data](#1-s0)
> - [delta_time](#1-s1)
> - [time](#1-s2)
> - [fps](#1-s3)
> - [fullscreen](#1-s4)
> - [screen_size](#1-s5)
#### Methods
> - [tick_fibers](#1-m-1)
> - [start_fiber](#1-m0)
> - [instantiate](#1-m1)
> - [instantiate](#1-m2)
> - [clear](#1-m3)
> - [destroy](#1-m4)
> - [find](#1-m5)
##### Static Getter ``camera`` {#1-g-1}
``return _``

##### Static Getter ``destroy`` {#1-g0}
``return _``

##### Static Getter ``destroy_internal`` {#1-g1}
``return _``

##### Static Getter ``gameobjects`` {#1-g2}
``return _``

##### Static Setter ``gameobjects = v: _`` {#1-s-1}

##### Static Getter ``gameobjects_values`` {#1-g3}
``return _``

##### Static Getter ``data`` {#1-g4}
``return _``

##### Static Setter ``data = v: _`` {#1-s0}

##### Static Getter ``delta_time`` {#1-g5}
``return _``

##### Static Setter ``delta_time = v: _`` {#1-s1}

##### Static Getter ``time`` {#1-g6}
``return _``

##### Static Setter ``time = v: _`` {#1-s2}

##### Static Getter ``fps`` {#1-g7}
``return _``

##### Static Setter ``fps = v: _`` {#1-s3}

##### Static Getter ``fullscreen`` {#1-g8}
``return _``

##### Static Setter ``fullscreen = v: _`` {#1-s4}

##### Static Getter ``screen_size`` {#1-g9}
``return _``

##### Static Setter ``screen_size = v: _`` {#1-s5}

##### Static Getter ``fiberCount`` {#1-g10}
``return _``

##### Static Method ``tick_fibers()`` {#1-m-1}
``return _``

##### Static Method ``start_fiber(f: _)`` {#1-m0}
``return _``

##### Static Method ``instantiate(go: _, d: _)`` {#1-m1}
``return _``

##### Static Method ``instantiate(go: _)`` {#1-m2}
``return _``

##### Static Method ``clear()`` {#1-m3}
``return _``

##### Static Method ``destroy(key: _)`` {#1-m4}
``return _``

##### Static Method ``find(key: _)`` {#1-m5}
``return _``

### Class ``Audio``
>

#### Getters
> - [music](#2-g-1)
> - [command](#2-g0)
> - [dirty](#2-g1)
> - [volume](#2-g2)
> - [fade](#2-g3)
#### Setters
> - [volume](#2-s-1)
#### Methods
> - [play](#2-m-1)
> - [play](#2-m0)
> - [play](#2-m1)
> - [pause](#2-m2)
> - [pause](#2-m3)
> - [clear](#2-m4)
##### Static Getter ``music`` {#2-g-1}
``return _``

##### Static Getter ``command`` {#2-g0}
``return _``

##### Static Getter ``dirty`` {#2-g1}
``return _``

##### Static Getter ``volume`` {#2-g2}
``return _``

##### Static Getter ``fade`` {#2-g3}
``return _``

##### Static Setter ``volume = v: _`` {#2-s-1}

##### Static Method ``play(file: _)`` {#2-m-1}
``return _``

##### Static Method ``play(file: _, fade_in_ms: _)`` {#2-m0}
``return _``

##### Static Method ``play()`` {#2-m1}
``return _``

##### Static Method ``pause()`` {#2-m2}
``return _``

##### Static Method ``pause(fade_out_ms: _)`` {#2-m3}
``return _``

##### Static Method ``clear()`` {#2-m4}
``return _``

### Class ``KeycodeLookup``
>

#### Constructors
> - [new](#3-c-1)
#### Getters
> - [W](#3-g-1)
> - [A](#3-g0)
> - [S](#3-g1)
> - [D](#3-g2)
> - [Up](#3-g3)
> - [Right](#3-g4)
> - [Down](#3-g5)
> - [Left](#3-g6)
##### Constructor ``new()`` {#3-c-1}
``return _``

##### Getter ``W`` {#3-g-1}
``return _``

##### Getter ``A`` {#3-g0}
``return _``

##### Getter ``S`` {#3-g1}
``return _``

##### Getter ``D`` {#3-g2}
``return _``

##### Getter ``Up`` {#3-g3}
``return _``

##### Getter ``Right`` {#3-g4}
``return _``

##### Getter ``Down`` {#3-g5}
``return _``

##### Getter ``Left`` {#3-g6}
``return _``

### Class ``Input``
>

#### Getters
> - [mouse_pos](#4-g-1)
> - [Keycode](#4-g0)
> - [mappings](#4-g1)
> - [mouse_mappings](#4-g2)
> - [bindings](#4-g3)
#### Methods
> - [is_pressed](#4-m-1)
> - [is_mouse_pressed](#4-m0)
> - [set_mouse_pos](#4-m1)
> - [update_mapping](#4-m2)
> - [update_mouse_mapping](#4-m3)
> - [update_binding](#4-m4)
> - [key](#4-m5)
> - [mouse](#4-m6)
> - [key_down](#4-m7)
> - [mouse_down](#4-m8)
> - [binding](#4-m9)
> - [binding2D](#4-m10)
##### Static Getter ``mouse_pos`` {#4-g-1}
``return _``

##### Static Getter ``Keycode`` {#4-g0}
``return _``

##### Static Method ``is_pressed(key: _)`` {#4-m-1}
``return _``

##### Static Method ``is_mouse_pressed(key: _)`` {#4-m0}
``return _``

##### Static Getter ``mappings`` {#4-g1}
``return _``

##### Static Getter ``mouse_mappings`` {#4-g2}
``return _``

##### Static Getter ``bindings`` {#4-g3}
``return _``

##### Static Method ``set_mouse_pos(pos: _)`` {#4-m1}
``return _``

##### Static Method ``update_mapping(key: _, pressed: _, pressed_down: _)`` {#4-m2}
``return _``

##### Static Method ``update_mouse_mapping(button: _, pressed: _, pressed_down: _)`` {#4-m3}
``return _``

##### Static Method ``update_binding(bind: _, neg: _, pos: _)`` {#4-m4}
``return _``

##### Static Method ``key(key: _)`` {#4-m5}
``return _``

##### Static Method ``mouse(button: _)`` {#4-m6}
``return _``

##### Static Method ``key_down(key: _)`` {#4-m7}
``return _``

##### Static Method ``mouse_down(button: _)`` {#4-m8}
``return _``

##### Static Method ``binding(bind: _)`` {#4-m9}
``return _``

##### Static Method ``binding2D(bind1: _, bind2: _)`` {#4-m10}
``return _``

### Class ``UI``
>

#### Getters
> - [on_click_callbacks](#5-g-1)
> - [on_click_down_callbacks](#5-g0)
> - [on_hover_callbacks](#5-g1)
#### Methods
> - [on_click](#5-m-1)
> - [on_click_down](#5-m0)
> - [on_hover](#5-m1)
> - [tick](#5-m2)
##### Static Getter ``on_click_callbacks`` {#5-g-1}
``return _``

##### Static Getter ``on_click_down_callbacks`` {#5-g0}
``return _``

##### Static Getter ``on_hover_callbacks`` {#5-g1}
``return _``

##### Static Method ``on_click(gameobject: _, callback: _)`` {#5-m-1}
``return _``

##### Static Method ``on_click_down(gameobject: _, callback: _)`` {#5-m0}
``return _``

##### Static Method ``on_hover(gameobject: _, callback: _)`` {#5-m1}
``return _``

##### Static Method ``tick()`` {#5-m2}
``return _``

### Class ``Curve``
>

#### Getters
> - [linear](#6-g-1)
> - [inQuad](#6-g0)
> - [outQuad](#6-g1)
> - [inOutQuad](#6-g2)
> - [inQuart](#6-g3)
> - [outQuart](#6-g4)
> - [inOutQuart](#6-g5)
> - [inBack](#6-g6)
> - [outBack](#6-g7)
> - [inOutBack](#6-g8)
> - [inElastic](#6-g9)
> - [outElastic](#6-g10)
> - [inOutElastic](#6-g11)
##### Static Getter ``linear`` {#6-g-1}
``return _``

##### Static Getter ``inQuad`` {#6-g0}
``return _``

##### Static Getter ``outQuad`` {#6-g1}
``return _``

##### Static Getter ``inOutQuad`` {#6-g2}
``return _``

##### Static Getter ``inQuart`` {#6-g3}
``return _``

##### Static Getter ``outQuart`` {#6-g4}
``return _``

##### Static Getter ``inOutQuart`` {#6-g5}
``return _``

##### Static Getter ``inBack`` {#6-g6}
``return _``

##### Static Getter ``outBack`` {#6-g7}
``return _``

##### Static Getter ``inOutBack`` {#6-g8}
``return _``

##### Static Getter ``inElastic`` {#6-g9}
``return _``

##### Static Getter ``outElastic`` {#6-g10}
``return _``

##### Static Getter ``inOutElastic`` {#6-g11}
``return _``

### Class ``Tween``
>

#### Constructors
> - [new](#7-c-1)
#### Getters
> - [tweens](#7-g-1)
> - [tweenCount](#7-g0)
> - [duration](#7-g1)
> - [use_curve](#7-g2)
> - [from](#7-g3)
> - [to](#7-g4)
> - [on_complete](#7-g5)
> - [toString { "Tween](#7-g6)
#### Setters
> - [tweens](#7-s-1)
> - [duration](#7-s0)
> - [use_curve](#7-s1)
> - [from](#7-s2)
> - [to](#7-s3)
> - [on_complete](#7-s4)
#### Methods
> - [insert_tween](#7-m-1)
> - [time](#7-m0)
> - [curve](#7-m1)
> - [onComplete](#7-m2)
> - [play](#7-m3)
##### Static Getter ``tweens`` {#7-g-1}
``return _``

##### Static Setter ``tweens = v: _`` {#7-s-1}

##### Static Getter ``tweenCount`` {#7-g0}
``return _``

##### Static Method ``insert_tween(t: _)`` {#7-m-1}
``return _``

##### Getter ``duration`` {#7-g1}
``return _``

##### Getter ``use_curve`` {#7-g2}
``return _``

##### Getter ``from`` {#7-g3}
``return _``

##### Getter ``to`` {#7-g4}
``return _``

##### Getter ``on_complete`` {#7-g5}
``return _``

##### Setter ``duration = v: _`` {#7-s0}

##### Setter ``use_curve = v: _`` {#7-s1}

##### Setter ``from = v: _`` {#7-s2}

##### Setter ``to = v: _`` {#7-s3}

##### Setter ``on_complete = v: _`` {#7-s4}

##### Constructor ``new(f: _, t: _)`` {#7-c-1}
``return _``

##### Getter ``toString { "Tween`` {#7-g6}
``return _``

##### Method ``time(t: _)`` {#7-m0}
``return _``

##### Method ``curve(c: _)`` {#7-m1}
``return _``

##### Method ``onComplete(c: _)`` {#7-m2}
``return _``

##### Method ``play(c: _)`` {#7-m3}
``return _``

## Module ``io``
### Classes
> - [Fs](#foreign-class-fs)
> - [Serializable](#class-serializable)
> - [Json](#class-json)
> - [JsonStringify](#class-jsonstringify)
> - [JsonParser](#class-jsonparser)
### Foreign Class ``Fs``
>

#### Methods
> - [read](#0-m-1)
> - [write](#0-m0)
##### Foreign Static Method ``read(file: _)`` {#0-m-1}
``return _``

##### Foreign Static Method ``write(file: _, content: _)`` {#0-m0}
``return _``

### Class ``Serializable``
>

#### Methods
> - [//example Serializable.wrapper](#1-m-1)
> - [wrapperFn](#1-m0)
> - [wrapper](#1-m1)
> - [properties](#1-m2)
> - [serialize](#1-m3)
> - [serialize](#1-m4)
> - [deserialize](#1-m5)
> - [iterProperties](#1-m6)
##### Method ``//example Serializable.wrapper({"math": "Vec2"}: _, "Rect": _, [["pos": _, Vec2]: _, ["size": _, Vec2]]: _)`` {#1-m-1}
``return _``

##### Static Method ``wrapperFn(imports: _, name: _, values: _)`` {#1-m0}
``return _``

##### Static Method ``wrapper(imports: _, name: _, values: _)`` {#1-m1}
``return _``

##### Method ``properties(f: _)`` {#1-m2}
``return _``

##### Static Method ``serialize(obj: _)`` {#1-m3}
``return _``

##### Method ``serialize()`` {#1-m4}
``return _``

##### Method ``deserialize(obj: _)`` {#1-m5}
``return _``

##### Static Method ``iterProperties(t: _)`` {#1-m6}
``return _``

### Class ``Json``
>

#### Methods
> - [parse(string) { parse](#2-m-1)
> - [parse](#2-m0)
> - [stringify(value) { stringify](#2-m1)
> - [stringify](#2-m2)
> - [stringify](#2-m3)
##### Static Method ``parse(string) { parse(string) { parse("json": _, string: _)`` {#2-m-1}
``return _``

##### Static Method ``parse(source_id: _, source_string: _)`` {#2-m0}
``return _``

##### Static Method ``stringify(value) { stringify(value) { stringify(value: _, "  ": _)`` {#2-m1}
``return _``

##### Static Method ``stringify(value: _, whitespace: _)`` {#2-m2}
``return _``

##### Static Method ``stringify(value: _, whitespace: _, callback: _)`` {#2-m3}
``return _``

### Class ``JsonStringify``
>

#### Methods
> - [stringify](#3-m-1)
> - [stringify_map](#3-m0)
> - [stringify_primitive](#3-m1)
> - [stringify_list](#3-m2)
> - [stringify_value](#3-m3)
##### Static Method ``stringify(value: _, whitespace: _, out: _)`` {#3-m-1}
``return _``

##### Static Method ``stringify_map(map: _, whitespace: _, depth: _, out: _)`` {#3-m0}
``return _``

##### Static Method ``stringify_primitive(value: _, out: _)`` {#3-m1}
``return _``

##### Static Method ``stringify_list(list: _, whitespace: _, depth: _, out: _)`` {#3-m2}
``return _``

##### Static Method ``stringify_value(value: _, whitespace: _, depth: _, out: _)`` {#3-m3}
``return _``

### Class ``JsonParser``
>

#### Constructors
> - [new](#4-c-1)
#### Getters
> - [root](#4-g-1)
#### Methods
> - [unexpected](#4-m-1)
> - [is_eof](#4-m0)
> - [is_whitespace](#4-m1)
> - [is_token](#4-m2)
> - [next](#4-m3)
> - [peek() { peek](#4-m4)
> - [peek](#4-m5)
> - [peeks() { peeks](#4-m6)
> - [peeks](#4-m7)
> - [step](#4-m8)
> - [skips](#4-m9)
> - [parse_key](#4-m10)
> - [parse_primitive](#4-m11)
> - [read_raw_string](#4-m12)
> - [read_string](#4-m13)
> - [parse_string](#4-m14)
> - [parse_value](#4-m15)
> - [parse_list](#4-m16)
> - [parse_map](#4-m17)
> - [parse_map_value](#4-m18)
##### Getter ``root`` {#4-g-1}
``return _``

##### Constructor ``new(source_id: _, source: _)`` {#4-c-1}
``return _``

##### Method ``unexpected(point: _)`` {#4-m-1}
``return _``

##### Method ``is_eof(point: _)`` {#4-m0}
``return _``

##### Method ``is_whitespace(point: _)`` {#4-m1}
``return _``

##### Method ``is_token(point: _)`` {#4-m2}
``return _``

##### Method ``next()`` {#4-m3}
``return _``

##### Method ``peek() { peek() { peek(1: _)`` {#4-m4}
``return _``

##### Method ``peek(n: _)`` {#4-m5}
``return _``

##### Method ``peeks() { peeks() { peeks(1: _)`` {#4-m6}
``return _``

##### Method ``peeks(n: _)`` {#4-m7}
``return _``

##### Method ``step(consume: _)`` {#4-m8}
``return _``

##### Method ``skips(consume: _)`` {#4-m9}
``return _``

##### Method ``parse_key()`` {#4-m10}
``return _``

##### Method ``parse_primitive()`` {#4-m11}
``return _``

##### Method ``read_raw_string()`` {#4-m12}
``return _``

##### Method ``read_string()`` {#4-m13}
``return _``

##### Method ``parse_string()`` {#4-m14}
``return _``

##### Method ``parse_value()`` {#4-m15}
``return _``

##### Method ``parse_list()`` {#4-m16}
``return _``

##### Method ``parse_map()`` {#4-m17}
``return _``

##### Method ``parse_map_value()`` {#4-m18}
``return _``

## Module ``math``
### Classes
> - [Util](#class-util)
> - [Vec2](#foreign-class-vec2)
### Class ``Util``
>

#### Methods
> - [lerp](#0-m-1)
##### Static Method ``lerp(a: _, b: _, t: _)`` {#0-m-1}
``return _``

### Foreign Class ``Vec2``
> Inherits from ``Serializable``
>

#### Constructors
> - [new](#1-c-1)
#### Getters
> - [x](#1-g-1)
> - [y](#1-g0)
> - [default](#1-g1)
> - [one](#1-g2)
> - [zero](#1-g3)
> - [up](#1-g4)
> - [down](#1-g5)
> - [left](#1-g6)
> - [right](#1-g7)
> - [-](#1-g8)
> - [toString](#1-g9)
#### Setters
> - [x](#1-s-1)
> - [y](#1-s0)
#### Methods
> - [getProperty](#1-m-1)
> - [setProperty](#1-m0)
> - [magnitude](#1-m1)
> - [magnitude_sqr](#1-m2)
> - [normalized](#1-m3)
> - [normalize](#1-m4)
> - [cross](#1-m5)
> - [dot](#1-m6)
> - [lerp](#1-m7)
> - [screen_to_world_space](#1-m8)
> - [world_to_screen_space](#1-m9)
> - [+](#1-m10)
> - [-](#1-m11)
> - [*](#1-m12)
> - [/](#1-m13)
> - [=](#1-m14)
##### Constructor ``new(x: _, y: _)`` {#1-c-1}
``return _``

##### Foreign Getter ``x`` {#1-g-1}
``return _``

##### Foreign Getter ``y`` {#1-g0}
``return _``

##### Foreign Setter ``x = x: _`` {#1-s-1}

##### Foreign Setter ``y = y: _`` {#1-s0}

##### Static Getter ``default`` {#1-g1}
``return _``

##### Method ``getProperty()`` {#1-m-1}
``return _``

##### Method ``setProperty()`` {#1-m0}
``return _``

##### Foreign Static Getter ``one`` {#1-g2}
``return _``

##### Foreign Static Getter ``zero`` {#1-g3}
``return _``

##### Foreign Static Getter ``up`` {#1-g4}
``return _``

##### Foreign Static Getter ``down`` {#1-g5}
``return _``

##### Foreign Static Getter ``left`` {#1-g6}
``return _``

##### Foreign Static Getter ``right`` {#1-g7}
``return _``

##### Foreign Method ``magnitude()`` {#1-m1}
``return _``

##### Foreign Method ``magnitude_sqr()`` {#1-m2}
``return _``

##### Foreign Method ``normalized()`` {#1-m3}
``return _``

##### Foreign Method ``normalize()`` {#1-m4}
``return _``

##### Foreign Static Method ``cross(a: _, b: _)`` {#1-m5}
``return _``

##### Foreign Static Method ``dot(a: _, b: _)`` {#1-m6}
``return _``

##### Foreign Static Method ``lerp(a: _, b: _, t: _)`` {#1-m7}
``return _``

##### Foreign Static Method ``screen_to_world_space(pos: _)`` {#1-m8}
``return _``

##### Foreign Static Method ``world_to_screen_space(pos: _)`` {#1-m9}
``return _``

##### Method ``+(other: _)`` {#1-m10}
``return _``

##### Method ``-(other: _)`` {#1-m11}
``return _``

##### Method ``*(other: _)`` {#1-m12}
``return _``

##### Method ``/(other: _)`` {#1-m13}
``return _``

##### Getter ``-`` {#1-g8}
``return _``

##### Method ``==( = other: _)`` {#1-m14}

##### Getter ``toString`` {#1-g9}
``return _``

## Module ``Trail``
### Classes
> - [TrailField](#class-trailfield)
> - [Trail](#class-trail)
### Class ``TrailField``
>

#### Constructors
> - [new](#0-c-1)
> - [new](#0-c0)
#### Getters
> - [raw](#0-g-1)
> - [value](#0-g0)
> - [[t]](#0-g1)
> - [curve](#0-g2)
#### Setters
> - [value](#0-s-1)
> - [[t]](#0-s0)
> - [curve](#0-s1)
##### Getter ``raw`` {#0-g-1}
``return _``

##### Getter ``value`` {#0-g0}
``return _``

##### Setter ``value = v: _`` {#0-s-1}

##### Setter ``[t]`` {#0-s0}

##### Getter ``[t]`` {#0-g1}
``return _``

##### Getter ``curve`` {#0-g2}
``return _``

##### Setter ``curve = v: _`` {#0-s1}

##### Constructor ``new(v: _, c: _)`` {#0-c-1}
``return _``

##### Constructor ``new(v: _)`` {#0-c0}
``return _``

### Class ``Trail``
> Inherits from ``Behaviour``
>

#### Constructors
> - [new(g) { super](#1-c-1)
> - [new](#1-c0)
#### Getters
> - [gameobject](#1-g-1)
> - [gamebehaviour](#1-g0)
> - [default](#1-g1)
> - [minDistance](#1-g2)
> - [maxCount](#1-g3)
> - [hist](#1-g4)
#### Setters
> - [gameobject](#1-s-1)
> - [gamebehaviour](#1-s0)
> - [minDistance](#1-s1)
> - [maxCount](#1-s2)
> - [hist](#1-s3)
#### Methods
> - [start](#1-m-1)
> - [update](#1-m0)
##### Static Getter ``gameobject`` {#1-g-1}
``return _``

##### Static Setter ``gameobject = v) { __gameobject = GameObjectRef.new(v: _`` {#1-s-1}

##### Static Getter ``gamebehaviour`` {#1-g0}
``return _``

##### Static Setter ``gamebehaviour = v: _`` {#1-s0}

##### Constructor ``new(g) { super(g) { super(g: _, Trail: _)`` {#1-c-1}
``return _``

##### Static Getter ``default`` {#1-g1}
``return _``

##### Getter ``minDistance`` {#1-g2}
``return _``

##### Setter ``minDistance = v: _`` {#1-s1}

##### Getter ``maxCount`` {#1-g3}
``return _``

##### Setter ``maxCount = v: _`` {#1-s2}

##### Getter ``hist`` {#1-g4}
``return _``

##### Setter ``hist = v: _`` {#1-s3}

##### Constructor ``new()`` {#1-c0}
``return _``

##### Static Method ``start()`` {#1-m-1}
``return _``

##### Static Method ``update()`` {#1-m0}
``return _``

## Module ``ParticleSystem``
### Classes
> - [ParticleField](#class-particlefield)
> - [ParticleSystem](#class-particlesystem)
### Class ``ParticleField``
>

#### Constructors
> - [new](#0-c-1)
> - [new](#0-c0)
#### Getters
> - [raw](#0-g-1)
> - [value](#0-g0)
> - [[t]](#0-g1)
> - [curve](#0-g2)
#### Setters
> - [value](#0-s-1)
> - [[t]](#0-s0)
> - [curve](#0-s1)
##### Getter ``raw`` {#0-g-1}
``return _``

##### Getter ``value`` {#0-g0}
``return _``

##### Setter ``value = v: _`` {#0-s-1}

##### Setter ``[t]`` {#0-s0}

##### Getter ``[t]`` {#0-g1}
``return _``

##### Getter ``curve`` {#0-g2}
``return _``

##### Setter ``curve = v: _`` {#0-s1}

##### Constructor ``new(v: _, c: _)`` {#0-c-1}
``return _``

##### Constructor ``new(v: _)`` {#0-c0}
``return _``

### Class ``ParticleSystem``
> Inherits from ``Behaviour``
>

#### Constructors
> - [new(g) { super](#1-c-1)
> - [new](#1-c0)
#### Getters
> - [gameobject](#1-g-1)
> - [gamebehaviour](#1-g0)
> - [default](#1-g1)
> - [lifeSpan](#1-g2)
> - [rate](#1-g3)
> - [speed](#1-g4)
> - [rotation](#1-g5)
> - [direction](#1-g6)
> - [scale](#1-g7)
> - [color](#1-g8)
> - [distance](#1-g9)
> - [partSetup](#1-g10)
> - [partStart](#1-g11)
> - [internal_time](#1-g12)
> - [parts](#1-g13)
> - [internal_pos](#1-g14)
#### Setters
> - [gameobject](#1-s-1)
> - [gamebehaviour](#1-s0)
> - [lifeSpan](#1-s1)
> - [rate](#1-s2)
> - [speed](#1-s3)
> - [rotation](#1-s4)
> - [direction](#1-s5)
> - [scale](#1-s6)
> - [color](#1-s7)
> - [distance](#1-s8)
> - [partSetup](#1-s9)
> - [partStart](#1-s10)
> - [internal_time](#1-s11)
> - [parts](#1-s12)
> - [internal_pos](#1-s13)
#### Methods
> - [play](#1-m-1)
> - [stop](#1-m0)
> - [toggle](#1-m1)
> - [isPlaying](#1-m2)
> - [emit](#1-m3)
> - [update](#1-m4)
##### Static Getter ``gameobject`` {#1-g-1}
``return _``

##### Static Setter ``gameobject = v) { __gameobject = GameObjectRef.new(v: _`` {#1-s-1}

##### Static Getter ``gamebehaviour`` {#1-g0}
``return _``

##### Static Setter ``gamebehaviour = v: _`` {#1-s0}

##### Constructor ``new(g) { super(g) { super(g: _, ParticleSystem: _)`` {#1-c-1}
``return _``

##### Static Getter ``default`` {#1-g1}
``return _``

##### Getter ``lifeSpan`` {#1-g2}
``return _``

##### Setter ``lifeSpan = v: _`` {#1-s1}

##### Getter ``rate`` {#1-g3}
``return _``

##### Setter ``rate = v: _`` {#1-s2}

##### Getter ``speed`` {#1-g4}
``return _``

##### Setter ``speed = v: _`` {#1-s3}

##### Getter ``rotation`` {#1-g5}
``return _``

##### Setter ``rotation = v: _`` {#1-s4}

##### Getter ``direction`` {#1-g6}
``return _``

##### Setter ``direction = v: _`` {#1-s5}

##### Getter ``scale`` {#1-g7}
``return _``

##### Setter ``scale = v: _`` {#1-s6}

##### Getter ``color`` {#1-g8}
``return _``

##### Setter ``color = v: _`` {#1-s7}

##### Getter ``distance`` {#1-g9}
``return _``

##### Setter ``distance = v: _`` {#1-s8}

##### Getter ``partSetup`` {#1-g10}
``return _``

##### Setter ``partSetup = v: _`` {#1-s9}

##### Getter ``partStart`` {#1-g11}
``return _``

##### Setter ``partStart = v: _`` {#1-s10}

##### Getter ``internal_time`` {#1-g12}
``return _``

##### Setter ``internal_time = v: _`` {#1-s11}

##### Getter ``parts`` {#1-g13}
``return _``

##### Setter ``parts = v: _`` {#1-s12}

##### Getter ``internal_pos`` {#1-g14}
``return _``

##### Setter ``internal_pos = v: _`` {#1-s13}

##### Method ``play()`` {#1-m-1}
``return _``

##### Method ``stop()`` {#1-m0}
``return _``

##### Method ``toggle()`` {#1-m1}
``return _``

##### Method ``isPlaying()`` {#1-m2}
``return _``

##### Constructor ``new()`` {#1-c0}
``return _``

##### Static Method ``emit()`` {#1-m3}
``return _``

##### Static Method ``update()`` {#1-m4}
``return _``

