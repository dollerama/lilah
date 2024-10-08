# LilahScriptingAPI
### Modules
> - [io](#module-io)
> - [game](#module-game)
> - [ParticleSystem](#module-particlesystem)
> - [Trail](#module-trail)
> - [app](#module-app)
> - [math](#module-math)
## Module ``io``
### Classes
> - [Fs](#io--k-1)
> - [Serializable](#io--k0)
> - [Json](#io--k1)
> - [JsonStringify](#io--k2)
> - [JsonParser](#io--k3)
### Foreign Class ``Fs`` <a id='io--k-1'></a> 
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
> - [wrapperFn](#io-1-m-1)
> - [wrapper](#io-1-m0)
> - [properties](#io-1-m1)
> - [serialize](#io-1-m2)
> - [serialize](#io-1-m3)
> - [deserialize](#io-1-m4)
> - [iterProperties](#io-1-m5)
##### Static Method ``wrapperFn(imports: _, name: _, values: _)`` <a id='io-1-m-1'></a>
``return _``

##### Static Method ``wrapper(imports: _, name: _, values: _)`` <a id='io-1-m0'></a>
``return _``

##### Method ``properties(f: _)`` <a id='io-1-m1'></a>
``return _``

##### Static Method ``serialize(obj: _)`` <a id='io-1-m2'></a>
``return _``

##### Method ``serialize()`` <a id='io-1-m3'></a>
``return _``

##### Method ``deserialize(obj: _)`` <a id='io-1-m4'></a>
``return _``

##### Static Method ``iterProperties(t: _)`` <a id='io-1-m5'></a>
``return _``

### Class ``Json`` <a id='io--k1'></a>
>

#### Methods
> - [parse](#io-2-m-1)
> - [parse](#io-2-m0)
> - [stringify](#io-2-m1)
> - [stringify](#io-2-m2)
> - [stringify](#io-2-m3)
##### Static Method ``parse(string: _)`` <a id='io-2-m-1'></a>
``return _``

##### Static Method ``parse(source_id: _, source_string: _)`` <a id='io-2-m0'></a>
``return _``

##### Static Method ``stringify(value: _)`` <a id='io-2-m1'></a>
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
> - [peek](#io-4-m4)
> - [peek](#io-4-m5)
> - [peeks](#io-4-m6)
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

##### Method ``peek()`` <a id='io-4-m4'></a>
``return _``

##### Method ``peek(n: _)`` <a id='io-4-m5'></a>
``return _``

##### Method ``peeks()`` <a id='io-4-m6'></a>
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

## Module ``game``
### Classes
> - [Behaviour](#game--k-1)
> - [Component](#game--k0)
> - [Text](#game--k1)
> - [Sprite](#game--k2)
> - [Scene](#game--k3)
> - [Rigidbody](#game--k4)
> - [Animator](#game--k5)
> - [Transform](#game--k6)
> - [GameObject](#game--k7)
> - [Sfx](#game--k8)
> - [Line](#game--k9)
> - [ComponentBehaviour](#game--k10)
> - [Debug](#game--k11)
### Class ``Behaviour`` <a id='game--k-1'></a> 
> Inherits from ``Serializable``
>

#### Constructors
> - [new](#game-0-c-1)
#### Getters
> - [data](#game-0-g-1)
> - [[i]](#game-0-g0)
> - [frame](#game-0-g1)
> - [as_behaviour](#game-0-g2)
#### Setters
> - [data](#game-0-s-1)
> - [[i]](#game-0-s0)
> - [frame](#game-0-s1)
#### Methods
> - [start](#game-0-m-1)
> - [update](#game-0-m0)
> - [onCollision](#game-0-m1)
> - [setup](#game-0-m2)
> - [start](#game-0-m3)
> - [update](#game-0-m4)
##### Static Getter ``data`` <a id='game-0-g-1'></a>
``return Map``

##### Static Setter ``data = v: Any`` <a id='game-0-s-1'></a>

##### Static Getter ``[i]`` <a id='game-0-g0'></a>
``return Any``

##### Static Setter ``[i]: Any = v: Any`` <a id='game-0-s0'></a>

##### Getter ``frame`` <a id='game-0-g1'></a>
``return Num``

##### Setter ``frame = v: Num`` <a id='game-0-s1'></a>

##### Getter ``as_behaviour`` <a id='game-0-g2'></a>
``return ComponentBehaviour``
> Example:
> ```js
> gameobject.add(ParticleSystem.new(gameobject).as_behaviour)
> ```

##### Constructor ``new(g: GameObject, c: ComponentBehaviour)`` <a id='game-0-c-1'></a>
``return Behaviour``

##### Static Method ``start()`` <a id='game-0-m-1'></a>
``return Null``
> Runs the frame after setup.

##### Static Method ``update()`` <a id='game-0-m0'></a>
``return Null``
> Run every frame.

##### Static Method ``onCollision(collision: Map)`` <a id='game-0-m1'></a>
``return Null``
> Runs every frame after start that the Behaviour has a collision given a Rigidbody and Transform is attached.

##### Method ``setup()`` <a id='game-0-m2'></a>
``return Null``
> Runs the first frame regardless of whether or not the Behaviour is attached.

##### Method ``start()`` <a id='game-0-m3'></a>
``return Null``
> Runs the second frame regardless of whether or not the Behaviour is attached.

##### Method ``update()`` <a id='game-0-m4'></a>
``return Null``
> Runs every frame after start regardless of whether or not the Behaviour is attached.

### Foreign Class ``Component`` <a id='game--k0'></a> 
>
> Rust dyn obj that all components derive from

### Foreign Class ``Text`` <a id='game--k1'></a> 
>

#### Constructors
> - [new](#game-2-c-1)
#### Getters
> - [](#game-2-g-1)
> - [](#game-2-g0)
> - [](#game-2-g1)
> - [](#game-2-g2)
#### Setters
> - [text](#game-2-s-1)
> - [font](#game-2-s0)
> - [font_size](#game-2-s1)
#### Methods
> - [get_text](#game-2-m-1)
> - [get_font](#game-2-m0)
> - [get_font_size](#game-2-m1)
> - [set_text](#game-2-m2)
> - [set_font](#game-2-m3)
> - [set_font_size](#game-2-m4)
##### Constructor ``new(text: _, font: _)`` <a id='game-2-c-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-2-g-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-2-g0'></a>
``return _``

##### Foreign Getter ```` <a id='game-2-g1'></a>
``return _``

##### Foreign Getter ```` <a id='game-2-g2'></a>
``return _``

##### Setter ``text = value: _`` <a id='game-2-s-1'></a>

##### Setter ``font = value: _`` <a id='game-2-s0'></a>

##### Setter ``font_size = value: _`` <a id='game-2-s1'></a>

##### Foreign Static Method ``get_text(go: _)`` <a id='game-2-m-1'></a>
``return _``

##### Foreign Static Method ``get_font(go: _)`` <a id='game-2-m0'></a>
``return _``

##### Foreign Static Method ``get_font_size(go: _)`` <a id='game-2-m1'></a>
``return _``

##### Foreign Static Method ``set_text(go: _, text: _)`` <a id='game-2-m2'></a>
``return _``

##### Foreign Static Method ``set_font(go: _, font: _)`` <a id='game-2-m3'></a>
``return _``

##### Foreign Static Method ``set_font_size(go: _, fs: _)`` <a id='game-2-m4'></a>
``return _``

### Foreign Class ``Sprite`` <a id='game--k2'></a> 
>

#### Constructors
> - [new](#game-3-c-1)
#### Getters
> - [](#game-3-g-1)
> - [](#game-3-g0)
> - [](#game-3-g1)
> - [](#game-3-g2)
> - [](#game-3-g3)
> - [](#game-3-g4)
#### Setters
> - [tint](#game-3-s-1)
#### Methods
> - [cut_sprite_sheet](#game-3-m-1)
> - [cut_sprite_sheet](#game-3-m0)
> - [set_sort](#game-3-m1)
> - [set_tint](#game-3-m2)
##### Constructor ``new(id: _)`` <a id='game-3-c-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-3-g-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-3-g0'></a>
``return _``

##### Foreign Getter ```` <a id='game-3-g1'></a>
``return _``

##### Foreign Getter ```` <a id='game-3-g2'></a>
``return _``

##### Foreign Getter ```` <a id='game-3-g3'></a>
``return _``

##### Foreign Getter ```` <a id='game-3-g4'></a>
``return _``
> Returns in the form [r,g,b,a]

##### Setter ``tint = value: _`` <a id='game-3-s-1'></a>

##### Foreign Method ``cut_sprite_sheet(i: _, j: _)`` <a id='game-3-m-1'></a>
``return _``

##### Foreign Static Method ``cut_sprite_sheet(go: _, i: _, j: _)`` <a id='game-3-m0'></a>
``return _``

##### Foreign Static Method ``set_sort(go: _, i: _)`` <a id='game-3-m1'></a>
``return _``

##### Foreign Static Method ``set_tint(go: _, color: _)`` <a id='game-3-m2'></a>
``return _``

### Foreign Class ``Scene`` <a id='game--k3'></a> 
>

#### Constructors
> - [new](#game-4-c-1)
#### Getters
> - [](#game-4-g-1)
> - [](#game-4-g0)
> - [](#game-4-g1)
#### Methods
> - [getMarker](#game-4-m-1)
##### Constructor ``new(i: _)`` <a id='game-4-c-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-4-g-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-4-g0'></a>
``return _``

##### Foreign Getter ```` <a id='game-4-g1'></a>
``return _``

##### Method ``getMarker(index: _)`` <a id='game-4-m-1'></a>
``return _``
> either returns the Vec2 that is mapped to the String or a list of Vec2's if the String has multiple mappings.

### Foreign Class ``Rigidbody`` <a id='game--k4'></a> 
>

#### Constructors
> - [new](#game-5-c-1)
#### Getters
> - [](#game-5-g-1)
> - [](#game-5-g0)
> - [](#game-5-g1)
> - [](#game-5-g2)
> - [](#game-5-g3)
> - [](#game-5-g4)
#### Setters
> - [velocity](#game-5-s-1)
> - [solid](#game-5-s0)
> - [position](#game-5-s1)
#### Methods
> - [colliding](#game-5-m-1)
> - [set_solid](#game-5-m0)
> - [set_position](#game-5-m1)
> - [set_position_x](#game-5-m2)
> - [set_position_y](#game-5-m3)
> - [set_velocity](#game-5-m4)
> - [set_velocity_x](#game-5-m5)
> - [set_velocity_y](#game-5-m6)
> - [update_velocity](#game-5-m7)
> - [update_velocity_x](#game-5-m8)
> - [update_velocity_y](#game-5-m9)
> - [set_rotation](#game-5-m10)
##### Constructor ``new()`` <a id='game-5-c-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-5-g-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-5-g0'></a>
``return _``

##### Foreign Getter ```` <a id='game-5-g1'></a>
``return _``

##### Foreign Getter ```` <a id='game-5-g2'></a>
``return _``

##### Foreign Getter ```` <a id='game-5-g3'></a>
``return _``

##### Foreign Getter ```` <a id='game-5-g4'></a>
``return _``
> returns a map in the form "name": _, "uuid": _ or null if no collision

##### Setter ``velocity = value: _`` <a id='game-5-s-1'></a>

##### Setter ``solid = value: _`` <a id='game-5-s0'></a>

##### Setter ``position = value: _`` <a id='game-5-s1'></a>

##### Foreign Static Method ``colliding(go: _)`` <a id='game-5-m-1'></a>
``return _``
> returns a map in the form "name": _, "uuid": _ or null if no collision

##### Foreign Static Method ``set_solid(go: _, solid: _)`` <a id='game-5-m0'></a>
``return _``

##### Foreign Static Method ``set_position(go: _, new_pos: _)`` <a id='game-5-m1'></a>
``return _``

##### Foreign Static Method ``set_position_x(go: _, new_x: _)`` <a id='game-5-m2'></a>
``return _``

##### Foreign Static Method ``set_position_y(go: _, new_y: _)`` <a id='game-5-m3'></a>
``return _``

##### Foreign Static Method ``set_velocity(go: _, vel: _)`` <a id='game-5-m4'></a>
``return _``

##### Foreign Static Method ``set_velocity_x(go: _, new_x: _)`` <a id='game-5-m5'></a>
``return _``

##### Foreign Static Method ``set_velocity_y(go: _, new_y: _)`` <a id='game-5-m6'></a>
``return _``

##### Foreign Static Method ``update_velocity(go: _, vel: _)`` <a id='game-5-m7'></a>
``return _``

##### Foreign Static Method ``update_velocity_x(go: _, new_x: _)`` <a id='game-5-m8'></a>
``return _``

##### Foreign Static Method ``update_velocity_y(go: _, new_y: _)`` <a id='game-5-m9'></a>
``return _``

##### Foreign Static Method ``set_rotation(go: _, new_rot: _)`` <a id='game-5-m10'></a>
``return _``

### Foreign Class ``Animator`` <a id='game--k5'></a> 
>

#### Constructors
> - [new](#game-6-c-1)
#### Getters
> - [](#game-6-g-1)
> - [](#game-6-g0)
> - [](#game-6-g1)
> - [](#game-6-g2)
> - [](#game-6-g3)
#### Setters
> - [speed](#game-6-s-1)
> - [frame](#game-6-s0)
#### Methods
> - [play](#game-6-m-1)
> - [stop](#game-6-m0)
> - [get_state](#game-6-m1)
> - [set_state](#game-6-m2)
> - [insert_state](#game-6-m3)
> - [play](#game-6-m4)
> - [stop](#game-6-m5)
> - [set_state](#game-6-m6)
> - [get_state](#game-6-m7)
> - [insert_state](#game-6-m8)
> - [set_speed](#game-6-m9)
> - [set_frame](#game-6-m10)
##### Constructor ``new()`` <a id='game-6-c-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-6-g-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-6-g0'></a>
``return _``

##### Foreign Getter ```` <a id='game-6-g1'></a>
``return _``

##### Foreign Getter ```` <a id='game-6-g2'></a>
``return _``

##### Setter ``speed = value: _`` <a id='game-6-s-1'></a>

##### Foreign Getter ```` <a id='game-6-g3'></a>
``return _``

##### Setter ``frame = value: _`` <a id='game-6-s0'></a>

##### Foreign Method ``play()`` <a id='game-6-m-1'></a>
``return _``

##### Foreign Method ``stop()`` <a id='game-6-m0'></a>
``return _``

##### Foreign Method ``get_state(s: _)`` <a id='game-6-m1'></a>
``return _``
> returns map in the form state: value:Vec2

##### Foreign Method ``set_state(s: _)`` <a id='game-6-m2'></a>
``return _``

##### Foreign Method ``insert_state(s: _, i: _)`` <a id='game-6-m3'></a>
``return _``

##### Foreign Static Method ``play(g: _)`` <a id='game-6-m4'></a>
``return _``

##### Foreign Static Method ``stop(g: _)`` <a id='game-6-m5'></a>
``return _``

##### Foreign Static Method ``set_state(g: _, s: _)`` <a id='game-6-m6'></a>
``return _``

##### Foreign Static Method ``get_state(g: _, s: _)`` <a id='game-6-m7'></a>
``return _``

##### Foreign Static Method ``insert_state(g: _, s: _, i: _)`` <a id='game-6-m8'></a>
``return _``

##### Foreign Static Method ``set_speed(g: _, s: _)`` <a id='game-6-m9'></a>
``return _``

##### Foreign Static Method ``set_frame(g: _, f: _)`` <a id='game-6-m10'></a>
``return _``

### Foreign Class ``Transform`` <a id='game--k6'></a> 
> Inherits from ``Serializable``
>

#### Constructors
> - [new](#game-7-c-1)
#### Getters
> - [](#game-7-g-1)
> - [](#game-7-g0)
> - [](#game-7-g1)
> - [](#game-7-g2)
> - [](#game-7-g3)
> - [](#game-7-g4)
> - [default](#game-7-g5)
#### Setters
> - [position](#game-7-s-1)
> - [scale](#game-7-s0)
> - [rotation](#game-7-s1)
> - [pivot](#game-7-s2)
#### Methods
> - [getProperty](#game-7-m-1)
> - [setProperty](#game-7-m0)
> - [set_pivot](#game-7-m1)
> - [set_position](#game-7-m2)
> - [set_position_x](#game-7-m3)
> - [set_position_y](#game-7-m4)
> - [update_position](#game-7-m5)
> - [update_position_x](#game-7-m6)
> - [update_position_y](#game-7-m7)
> - [set_scale](#game-7-m8)
> - [set_scale_x](#game-7-m9)
> - [set_scale_y](#game-7-m10)
> - [update_scale](#game-7-m11)
> - [update_scale_x](#game-7-m12)
> - [update_scale_y](#game-7-m13)
> - [set_rotation](#game-7-m14)
> - [update_rotation](#game-7-m15)
> - [inverse_point](#game-7-m16)
##### Constructor ``new(p: _)`` <a id='game-7-c-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-7-g-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-7-g0'></a>
``return _``

##### Foreign Getter ```` <a id='game-7-g1'></a>
``return _``

##### Foreign Getter ```` <a id='game-7-g2'></a>
``return _``

##### Foreign Getter ```` <a id='game-7-g3'></a>
``return _``

##### Foreign Getter ```` <a id='game-7-g4'></a>
``return _``

##### Setter ``position = value: _`` <a id='game-7-s-1'></a>

##### Setter ``scale = value: _`` <a id='game-7-s0'></a>

##### Setter ``rotation = value: _`` <a id='game-7-s1'></a>

##### Setter ``pivot = value: _`` <a id='game-7-s2'></a>

##### Static Getter ``default`` <a id='game-7-g5'></a>
``return _``

##### Method ``getProperty()`` <a id='game-7-m-1'></a>
``return _``

##### Method ``setProperty()`` <a id='game-7-m0'></a>
``return _``

##### Foreign Static Method ``set_pivot(go: _, new_pivot: _)`` <a id='game-7-m1'></a>
``return _``

##### Foreign Static Method ``set_position(go: _, new_pos: _)`` <a id='game-7-m2'></a>
``return _``

##### Foreign Static Method ``set_position_x(go: _, new_x: _)`` <a id='game-7-m3'></a>
``return _``

##### Foreign Static Method ``set_position_y(go: _, new_y: _)`` <a id='game-7-m4'></a>
``return _``

##### Foreign Static Method ``update_position(go: _, new_pos: _)`` <a id='game-7-m5'></a>
``return _``

##### Foreign Static Method ``update_position_x(go: _, new_x: _)`` <a id='game-7-m6'></a>
``return _``

##### Foreign Static Method ``update_position_y(go: _, new_y: _)`` <a id='game-7-m7'></a>
``return _``

##### Foreign Static Method ``set_scale(go: _, new_scale: _)`` <a id='game-7-m8'></a>
``return _``

##### Foreign Static Method ``set_scale_x(go: _, new_x: _)`` <a id='game-7-m9'></a>
``return _``

##### Foreign Static Method ``set_scale_y(go: _, new_y: _)`` <a id='game-7-m10'></a>
``return _``

##### Foreign Static Method ``update_scale(go: _, new_scale: _)`` <a id='game-7-m11'></a>
``return _``

##### Foreign Static Method ``update_scale_x(go: _, new_x: _)`` <a id='game-7-m12'></a>
``return _``

##### Foreign Static Method ``update_scale_y(go: _, new_y: _)`` <a id='game-7-m13'></a>
``return _``

##### Foreign Static Method ``set_rotation(go: _, new_rot: _)`` <a id='game-7-m14'></a>
``return _``

##### Foreign Static Method ``update_rotation(go: _, new_rot: _)`` <a id='game-7-m15'></a>
``return _``

##### Foreign Static Method ``inverse_point(go: _, point: _)`` <a id='game-7-m16'></a>
``return _``

### Foreign Class ``GameObject`` <a id='game--k7'></a> 
>

#### Constructors
> - [new](#game-8-c-1)
#### Getters
> - [](#game-8-g-1)
> - [](#game-8-g0)
> - [](#game-8-g1)
> - [](#game-8-g2)
> - [toString](#game-8-g3)
#### Setters
> - [name](#game-8-s-1)
#### Methods
> - [addComponent](#game-8-m-1)
> - [getComponent](#game-8-m0)
> - [get](#game-8-m1)
> - [add](#game-8-m2)
##### Constructor ``new(name: _)`` <a id='game-8-c-1'></a>
``return _``

##### Foreign Method ``addComponent(x: _)`` <a id='game-8-m-1'></a>
``return _``

##### Foreign Method ``getComponent(x: _)`` <a id='game-8-m0'></a>
``return _``

##### Method ``get(x: _)`` <a id='game-8-m1'></a>
``return _``

##### Method ``add(x: _)`` <a id='game-8-m2'></a>
``return _``

##### Foreign Getter ```` <a id='game-8-g-1'></a>
``return _``
> Returns a map in the form "name": _, "uuid": _

##### Foreign Getter ```` <a id='game-8-g0'></a>
``return _``

##### Foreign Getter ```` <a id='game-8-g1'></a>
``return _``

##### Foreign Setter ``name = v: _`` <a id='game-8-s-1'></a>

##### Foreign Getter ```` <a id='game-8-g2'></a>
``return _``

##### Getter ``toString`` <a id='game-8-g3'></a>
``return _``

### Foreign Class ``Sfx`` <a id='game--k8'></a> 
>

#### Constructors
> - [new](#game-9-c-1)
#### Getters
> - [](#game-9-g-1)
> - [](#game-9-g0)
> - [](#game-9-g1)
> - [](#game-9-g2)
> - [](#game-9-g3)
#### Setters
> - [name](#game-9-s-1)
> - [volume](#game-9-s0)
#### Methods
> - [play](#game-9-m-1)
> - [get_volume](#game-9-m0)
> - [set_volume](#game-9-m1)
> - [play](#game-9-m2)
##### Constructor ``new(name: _, file: _)`` <a id='game-9-c-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-9-g-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-9-g0'></a>
``return _``

##### Foreign Getter ```` <a id='game-9-g1'></a>
``return _``

##### Foreign Setter ``name = v: _`` <a id='game-9-s-1'></a>

##### Foreign Getter ```` <a id='game-9-g2'></a>
``return _``

##### Setter ``volume = value: _`` <a id='game-9-s0'></a>

##### Foreign Getter ```` <a id='game-9-g3'></a>
``return _``

##### Foreign Method ``play()`` <a id='game-9-m-1'></a>
``return _``

##### Foreign Static Method ``get_volume(go: _, name: _)`` <a id='game-9-m0'></a>
``return _``

##### Foreign Static Method ``set_volume(go: _, name: _, amt: _)`` <a id='game-9-m1'></a>
``return _``

##### Foreign Static Method ``play(go: _, name: _)`` <a id='game-9-m2'></a>
``return _``

### Foreign Class ``Line`` <a id='game--k9'></a> 
>

#### Constructors
> - [new](#game-10-c-1)
#### Getters
> - [](#game-10-g-1)
> - [](#game-10-g0)
> - [](#game-10-g1)
> - [](#game-10-g2)
> - [](#game-10-g3)
> - [](#game-10-g4)
> - [](#game-10-g5)
#### Setters
> - [sort](#game-10-s-1)
> - [color](#game-10-s0)
> - [opacity](#game-10-s1)
> - [thickness](#game-10-s2)
#### Methods
> - [set_sort](#game-10-m-1)
> - [get_sort](#game-10-m0)
> - [set_thickness](#game-10-m1)
> - [get_thickness](#game-10-m2)
> - [set_color](#game-10-m3)
> - [set_opacity](#game-10-m4)
> - [add_point](#game-10-m5)
> - [remove_point](#game-10-m6)
> - [pop_point](#game-10-m7)
> - [insert_point](#game-10-m8)
> - [set_point](#game-10-m9)
##### Constructor ``new()`` <a id='game-10-c-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-10-g-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-10-g0'></a>
``return _``

##### Foreign Getter ```` <a id='game-10-g1'></a>
``return _``

##### Foreign Getter ```` <a id='game-10-g2'></a>
``return _``

##### Foreign Getter ```` <a id='game-10-g3'></a>
``return _``

##### Foreign Getter ```` <a id='game-10-g4'></a>
``return _``
> Gets line thickness in form [start, end]

##### Foreign Getter ```` <a id='game-10-g5'></a>
``return _``

##### Setter ``sort = value: _`` <a id='game-10-s-1'></a>

##### Setter ``color = value: _`` <a id='game-10-s0'></a>

##### Setter ``opacity = value: _`` <a id='game-10-s1'></a>

##### Setter ``thickness = value: _`` <a id='game-10-s2'></a>

##### Foreign Static Method ``set_sort(go: _, sort: _)`` <a id='game-10-m-1'></a>
``return _``

##### Foreign Static Method ``get_sort(go: _)`` <a id='game-10-m0'></a>
``return _``

##### Foreign Static Method ``set_thickness(go: _, thickness: _)`` <a id='game-10-m1'></a>
``return _``

##### Foreign Static Method ``get_thickness(go: _)`` <a id='game-10-m2'></a>
``return _``

##### Foreign Static Method ``set_color(go: _, color: _)`` <a id='game-10-m3'></a>
``return _``

##### Foreign Static Method ``set_opacity(go: _, opacity: _)`` <a id='game-10-m4'></a>
``return _``

##### Foreign Static Method ``add_point(go: _, point: _)`` <a id='game-10-m5'></a>
``return _``

##### Foreign Static Method ``remove_point(go: _, index: _)`` <a id='game-10-m6'></a>
``return _``

##### Foreign Static Method ``pop_point(go: _)`` <a id='game-10-m7'></a>
``return _``

##### Foreign Static Method ``insert_point(go: _, point: _, index: _)`` <a id='game-10-m8'></a>
``return _``

##### Foreign Static Method ``set_point(go: _, index: _, point: _)`` <a id='game-10-m9'></a>
``return _``

### Foreign Class ``ComponentBehaviour`` <a id='game--k10'></a> 
>

#### Constructors
> - [new](#game-11-c-1)
#### Getters
> - [](#game-11-g-1)
> - [](#game-11-g0)
> - [](#game-11-g1)
##### Constructor ``new(b: _)`` <a id='game-11-c-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-11-g-1'></a>
``return _``

##### Foreign Getter ```` <a id='game-11-g0'></a>
``return _``

##### Foreign Getter ```` <a id='game-11-g1'></a>
``return _``

### Foreign Class ``Debug`` <a id='game--k11'></a> 
>

#### Methods
> - [drawLine](#game-12-m-1)
> - [printFrameInfo](#game-12-m0)
##### Foreign Static Method ``drawLine(start: _, end: _, color: _)`` <a id='game-12-m-1'></a>
``return _``

##### Static Method ``printFrameInfo()`` <a id='game-12-m0'></a>
``return _``

## Module ``ParticleSystem``
### Classes
> - [ParticleField](#particlesystem--k-1)
> - [ParticleSystem](#particlesystem--k0)
### Class ``ParticleField`` <a id='particlesystem--k-1'></a>
>

#### Constructors
> - [new](#particlesystem-0-c-1)
> - [new](#particlesystem-0-c0)
#### Getters
> - [raw](#particlesystem-0-g-1)
> - [value](#particlesystem-0-g0)
> - [[t]](#particlesystem-0-g1)
> - [curve](#particlesystem-0-g2)
#### Setters
> - [value](#particlesystem-0-s-1)
> - [[t]](#particlesystem-0-s0)
> - [curve](#particlesystem-0-s1)
##### Getter ``raw`` <a id='particlesystem-0-g-1'></a>
``return _``

##### Getter ``value`` <a id='particlesystem-0-g0'></a>
``return _``

##### Setter ``value = v: _`` <a id='particlesystem-0-s-1'></a>

##### Setter ``[t]`` <a id='particlesystem-0-s0'></a>

##### Getter ``[t]`` <a id='particlesystem-0-g1'></a>
``return _``

##### Getter ``curve`` <a id='particlesystem-0-g2'></a>
``return _``

##### Setter ``curve = v: _`` <a id='particlesystem-0-s1'></a>

##### Constructor ``new(v: _, c: _)`` <a id='particlesystem-0-c-1'></a>
``return _``

##### Constructor ``new(v: _)`` <a id='particlesystem-0-c0'></a>
``return _``

### Class ``ParticleSystem`` <a id='particlesystem--k0'></a> 
> Inherits from ``Behaviour``
>

#### Constructors
> - [new](#particlesystem-1-c-1)
> - [new](#particlesystem-1-c0)
#### Getters
> - [gameobject](#particlesystem-1-g-1)
> - [gamebehaviour](#particlesystem-1-g0)
> - [default](#particlesystem-1-g1)
> - [lifeSpan](#particlesystem-1-g2)
> - [rate](#particlesystem-1-g3)
> - [speed](#particlesystem-1-g4)
> - [rotation](#particlesystem-1-g5)
> - [direction](#particlesystem-1-g6)
> - [scale](#particlesystem-1-g7)
> - [color](#particlesystem-1-g8)
> - [distance](#particlesystem-1-g9)
> - [partSetup](#particlesystem-1-g10)
> - [partStart](#particlesystem-1-g11)
> - [internal_time](#particlesystem-1-g12)
> - [parts](#particlesystem-1-g13)
> - [internal_pos](#particlesystem-1-g14)
#### Setters
> - [gameobject](#particlesystem-1-s-1)
> - [gamebehaviour](#particlesystem-1-s0)
> - [lifeSpan](#particlesystem-1-s1)
> - [rate](#particlesystem-1-s2)
> - [speed](#particlesystem-1-s3)
> - [rotation](#particlesystem-1-s4)
> - [direction](#particlesystem-1-s5)
> - [scale](#particlesystem-1-s6)
> - [color](#particlesystem-1-s7)
> - [distance](#particlesystem-1-s8)
> - [partSetup](#particlesystem-1-s9)
> - [partStart](#particlesystem-1-s10)
> - [internal_time](#particlesystem-1-s11)
> - [parts](#particlesystem-1-s12)
> - [internal_pos](#particlesystem-1-s13)
#### Methods
> - [play](#particlesystem-1-m-1)
> - [stop](#particlesystem-1-m0)
> - [toggle](#particlesystem-1-m1)
> - [isPlaying](#particlesystem-1-m2)
> - [emit](#particlesystem-1-m3)
> - [update](#particlesystem-1-m4)
##### Static Getter ``gameobject`` <a id='particlesystem-1-g-1'></a>
``return _``

##### Static Setter ``gameobject = v: _`` <a id='particlesystem-1-s-1'></a>

##### Static Getter ``gamebehaviour`` <a id='particlesystem-1-g0'></a>
``return _``

##### Static Setter ``gamebehaviour = v: _`` <a id='particlesystem-1-s0'></a>

##### Constructor ``new(g: _)`` <a id='particlesystem-1-c-1'></a>
``return _``

##### Static Getter ``default`` <a id='particlesystem-1-g1'></a>
``return _``

##### Getter ``lifeSpan`` <a id='particlesystem-1-g2'></a>
``return _``

##### Setter ``lifeSpan = v: _`` <a id='particlesystem-1-s1'></a>

##### Getter ``rate`` <a id='particlesystem-1-g3'></a>
``return _``

##### Setter ``rate = v: _`` <a id='particlesystem-1-s2'></a>

##### Getter ``speed`` <a id='particlesystem-1-g4'></a>
``return _``

##### Setter ``speed = v: _`` <a id='particlesystem-1-s3'></a>

##### Getter ``rotation`` <a id='particlesystem-1-g5'></a>
``return _``

##### Setter ``rotation = v: _`` <a id='particlesystem-1-s4'></a>

##### Getter ``direction`` <a id='particlesystem-1-g6'></a>
``return _``

##### Setter ``direction = v: _`` <a id='particlesystem-1-s5'></a>

##### Getter ``scale`` <a id='particlesystem-1-g7'></a>
``return _``

##### Setter ``scale = v: _`` <a id='particlesystem-1-s6'></a>

##### Getter ``color`` <a id='particlesystem-1-g8'></a>
``return _``

##### Setter ``color = v: _`` <a id='particlesystem-1-s7'></a>

##### Getter ``distance`` <a id='particlesystem-1-g9'></a>
``return _``

##### Setter ``distance = v: _`` <a id='particlesystem-1-s8'></a>

##### Getter ``partSetup`` <a id='particlesystem-1-g10'></a>
``return _``

##### Setter ``partSetup = v: _`` <a id='particlesystem-1-s9'></a>

##### Getter ``partStart`` <a id='particlesystem-1-g11'></a>
``return _``

##### Setter ``partStart = v: _`` <a id='particlesystem-1-s10'></a>

##### Getter ``internal_time`` <a id='particlesystem-1-g12'></a>
``return _``

##### Setter ``internal_time = v: _`` <a id='particlesystem-1-s11'></a>

##### Getter ``parts`` <a id='particlesystem-1-g13'></a>
``return _``

##### Setter ``parts = v: _`` <a id='particlesystem-1-s12'></a>

##### Getter ``internal_pos`` <a id='particlesystem-1-g14'></a>
``return _``

##### Setter ``internal_pos = v: _`` <a id='particlesystem-1-s13'></a>

##### Method ``play()`` <a id='particlesystem-1-m-1'></a>
``return _``

##### Method ``stop()`` <a id='particlesystem-1-m0'></a>
``return _``

##### Method ``toggle()`` <a id='particlesystem-1-m1'></a>
``return _``

##### Method ``isPlaying()`` <a id='particlesystem-1-m2'></a>
``return _``

##### Constructor ``new()`` <a id='particlesystem-1-c0'></a>
``return _``

##### Static Method ``emit()`` <a id='particlesystem-1-m3'></a>
``return _``

##### Static Method ``update()`` <a id='particlesystem-1-m4'></a>
``return _``

## Module ``Trail``
### Classes
> - [TrailField](#trail--k-1)
> - [Trail](#trail--k0)
### Class ``TrailField`` <a id='trail--k-1'></a>
>

#### Constructors
> - [new](#trail-0-c-1)
> - [new](#trail-0-c0)
#### Getters
> - [raw](#trail-0-g-1)
> - [value](#trail-0-g0)
> - [[t]](#trail-0-g1)
> - [curve](#trail-0-g2)
#### Setters
> - [value](#trail-0-s-1)
> - [[t]](#trail-0-s0)
> - [curve](#trail-0-s1)
##### Getter ``raw`` <a id='trail-0-g-1'></a>
``return _``

##### Getter ``value`` <a id='trail-0-g0'></a>
``return _``

##### Setter ``value = v: _`` <a id='trail-0-s-1'></a>

##### Setter ``[t]`` <a id='trail-0-s0'></a>

##### Getter ``[t]`` <a id='trail-0-g1'></a>
``return _``

##### Getter ``curve`` <a id='trail-0-g2'></a>
``return _``

##### Setter ``curve = v: _`` <a id='trail-0-s1'></a>

##### Constructor ``new(v: _, c: _)`` <a id='trail-0-c-1'></a>
``return _``

##### Constructor ``new(v: _)`` <a id='trail-0-c0'></a>
``return _``

### Class ``Trail`` <a id='trail--k0'></a> 
> Inherits from ``Behaviour``
>

#### Constructors
> - [new](#trail-1-c-1)
> - [new](#trail-1-c0)
#### Getters
> - [gameobject](#trail-1-g-1)
> - [gamebehaviour](#trail-1-g0)
> - [default](#trail-1-g1)
> - [minDistance](#trail-1-g2)
> - [maxCount](#trail-1-g3)
> - [hist](#trail-1-g4)
#### Setters
> - [gameobject](#trail-1-s-1)
> - [gamebehaviour](#trail-1-s0)
> - [minDistance](#trail-1-s1)
> - [maxCount](#trail-1-s2)
> - [hist](#trail-1-s3)
#### Methods
> - [start](#trail-1-m-1)
> - [update](#trail-1-m0)
##### Static Getter ``gameobject`` <a id='trail-1-g-1'></a>
``return _``

##### Static Setter ``gameobject = v: _`` <a id='trail-1-s-1'></a>

##### Static Getter ``gamebehaviour`` <a id='trail-1-g0'></a>
``return _``

##### Static Setter ``gamebehaviour = v: _`` <a id='trail-1-s0'></a>

##### Constructor ``new(g: _)`` <a id='trail-1-c-1'></a>
``return _``

##### Static Getter ``default`` <a id='trail-1-g1'></a>
``return _``

##### Getter ``minDistance`` <a id='trail-1-g2'></a>
``return _``

##### Setter ``minDistance = v: _`` <a id='trail-1-s1'></a>

##### Getter ``maxCount`` <a id='trail-1-g3'></a>
``return _``

##### Setter ``maxCount = v: _`` <a id='trail-1-s2'></a>

##### Getter ``hist`` <a id='trail-1-g4'></a>
``return _``

##### Setter ``hist = v: _`` <a id='trail-1-s3'></a>

##### Constructor ``new()`` <a id='trail-1-c0'></a>
``return _``

##### Static Method ``start()`` <a id='trail-1-m-1'></a>
``return _``

##### Static Method ``update()`` <a id='trail-1-m0'></a>
``return _``

## Module ``app``
### Classes
> - [GameObjectRef](#app--k-1)
> - [Lilah](#app--k0)
> - [Audio](#app--k1)
> - [KeycodeLookup](#app--k2)
> - [Input](#app--k3)
> - [UI](#app--k4)
> - [Curve](#app--k5)
> - [Tween](#app--k6)
### Class ``GameObjectRef`` <a id='app--k-1'></a>
>

#### Constructors
> - [new](#app-0-c-1)
#### Getters
> - [ref](#app-0-g-1)
> - [data](#app-0-g0)
> - [[key]](#app-0-g1)
#### Setters
> - [data](#app-0-s-1)
> - [[key]](#app-0-s0)
#### Methods
> - [create_ref](#app-0-m-1)
> - [behaviourData](#app-0-m0)
> - [behaviourData](#app-0-m1)
> - [behaviourData](#app-0-m2)
##### Static Method ``create_ref(id: _)`` <a id='app-0-m-1'></a>
``return _``

##### Getter ``ref`` <a id='app-0-g-1'></a>
``return _``

##### Method ``behaviourData(b: _)`` <a id='app-0-m0'></a>
``return _``

##### Method ``behaviourData(b: _, uuid: _)`` <a id='app-0-m1'></a>
``return _``

##### Method ``behaviourData(b: _, u: _, mut: _)`` <a id='app-0-m2'></a>
``return _``

##### Setter ``data = v: _`` <a id='app-0-s-1'></a>

##### Getter ``data`` <a id='app-0-g0'></a>
``return _``

##### Getter ``[key]`` <a id='app-0-g1'></a>
``return _``

##### Setter ``[key]`` <a id='app-0-s0'></a>

##### Constructor ``new(i: _)`` <a id='app-0-c-1'></a>
``return _``

### Class ``Lilah`` <a id='app--k0'></a>
>

#### Getters
> - [camera](#app-1-g-1)
> - [destroy](#app-1-g0)
> - [destroy_internal](#app-1-g1)
> - [gameobjects](#app-1-g2)
> - [gameobjects_values](#app-1-g3)
> - [data](#app-1-g4)
> - [delta_time](#app-1-g5)
> - [time](#app-1-g6)
> - [fps](#app-1-g7)
> - [fullscreen](#app-1-g8)
> - [screen_size](#app-1-g9)
> - [fiberCount](#app-1-g10)
#### Setters
> - [gameobjects](#app-1-s-1)
> - [data](#app-1-s0)
> - [delta_time](#app-1-s1)
> - [time](#app-1-s2)
> - [fps](#app-1-s3)
> - [fullscreen](#app-1-s4)
> - [screen_size](#app-1-s5)
#### Methods
> - [tick_fibers](#app-1-m-1)
> - [start_fiber](#app-1-m0)
> - [instantiate](#app-1-m1)
> - [instantiate](#app-1-m2)
> - [clear](#app-1-m3)
> - [destroy](#app-1-m4)
> - [find](#app-1-m5)
##### Static Getter ``camera`` <a id='app-1-g-1'></a>
``return _``

##### Static Getter ``destroy`` <a id='app-1-g0'></a>
``return _``

##### Static Getter ``destroy_internal`` <a id='app-1-g1'></a>
``return _``

##### Static Getter ``gameobjects`` <a id='app-1-g2'></a>
``return _``

##### Static Setter ``gameobjects = v: _`` <a id='app-1-s-1'></a>

##### Static Getter ``gameobjects_values`` <a id='app-1-g3'></a>
``return _``

##### Static Getter ``data`` <a id='app-1-g4'></a>
``return _``

##### Static Setter ``data = v: _`` <a id='app-1-s0'></a>

##### Static Getter ``delta_time`` <a id='app-1-g5'></a>
``return _``

##### Static Setter ``delta_time = v: _`` <a id='app-1-s1'></a>

##### Static Getter ``time`` <a id='app-1-g6'></a>
``return _``

##### Static Setter ``time = v: _`` <a id='app-1-s2'></a>

##### Static Getter ``fps`` <a id='app-1-g7'></a>
``return _``

##### Static Setter ``fps = v: _`` <a id='app-1-s3'></a>

##### Static Getter ``fullscreen`` <a id='app-1-g8'></a>
``return _``

##### Static Setter ``fullscreen = v: _`` <a id='app-1-s4'></a>

##### Static Getter ``screen_size`` <a id='app-1-g9'></a>
``return _``

##### Static Setter ``screen_size = v: _`` <a id='app-1-s5'></a>

##### Static Getter ``fiberCount`` <a id='app-1-g10'></a>
``return _``

##### Static Method ``tick_fibers()`` <a id='app-1-m-1'></a>
``return _``

##### Static Method ``start_fiber(f: _)`` <a id='app-1-m0'></a>
``return _``

##### Static Method ``instantiate(go: _, d: _)`` <a id='app-1-m1'></a>
``return _``

##### Static Method ``instantiate(go: _)`` <a id='app-1-m2'></a>
``return _``

##### Static Method ``clear()`` <a id='app-1-m3'></a>
``return _``

##### Static Method ``destroy(key: _)`` <a id='app-1-m4'></a>
``return _``

##### Static Method ``find(key: _)`` <a id='app-1-m5'></a>
``return _``

### Class ``Audio`` <a id='app--k1'></a>
>

#### Getters
> - [music](#app-2-g-1)
> - [command](#app-2-g0)
> - [dirty](#app-2-g1)
> - [volume](#app-2-g2)
> - [fade](#app-2-g3)
#### Setters
> - [volume](#app-2-s-1)
#### Methods
> - [play](#app-2-m-1)
> - [play](#app-2-m0)
> - [play](#app-2-m1)
> - [pause](#app-2-m2)
> - [pause](#app-2-m3)
> - [clear](#app-2-m4)
##### Static Getter ``music`` <a id='app-2-g-1'></a>
``return _``

##### Static Getter ``command`` <a id='app-2-g0'></a>
``return _``

##### Static Getter ``dirty`` <a id='app-2-g1'></a>
``return _``

##### Static Getter ``volume`` <a id='app-2-g2'></a>
``return _``

##### Static Getter ``fade`` <a id='app-2-g3'></a>
``return _``

##### Static Setter ``volume = v: _`` <a id='app-2-s-1'></a>

##### Static Method ``play(file: _)`` <a id='app-2-m-1'></a>
``return _``

##### Static Method ``play(file: _, fade_in_ms: _)`` <a id='app-2-m0'></a>
``return _``

##### Static Method ``play()`` <a id='app-2-m1'></a>
``return _``

##### Static Method ``pause()`` <a id='app-2-m2'></a>
``return _``

##### Static Method ``pause(fade_out_ms: _)`` <a id='app-2-m3'></a>
``return _``

##### Static Method ``clear()`` <a id='app-2-m4'></a>
``return _``

### Class ``KeycodeLookup`` <a id='app--k2'></a>
>

#### Constructors
> - [new](#app-3-c-1)
#### Getters
> - [W](#app-3-g-1)
> - [A](#app-3-g0)
> - [S](#app-3-g1)
> - [D](#app-3-g2)
> - [Up](#app-3-g3)
> - [Right](#app-3-g4)
> - [Down](#app-3-g5)
> - [Left](#app-3-g6)
##### Constructor ``new()`` <a id='app-3-c-1'></a>
``return _``

##### Getter ``W`` <a id='app-3-g-1'></a>
``return _``

##### Getter ``A`` <a id='app-3-g0'></a>
``return _``

##### Getter ``S`` <a id='app-3-g1'></a>
``return _``

##### Getter ``D`` <a id='app-3-g2'></a>
``return _``

##### Getter ``Up`` <a id='app-3-g3'></a>
``return _``

##### Getter ``Right`` <a id='app-3-g4'></a>
``return _``

##### Getter ``Down`` <a id='app-3-g5'></a>
``return _``

##### Getter ``Left`` <a id='app-3-g6'></a>
``return _``

### Class ``Input`` <a id='app--k3'></a>
>

#### Getters
> - [mouse_pos](#app-4-g-1)
> - [Keycode](#app-4-g0)
> - [mappings](#app-4-g1)
> - [mouse_mappings](#app-4-g2)
> - [bindings](#app-4-g3)
#### Methods
> - [is_pressed](#app-4-m-1)
> - [is_mouse_pressed](#app-4-m0)
> - [set_mouse_pos](#app-4-m1)
> - [update_mapping](#app-4-m2)
> - [update_mouse_mapping](#app-4-m3)
> - [update_binding](#app-4-m4)
> - [key](#app-4-m5)
> - [mouse](#app-4-m6)
> - [key_down](#app-4-m7)
> - [mouse_down](#app-4-m8)
> - [binding](#app-4-m9)
> - [binding2D](#app-4-m10)
##### Static Getter ``mouse_pos`` <a id='app-4-g-1'></a>
``return _``

##### Static Getter ``Keycode`` <a id='app-4-g0'></a>
``return _``

##### Static Method ``is_pressed(key: _)`` <a id='app-4-m-1'></a>
``return _``

##### Static Method ``is_mouse_pressed(key: _)`` <a id='app-4-m0'></a>
``return _``

##### Static Getter ``mappings`` <a id='app-4-g1'></a>
``return _``

##### Static Getter ``mouse_mappings`` <a id='app-4-g2'></a>
``return _``

##### Static Getter ``bindings`` <a id='app-4-g3'></a>
``return _``

##### Static Method ``set_mouse_pos(pos: _)`` <a id='app-4-m1'></a>
``return _``

##### Static Method ``update_mapping(key: _, pressed: _, pressed_down: _)`` <a id='app-4-m2'></a>
``return _``

##### Static Method ``update_mouse_mapping(button: _, pressed: _, pressed_down: _)`` <a id='app-4-m3'></a>
``return _``

##### Static Method ``update_binding(bind: _, neg: _, pos: _)`` <a id='app-4-m4'></a>
``return _``

##### Static Method ``key(key: _)`` <a id='app-4-m5'></a>
``return _``

##### Static Method ``mouse(button: _)`` <a id='app-4-m6'></a>
``return _``

##### Static Method ``key_down(key: _)`` <a id='app-4-m7'></a>
``return _``

##### Static Method ``mouse_down(button: _)`` <a id='app-4-m8'></a>
``return _``

##### Static Method ``binding(bind: _)`` <a id='app-4-m9'></a>
``return _``

##### Static Method ``binding2D(bind1: _, bind2: _)`` <a id='app-4-m10'></a>
``return _``

### Class ``UI`` <a id='app--k4'></a>
>

#### Getters
> - [on_click_callbacks](#app-5-g-1)
> - [on_click_down_callbacks](#app-5-g0)
> - [on_hover_callbacks](#app-5-g1)
#### Methods
> - [on_click](#app-5-m-1)
> - [on_click_down](#app-5-m0)
> - [on_hover](#app-5-m1)
> - [tick](#app-5-m2)
##### Static Getter ``on_click_callbacks`` <a id='app-5-g-1'></a>
``return _``

##### Static Getter ``on_click_down_callbacks`` <a id='app-5-g0'></a>
``return _``

##### Static Getter ``on_hover_callbacks`` <a id='app-5-g1'></a>
``return _``

##### Static Method ``on_click(gameobject: _, callback: _)`` <a id='app-5-m-1'></a>
``return _``

##### Static Method ``on_click_down(gameobject: _, callback: _)`` <a id='app-5-m0'></a>
``return _``

##### Static Method ``on_hover(gameobject: _, callback: _)`` <a id='app-5-m1'></a>
``return _``

##### Static Method ``tick()`` <a id='app-5-m2'></a>
``return _``

### Class ``Curve`` <a id='app--k5'></a>
>

#### Getters
> - [linear](#app-6-g-1)
> - [inQuad](#app-6-g0)
> - [outQuad](#app-6-g1)
> - [inOutQuad](#app-6-g2)
> - [inQuart](#app-6-g3)
> - [outQuart](#app-6-g4)
> - [inOutQuart](#app-6-g5)
> - [inBack](#app-6-g6)
> - [outBack](#app-6-g7)
> - [inOutBack](#app-6-g8)
> - [inElastic](#app-6-g9)
> - [outElastic](#app-6-g10)
> - [inOutElastic](#app-6-g11)
##### Static Getter ``linear`` <a id='app-6-g-1'></a>
``return _``

##### Static Getter ``inQuad`` <a id='app-6-g0'></a>
``return _``

##### Static Getter ``outQuad`` <a id='app-6-g1'></a>
``return _``

##### Static Getter ``inOutQuad`` <a id='app-6-g2'></a>
``return _``

##### Static Getter ``inQuart`` <a id='app-6-g3'></a>
``return _``

##### Static Getter ``outQuart`` <a id='app-6-g4'></a>
``return _``

##### Static Getter ``inOutQuart`` <a id='app-6-g5'></a>
``return _``

##### Static Getter ``inBack`` <a id='app-6-g6'></a>
``return _``

##### Static Getter ``outBack`` <a id='app-6-g7'></a>
``return _``

##### Static Getter ``inOutBack`` <a id='app-6-g8'></a>
``return _``

##### Static Getter ``inElastic`` <a id='app-6-g9'></a>
``return _``

##### Static Getter ``outElastic`` <a id='app-6-g10'></a>
``return _``

##### Static Getter ``inOutElastic`` <a id='app-6-g11'></a>
``return _``

### Class ``Tween`` <a id='app--k6'></a>
>

#### Constructors
> - [new](#app-7-c-1)
#### Getters
> - [tweens](#app-7-g-1)
> - [tweenCount](#app-7-g0)
> - [duration](#app-7-g1)
> - [use_curve](#app-7-g2)
> - [from](#app-7-g3)
> - [to](#app-7-g4)
> - [on_complete](#app-7-g5)
> - [toString](#app-7-g6)
#### Setters
> - [tweens](#app-7-s-1)
> - [duration](#app-7-s0)
> - [use_curve](#app-7-s1)
> - [from](#app-7-s2)
> - [to](#app-7-s3)
> - [on_complete](#app-7-s4)
#### Methods
> - [insert_tween](#app-7-m-1)
> - [time](#app-7-m0)
> - [curve](#app-7-m1)
> - [onComplete](#app-7-m2)
> - [play](#app-7-m3)
##### Static Getter ``tweens`` <a id='app-7-g-1'></a>
``return _``

##### Static Setter ``tweens = v: _`` <a id='app-7-s-1'></a>

##### Static Getter ``tweenCount`` <a id='app-7-g0'></a>
``return _``

##### Static Method ``insert_tween(t: _)`` <a id='app-7-m-1'></a>
``return _``

##### Getter ``duration`` <a id='app-7-g1'></a>
``return _``

##### Getter ``use_curve`` <a id='app-7-g2'></a>
``return _``

##### Getter ``from`` <a id='app-7-g3'></a>
``return _``

##### Getter ``to`` <a id='app-7-g4'></a>
``return _``

##### Getter ``on_complete`` <a id='app-7-g5'></a>
``return _``

##### Setter ``duration = v: _`` <a id='app-7-s0'></a>

##### Setter ``use_curve = v: _`` <a id='app-7-s1'></a>

##### Setter ``from = v: _`` <a id='app-7-s2'></a>

##### Setter ``to = v: _`` <a id='app-7-s3'></a>

##### Setter ``on_complete = v: _`` <a id='app-7-s4'></a>

##### Constructor ``new(f: _, t: _)`` <a id='app-7-c-1'></a>
``return _``

##### Getter ``toString`` <a id='app-7-g6'></a>
``return _``

##### Method ``time(t: _)`` <a id='app-7-m0'></a>
``return _``

##### Method ``curve(c: _)`` <a id='app-7-m1'></a>
``return _``

##### Method ``onComplete(c: _)`` <a id='app-7-m2'></a>
``return _``

##### Method ``play(c: _)`` <a id='app-7-m3'></a>
``return _``

## Module ``math``
### Classes
> - [Util](#math--k-1)
> - [Vec2](#math--k0)
### Class ``Util`` <a id='math--k-1'></a>
>

#### Methods
> - [lerp](#math-0-m-1)
##### Static Method ``lerp(a: _, b: _, t: _)`` <a id='math-0-m-1'></a>
``return _``

### Foreign Class ``Vec2`` <a id='math--k0'></a> 
> Inherits from ``Serializable``
>

#### Constructors
> - [new](#math-1-c-1)
#### Getters
> - [](#math-1-g-1)
> - [](#math-1-g0)
> - [default](#math-1-g1)
> - [](#math-1-g2)
> - [](#math-1-g3)
> - [](#math-1-g4)
> - [](#math-1-g5)
> - [](#math-1-g6)
> - [](#math-1-g7)
> - [-](#math-1-g8)
> - [toString](#math-1-g9)
#### Setters
> - [x](#math-1-s-1)
> - [y](#math-1-s0)
#### Methods
> - [getProperty](#math-1-m-1)
> - [setProperty](#math-1-m0)
> - [magnitude](#math-1-m1)
> - [magnitude_sqr](#math-1-m2)
> - [normalized](#math-1-m3)
> - [normalize](#math-1-m4)
> - [cross](#math-1-m5)
> - [dot](#math-1-m6)
> - [lerp](#math-1-m7)
> - [screen_to_world_space](#math-1-m8)
> - [world_to_screen_space](#math-1-m9)
> - [+](#math-1-m10)
> - [-](#math-1-m11)
> - [*](#math-1-m12)
> - [/](#math-1-m13)
> - [=](#math-1-m14)
##### Constructor ``new(x: _, y: _)`` <a id='math-1-c-1'></a>
``return _``

##### Foreign Getter ```` <a id='math-1-g-1'></a>
``return _``

##### Foreign Getter ```` <a id='math-1-g0'></a>
``return _``

##### Foreign Setter ``x = x: _`` <a id='math-1-s-1'></a>

##### Foreign Setter ``y = y: _`` <a id='math-1-s0'></a>

##### Static Getter ``default`` <a id='math-1-g1'></a>
``return _``

##### Method ``getProperty()`` <a id='math-1-m-1'></a>
``return _``

##### Method ``setProperty()`` <a id='math-1-m0'></a>
``return _``

##### Foreign Static Getter ```` <a id='math-1-g2'></a>
``return _``

##### Foreign Static Getter ```` <a id='math-1-g3'></a>
``return _``

##### Foreign Static Getter ```` <a id='math-1-g4'></a>
``return _``

##### Foreign Static Getter ```` <a id='math-1-g5'></a>
``return _``

##### Foreign Static Getter ```` <a id='math-1-g6'></a>
``return _``

##### Foreign Static Getter ```` <a id='math-1-g7'></a>
``return _``

##### Foreign Method ``magnitude()`` <a id='math-1-m1'></a>
``return _``

##### Foreign Method ``magnitude_sqr()`` <a id='math-1-m2'></a>
``return _``

##### Foreign Method ``normalized()`` <a id='math-1-m3'></a>
``return _``

##### Foreign Method ``normalize()`` <a id='math-1-m4'></a>
``return _``

##### Foreign Static Method ``cross(a: _, b: _)`` <a id='math-1-m5'></a>
``return _``

##### Foreign Static Method ``dot(a: _, b: _)`` <a id='math-1-m6'></a>
``return _``

##### Foreign Static Method ``lerp(a: _, b: _, t: _)`` <a id='math-1-m7'></a>
``return _``

##### Foreign Static Method ``screen_to_world_space(pos: _)`` <a id='math-1-m8'></a>
``return _``

##### Foreign Static Method ``world_to_screen_space(pos: _)`` <a id='math-1-m9'></a>
``return _``

##### Method ``+(other: _)`` <a id='math-1-m10'></a>
``return _``

##### Method ``-(other: _)`` <a id='math-1-m11'></a>
``return _``

##### Method ``*(other: _)`` <a id='math-1-m12'></a>
``return _``

##### Method ``/(other: _)`` <a id='math-1-m13'></a>
``return _``

##### Getter ``-`` <a id='math-1-g8'></a>
``return _``

##### Method ``==( = other: _)`` <a id='math-1-m14'></a>

##### Getter ``toString`` <a id='math-1-g9'></a>
``return _``

