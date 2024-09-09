# test
### Modules
> - [app](#app)
> - [io](#io)
> - [math](#math)
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
> - [%(name)](#%(name))
> - [](#)
> - [Json](#json)
> - [JsonStringify](#jsonstringify)
> - [JsonParser](#jsonparser)
### Foreign Class ``Fs``
#### Methods

### Class ``Serializable``
### Class ``%(name)``
> Inherits from ``Serializable``
### Class ````
#### Methods
> - //example Serializable.wrapper
> - wrapperFn
##### Method ``//example Serializable.wrapper({"math": "Vec2"}: _, "Rect": _, [["pos": _, Vec2]: _, ["size": _, Vec2]]: _)``
``return _``

##### Static Method ``wrapperFn(imports: _, name: _, values: _)``
``return _``

#### Methods

#### Methods
> - if
##### Method ``if(f is List: _)``
``return _``

#### Methods
> - var result =
> - for
##### Getter ``var result =``
``return _``

##### Method ``for(item in obj: _)``
``return _``

#### Methods
> - var result =
> - while
##### Getter ``var result =``
``return _``

##### Method ``while(true: _)``
``return _``

#### Methods
> - while
> - f2.call
##### Method ``while(true: _)``
``return _``

##### Method ``f2.call()``
``return _``

#### Methods
> - return Fiber.new
##### Getter ``return Fiber.new``
``return _``

### Class ``Json``
#### Methods

#### Methods

#### Methods

#### Methods
> - JsonStringify.stringify
##### Method ``JsonStringify.stringify(value: _, whitespace: _)``
``return _``

#### Methods

### Class ``JsonStringify``
#### Methods
> - if(!
> - if
##### Method ``if(!(!(value is Map || value is List): _)``
``return _``

##### Method ``if(value is Map: _)``
``return _``

#### Methods
> - if(map.count == 0) return out.call
> - out.call
##### Method ``if(map.count == 0) return out.call(map.count == 0) return out.call("{}": _)``
``return _``

##### Method ``out.call("{\n": _)``
``return _``

#### Methods
> - if
##### Method ``if(value is String: _)``
``return _``

#### Methods
> - for
##### Method ``for(item in list: _)``
``return _``

#### Methods
> - if
##### Method ``if(value is Map: _)``
``return _``

### Class ``JsonParser``
#### Methods

#### Methods
> - if
##### Method ``if(pk == _OPEN_BRACKET: _)``
``return _``

#### Methods

#### Methods

#### Methods

#### Methods

#### Methods

#### Methods

#### Methods

#### Methods

#### Methods

#### Methods
> - if
> - if
##### Method ``if(cur >= _end: _)``
``return _``

##### Method ``if(consume: _)``
``return _``

#### Methods
> - while
##### Method ``while(true: _)``
``return _``

#### Methods
> - while
> - if
##### Method ``while(true: _)``
``return _``

##### Method ``if(!quoted: _)``
``return _``

#### Methods
> - while
##### Method ``while(true: _)``
``return _``

#### Methods
> - while
##### Method ``while(true: _)``
``return _``

#### Methods
> - while
##### Method ``while(true: _)``
``return _``

#### Methods
> - if(peeks() == _OPEN_STRING && peeks
##### Method ``if(peeks() == _OPEN_STRING && peeks(peeks() == _OPEN_STRING && peeks(2) == _OPEN_STRING: _)``
``return _``

#### Methods

#### Methods
> - while
##### Method ``while(true: _)``
``return _``

#### Methods

#### Methods
> - var map =
> - while
##### Getter ``var map =``
``return _``

##### Method ``while(true: _)``
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

