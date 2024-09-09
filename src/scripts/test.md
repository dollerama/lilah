# test
### Modules
> - [app](#module---app--)
> - [io](#module---io--)
> - [math](#module---math--)
## Module ``app``
### Classes
> - [GameObjectRef](#gameobjectref)
> - [Lilah](#lilah)
> - [Audio](#audio)
> - [KeycodeLookup](#keycodelookup)
> - [Input](#input)
> - [UI](#ui)
> - [Curve](#curve)
> - [Tween](#tween)
### Class ``GameObjectRef``
#### Methods
> - create_ref
> - ref
> - behaviourData
> - behaviourData
> - behaviourData
> - data
> - data
> - [key]
> - [key]
> - new
##### Static Method ``create_ref(id: _)``
``return _``

##### Getter ``ref``
``return _``

##### Method ``behaviourData(b: _)``
``return _``

##### Method ``behaviourData(b: _, uuid: _)``
``return _``

##### Method ``behaviourData(b: _, u: _, mut: _)``
``return _``

##### Setter ``data = v: _``

##### Getter ``data``
``return _``

##### Getter ``[key]``
``return _``

##### Setter ``[key] = v: _``

##### Constructor ``new(i: _)``
``return _``

### Class ``Lilah``
#### Methods
> - camera
> - destroy
> - destroy_internal
> - gameobjects
> - gameobjects
> - gameobjects_values
> - data
> - data
> - delta_time
> - delta_time
> - time
> - time
> - fps
> - fps
> - fullscreen
> - fullscreen
> - screen_size
> - screen_size
> - fiberCount
> - tick_fibers
> - start_fiber
> - instantiate
> - instantiate
> - clear
> - destroy
> - find
##### Static Getter ``camera``
``return _``

##### Static Getter ``destroy``
``return _``

##### Static Getter ``destroy_internal``
``return _``

##### Static Getter ``gameobjects``
``return _``

##### Static Setter ``gameobjects = v: _``

##### Static Getter ``gameobjects_values``
``return _``

##### Static Getter ``data``
``return _``

##### Static Setter ``data = v: _``

##### Static Getter ``delta_time``
``return _``

##### Static Setter ``delta_time = v: _``

##### Static Getter ``time``
``return _``

##### Static Setter ``time = v: _``

##### Static Getter ``fps``
``return _``

##### Static Setter ``fps = v: _``

##### Static Getter ``fullscreen``
``return _``

##### Static Setter ``fullscreen = v: _``

##### Static Getter ``screen_size``
``return _``

##### Static Setter ``screen_size = v: _``

##### Static Getter ``fiberCount``
``return _``

##### Static Method ``tick_fibers()``
``return _``

##### Static Method ``start_fiber(f: _)``
``return _``

##### Static Method ``instantiate(go: _, d: _)``
``return _``

##### Static Method ``instantiate(go: _)``
``return _``

##### Static Method ``clear()``
``return _``

##### Static Method ``destroy(key: _)``
``return _``

##### Static Method ``find(key: _)``
``return _``

### Class ``Audio``
#### Methods
> - music
> - command
> - dirty
> - volume
> - fade
> - volume
> - play
> - play
> - play
> - pause
> - pause
> - clear
##### Static Getter ``music``
``return _``

##### Static Getter ``command``
``return _``

##### Static Getter ``dirty``
``return _``

##### Static Getter ``volume``
``return _``

##### Static Getter ``fade``
``return _``

##### Static Setter ``volume = v: _``

##### Static Method ``play(file: _)``
``return _``

##### Static Method ``play(file: _, fade_in_ms: _)``
``return _``

##### Static Method ``play()``
``return _``

##### Static Method ``pause()``
``return _``

##### Static Method ``pause(fade_out_ms: _)``
``return _``

##### Static Method ``clear()``
``return _``

### Class ``KeycodeLookup``
#### Methods
> - new
> - W
> - A
> - S
> - D
> - Up
> - Right
> - Down
> - Left
##### Constructor ``new()``
``return _``

##### Getter ``W``
``return _``

##### Getter ``A``
``return _``

##### Getter ``S``
``return _``

##### Getter ``D``
``return _``

##### Getter ``Up``
``return _``

##### Getter ``Right``
``return _``

##### Getter ``Down``
``return _``

##### Getter ``Left``
``return _``

### Class ``Input``
#### Methods
> - mouse_pos
> - Keycode
> - is_pressed
> - is_mouse_pressed
> - mappings
> - mouse_mappings
> - bindings
> - set_mouse_pos
> - update_mapping
> - update_mouse_mapping
> - update_binding
> - key
> - mouse
> - key_down
> - mouse_down
> - binding
> - binding2D
##### Static Getter ``mouse_pos``
``return _``

##### Static Getter ``Keycode``
``return _``

##### Static Method ``is_pressed(key: _)``
``return _``

##### Static Method ``is_mouse_pressed(key: _)``
``return _``

##### Static Getter ``mappings``
``return _``

##### Static Getter ``mouse_mappings``
``return _``

##### Static Getter ``bindings``
``return _``

##### Static Method ``set_mouse_pos(pos: _)``
``return _``

##### Static Method ``update_mapping(key: _, pressed: _, pressed_down: _)``
``return _``

##### Static Method ``update_mouse_mapping(button: _, pressed: _, pressed_down: _)``
``return _``

##### Static Method ``update_binding(bind: _, neg: _, pos: _)``
``return _``

##### Static Method ``key(key: _)``
``return _``

##### Static Method ``mouse(button: _)``
``return _``

##### Static Method ``key_down(key: _)``
``return _``

##### Static Method ``mouse_down(button: _)``
``return _``

##### Static Method ``binding(bind: _)``
``return _``

##### Static Method ``binding2D(bind1: _, bind2: _)``
``return _``

### Class ``UI``
#### Methods
> - on_click_callbacks
> - on_click_down_callbacks
> - on_hover_callbacks
> - on_click
> - on_click_down
> - on_hover
> - tick
##### Static Getter ``on_click_callbacks``
``return _``

##### Static Getter ``on_click_down_callbacks``
``return _``

##### Static Getter ``on_hover_callbacks``
``return _``

##### Static Method ``on_click(gameobject: _, callback: _)``
``return _``

##### Static Method ``on_click_down(gameobject: _, callback: _)``
``return _``

##### Static Method ``on_hover(gameobject: _, callback: _)``
``return _``

##### Static Method ``tick()``
``return _``

### Class ``Curve``
#### Methods
> - linear
> - inQuad
> - outQuad
> - inOutQuad
> - inQuart
> - outQuart
> - inOutQuart
> - inBack
> - outBack
> - inOutBack
> - inElastic
> - outElastic
> - inOutElastic
##### Static Getter ``linear``
``return _``

##### Static Getter ``inQuad``
``return _``

##### Static Getter ``outQuad``
``return _``

##### Static Getter ``inOutQuad``
``return _``

##### Static Getter ``inQuart``
``return _``

##### Static Getter ``outQuart``
``return _``

##### Static Getter ``inOutQuart``
``return _``

##### Static Getter ``inBack``
``return _``

##### Static Getter ``outBack``
``return _``

##### Static Getter ``inOutBack``
``return _``

##### Static Getter ``inElastic``
``return _``

##### Static Getter ``outElastic``
``return _``

##### Static Getter ``inOutElastic``
``return _``

### Class ``Tween``
#### Methods
> - tweens
> - tweens
> - tweenCount
> - insert_tween
> - duration
> - use_curve
> - from
> - to
> - on_complete
> - duration
> - use_curve
> - from
> - to
> - on_complete
> - new
> - toString { "Tween
> - time
> - curve
> - onComplete
> - play
##### Static Getter ``tweens``
``return _``

##### Static Setter ``tweens = v: _``

##### Static Getter ``tweenCount``
``return _``

##### Static Method ``insert_tween(t: _)``
``return _``

##### Getter ``duration``
``return _``

##### Getter ``use_curve``
``return _``

##### Getter ``from``
``return _``

##### Getter ``to``
``return _``

##### Getter ``on_complete``
``return _``

##### Setter ``duration = v: _``

##### Setter ``use_curve = v: _``

##### Setter ``from = v: _``

##### Setter ``to = v: _``

##### Setter ``on_complete = v: _``

##### Constructor ``new(f: _, t: _)``
``return _``

##### Getter ``toString { "Tween``
``return _``

##### Method ``time(t: _)``
``return _``

##### Method ``curve(c: _)``
``return _``

##### Method ``onComplete(c: _)``
``return _``

##### Method ``play(c: _)``
``return _``

## Module ``io``
### Classes
> - [Fs](#fs)
> - [Serializable](#serializable)
> - [Json](#json)
> - [JsonStringify](#jsonstringify)
> - [JsonParser](#jsonparser)
### Foreign Class ``Fs``
#### Methods

### Class ``Serializable``
#### Methods
> - wrapperFn
> - wrapper
> - properties
> - serialize
> - serialize
> - deserialize
> - iterProperties
##### Static Method ``wrapperFn(imports: _, name: _, values: _)``
``return _``

##### Static Method ``wrapper(imports: _, name: _, values: _)``
``return _``

##### Method ``properties(f: _)``
``return _``

##### Static Method ``serialize(obj: _)``
``return _``

##### Method ``serialize()``
``return _``

##### Method ``deserialize(obj: _)``
``return _``

##### Static Method ``iterProperties(t: _)``
``return _``

### Class ``Json``
#### Methods
> - parse(string) { parse
> - parse
> - stringify(value) { stringify
> - stringify
> - stringify
##### Static Method ``parse(string) { parse(string) { parse("json": _, string: _)``
``return _``

##### Static Method ``parse(source_id: _, source_string: _)``
``return _``

##### Static Method ``stringify(value) { stringify(value) { stringify(value: _, "  ": _)``
``return _``

##### Static Method ``stringify(value: _, whitespace: _)``
``return _``

##### Static Method ``stringify(value: _, whitespace: _, callback: _)``
``return _``

### Class ``JsonStringify``
#### Methods
> - stringify
> - stringify_map
> - stringify_primitive
> - stringify_list
> - stringify_value
##### Static Method ``stringify(value: _, whitespace: _, out: _)``
``return _``

##### Static Method ``stringify_map(map: _, whitespace: _, depth: _, out: _)``
``return _``

##### Static Method ``stringify_primitive(value: _, out: _)``
``return _``

##### Static Method ``stringify_list(list: _, whitespace: _, depth: _, out: _)``
``return _``

##### Static Method ``stringify_value(value: _, whitespace: _, depth: _, out: _)``
``return _``

### Class ``JsonParser``
#### Methods
> - root
> - new
> - unexpected
> - is_eof
> - is_whitespace
> - is_token
> - next
> - peek() { peek
> - peek
> - peeks() { peeks
> - peeks
> - step
> - skips
> - parse_key
> - parse_primitive
> - read_raw_string
> - read_string
> - parse_string
> - parse_value
> - parse_list
> - parse_map
> - parse_map_value
##### Getter ``root``
``return _``

##### Constructor ``new(source_id: _, source: _)``
``return _``

##### Method ``unexpected(point: _)``
``return _``

##### Method ``is_eof(point: _)``
``return _``

##### Method ``is_whitespace(point: _)``
``return _``

##### Method ``is_token(point: _)``
``return _``

##### Method ``next()``
``return _``

##### Method ``peek() { peek() { peek(1: _)``
``return _``

##### Method ``peek(n: _)``
``return _``

##### Method ``peeks() { peeks() { peeks(1: _)``
``return _``

##### Method ``peeks(n: _)``
``return _``

##### Method ``step(consume: _)``
``return _``

##### Method ``skips(consume: _)``
``return _``

##### Method ``parse_key()``
``return _``

##### Method ``parse_primitive()``
``return _``

##### Method ``read_raw_string()``
``return _``

##### Method ``read_string()``
``return _``

##### Method ``parse_string()``
``return _``

##### Method ``parse_value()``
``return _``

##### Method ``parse_list()``
``return _``

##### Method ``parse_map()``
``return _``

##### Method ``parse_map_value()``
``return _``

## Module ``math``
### Classes
> - [Util](#util)
> - [Vec2](#vec2)
### Class ``Util``
#### Methods
> - lerp
##### Static Method ``lerp(a: _, b: _, t: _)``
``return _``

### Foreign Class ``Vec2``
> Inherits from ``Serializable``
#### Methods
> - new
> - default
> - getProperty
> - setProperty
> - +
> - -
> - *
> - /
> - -
> - =
> - toString
##### Constructor ``new(x: _, y: _)``
``return _``

##### Static Getter ``default``
``return _``

##### Method ``getProperty()``
``return _``

##### Method ``setProperty()``
``return _``

##### Method ``+(other: _)``
``return _``

##### Method ``-(other: _)``
``return _``

##### Method ``*(other: _)``
``return _``

##### Method ``/(other: _)``
``return _``

##### Getter ``-``
``return _``

##### Setter ``= = other: _``

##### Getter ``toString``
``return _``

