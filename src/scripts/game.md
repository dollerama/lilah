# Game
### Classes
> - [Behaviour](##behaviour)
> - [Component](##component)
> - [Text](##text)
> - [Sprite](##sprite)
> - [Scene](##scene)
> - [Rigidbody](##rigidbody)
> - [Animator](##animator)
> - [Transform](##transform)
> - [GameObject](##gameobject)
> - [Sfx](##sfx)
> - [ComponentBehaviour](##componentbehaviour)
## Behaviour
Inherits from ``Serializable``
> - data
> - data = v: Map
> - [i: Any]
> - [i: Any] = v: Any
> - frame
> - frame = v: Num
> - as_behaviour
> - new(g: Gameobject, c: Behaviour)
> - start()
> - update()
> - onCollision()
> - setup()
> - start()
> - update()
### ``data``
static getter returns ``Map``
### ``data = v: Map``
static setter
### ``[i: Any]``
static getter with arity(1) and returns ``Any``
### ``[i: Any] = v: Any``
static setter
### ``frame``
getter returns ``Num``
### ``frame = v: Num``
setter
### ``as_behaviour``
getter returns ``ComponentBehaviour``
> Example:
> ```js
> gameobject.add(ParticleSystem.new(gameobject).as_behaviour)
> ```
### ``new(g: Gameobject, c: Behaviour)``
constructor with arity(2) and returns ``Behaviour``
### ``start()``
static method returns ``null``
> Runs the frame after setup.
### ``update()``
static method returns ``null``
> Run every frame.
### ``onCollision()``
static method returns ``null``
> Runs every frame after start that the Behaviour has a collision given a Rigidbody and Transform is attached.
### ``setup()``
method returns ``null``
> Runs the first frame regardless of whether or not the Behaviour is attached.
### ``start()``
method returns ``null``
> Runs the second frame regardless of whether or not the Behaviour is attached.
### ``update()``
method returns ``null``
> Runs every frame after start regardless of whether or not the Behaviour is attached.
## Component
> Rust dyn obj that all components derive from
## Text
> - new(text: String, font: String)
> - as_component
> - text
> - font
> - font_size
> - text = v: String
> - font = v: String
> - font_size = v: Num
> - get_text(go: GameObject)
> - get_font(go: GameObject)
> - get_font_size(go: GameObject)
> - set_text(go: GameObject, text: String)
> - set_font(go: GameObject, font: String)
> - set_font_size(go: GameObject, fs: Num)
### ``new(text: String, font: String)``
constructor with arity(2) and returns ``Text``
### ``as_component``
getter returns ``Component``
### ``text``
getter returns ``String``
### ``font``
getter returns ``String``
### ``font_size``
getter returns ``Num``
### ``text = v: String``
setter
### ``font = v: String``
setter
### ``font_size = v: Num``
setter
### ``get_text(go: GameObject)``
static method with arity(1) and returns ``String``
### ``get_font(go: GameObject)``
static method with arity(1) and returns ``String``
### ``get_font_size(go: GameObject)``
static method with arity(1) and returns ``Num``
### ``set_text(go: GameObject, text: String)``
static method with arity(2) and returns ``null``
### ``set_font(go: GameObject, font: String)``
static method with arity(2) and returns ``null``
### ``set_font_size(go: GameObject, fs: Num)``
static method with arity(2) and returns ``null``
## Sprite
> - new(id: String)
> - as_component
> - size
> - texture_id
> - current_index
> - tint
> - cut_sprite_sheet(i: Vec2, j: Vec2)
> - cut_sprite_sheet(go: Gameobject, i: Vec2, j: Vec2)
> - set_sort(go: Gameobject, i: Num)
> - set_tint(go: Gameobject, color: List)
### ``new(id: String)``
constructor with arity(1) and returns ``Sprite``
### ``as_component``
getter returns ``Component``
### ``size``
getter returns ``Vec2``
### ``texture_id``
getter returns ``String``
### ``current_index``
getter returns ``Vec2``
### ``tint``
getter returns ``List``
> Returns in the form [r,g,b,a]
### ``cut_sprite_sheet(i: Vec2, j: Vec2)``
method with arity(2) and returns ``null``
### ``cut_sprite_sheet(go: Gameobject, i: Vec2, j: Vec2)``
method with arity(3) and returns ``null``
### ``set_sort(go: Gameobject, i: Num)``
method with arity(2) and returns ``null``
### ``set_tint(go: Gameobject, color: List)``
method with arity(2) and returns ``null``
## Scene
> - new(i: String)
> - as_component
### ``new(i: String)``
constructor with arity(1) and returns ``Scene``
### ``as_component``
getter returns ``Component``
## Rigidbody
> - new()
> - as_component
> - position
> - velocity
> - velocity = value: Vec2
> - solid
> - solid = value: bool
> - colliding
> - colliding(go: Gameobject)
> - set_solid(go: Gameobject, solid: bool)
> - set_position(go: Gameobject, new_pos: Vec2)
> - set_position_x(go: Gameobject, new_x: Num)
> - set_position_y(go: Gameobject, new_y: Num)
> - set_velocity(go: Gameobject, vel: Vec2)
> - set_velocity_x(go: Gameobject, new_x: Num)
> - set_velocity_y(go: Gameobject, new_y: Num)
> - update_velocity(go: Gameobject, vel: Vec2)
> - update_velocity_x(go: Gameobject, new_x: Num)
> - set_velocity(go: Gameobject, new_y: Num)
> - set_rotation(go: Gameobject, new_rot: Num)
### ``new()``
constructor returns ``Rigidbody``
### ``as_component``
getter returns ``Component``
### ``position``
getter returns ``Vec2``
### ``velocity``
getter returns ``Vec2``
### ``velocity = value: Vec2``
setter
### ``solid``
getter returns ``bool``
### ``solid = value: bool``
setter
### ``colliding``
getter returns ``Map/null``
> returns a map in the form "name": _, "uuid": _ or null if no collision
### ``colliding(go: Gameobject)``
static method with arity(1) and returns ``Map/null``
> returns a map in the form "name": _, "uuid": _ or null if no collision
### ``set_solid(go: Gameobject, solid: bool)``
static method with arity(2) and returns ``null``
### ``set_position(go: Gameobject, new_pos: Vec2)``
static method with arity(2) and returns ``null``
### ``set_position_x(go: Gameobject, new_x: Num)``
static method with arity(2) and returns ``null``
### ``set_position_y(go: Gameobject, new_y: Num)``
static method with arity(2) and returns ``null``
### ``set_velocity(go: Gameobject, vel: Vec2)``
static method with arity(2) and returns ``null``
### ``set_velocity_x(go: Gameobject, new_x: Num)``
static method with arity(2) and returns ``null``
### ``set_velocity_y(go: Gameobject, new_y: Num)``
static method with arity(2) and returns ``null``
### ``update_velocity(go: Gameobject, vel: Vec2)``
static method with arity(2) and returns ``null``
### ``update_velocity_x(go: Gameobject, new_x: Num)``
static method with arity(2) and returns ``null``
### ``set_velocity(go: Gameobject, new_y: Num)``
static method with arity(2) and returns ``null``
### ``set_rotation(go: Gameobject, new_rot: Num)``
static method with arity(2) and returns ``null``
## Animator
> - new()
> - as_component
> - playing
> - speed
> - speed = value: Num
> - frame
> - frame = value: Num
> - play()
> - stop()
> - get_state(s: String)
> - set_state(s: String)
> - insert_state(s: String, i: Vec2)
> - play(go: Gameobject)
> - stop(go: Gameobject)
> - set_state(go: Gameobject, s: String)
> - get_state(go: Gameobject, s: String)
> - insert_state(go: Gameobject, s: String, i: Vec2)
> - set_speed(go: Gameobject, s: Num)
> - set_frame(go: Gameobject, f: Num)
### ``new()``
constructor returns ``Animator``
### ``as_component``
getter returns ``Component``
### ``playing``
getter returns ``bool``
### ``speed``
getter returns ``Num``
### ``speed = value: Num``
setter
### ``frame``
getter returns ``Num``
### ``frame = value: Num``
setter
### ``play()``
method returns ``null``
### ``stop()``
method returns ``null``
### ``get_state(s: String)``
method with arity(1) and returns ``Map``
> returns map in the form state: value:Vec2
### ``set_state(s: String)``
method with arity(1) and returns ``null``
### ``insert_state(s: String, i: Vec2)``
method with arity(2) and returns ``null``
### ``play(go: Gameobject)``
static method with arity(1) and returns ``null``
### ``stop(go: Gameobject)``
static method with arity(1) and returns ``null``
### ``set_state(go: Gameobject, s: String)``
static method with arity(2) and returns ``null``
### ``get_state(go: Gameobject, s: String)``
static method with arity(2) and returns ``String``
### ``insert_state(go: Gameobject, s: String, i: Vec2)``
static method with arity(3) and returns ``null``
### ``set_speed(go: Gameobject, s: Num)``
static method with arity(2) and returns ``null``
### ``set_frame(go: Gameobject, f: Num)``
static method with arity(2) and returns ``null``
## Transform
Inherits from ``Serializable``
> - new(p: Vec2)
> - as_component
> - position
> - scale
> - rotation
> - pivot
> - position = value: Vec2
> - scale = value: Vec2
> - rotation = value: Num
> - pivot = value: Vec2
> - default
> - getProperty()
> - setProperty()
> - set_pivot(go: Gameobject, new_pivot: Vec2)
> - set_position(go: Gameobject, new_pos: Vec2)
> - set_position_x(go: Gameobject, new_x: Num)
> - set_position_y(go: Gameobject, new_y: Num)
> - update_position(go: Gameobject, new_pos: Vec2)
> - update_position_x(go: Gameobject, new_x: Num)
> - update_position_y(go: Gameobject, new_y: Num)
> - set_scale(go: Gameobject, new_scale: Vec2)
> - set_scale_x(go: Gameobject, new_x: Num)
> - set_scale_y(go: Gameobject, new_y: Num)
> - update_scale(go: Gameobject, new_scale: Vec2)
> - update_scale_x(go: Gameobject, new_x: Num)
> - update_scale_y(go: Gameobject, new_y: Num)
> - set_rotation(go: Gameobject, new_rot: Num)
> - update_rotation(go: Gameobject, new_rot: Num)
### ``new(p: Vec2)``
constructor with arity(1) and returns ``Transform``
### ``as_component``
getter returns ``Component``
### ``position``
getter returns ``Vec2``
### ``scale``
getter returns ``Vec2``
### ``rotation``
getter returns ``Num``
### ``pivot``
getter returns ``Vec2``
### ``position = value: Vec2``
setter
### ``scale = value: Vec2``
setter
### ``rotation = value: Num``
setter
### ``pivot = value: Vec2``
setter
### ``default``
static getter returns ``Transform``
### ``getProperty()``
method returns ``List/Fn``
### ``setProperty()``
method returns ``null``
### ``set_pivot(go: Gameobject, new_pivot: Vec2)``
static method with arity(2) and returns ``null``
### ``set_position(go: Gameobject, new_pos: Vec2)``
static method with arity(2) and returns ``null``
### ``set_position_x(go: Gameobject, new_x: Num)``
static method with arity(2) and returns ``null``
### ``set_position_y(go: Gameobject, new_y: Num)``
static method with arity(2) and returns ``null``
### ``update_position(go: Gameobject, new_pos: Vec2)``
static method with arity(2) and returns ``null``
### ``update_position_x(go: Gameobject, new_x: Num)``
static method with arity(2) and returns ``null``
### ``update_position_y(go: Gameobject, new_y: Num)``
static method with arity(2) and returns ``null``
### ``set_scale(go: Gameobject, new_scale: Vec2)``
static method with arity(2) and returns ``null``
### ``set_scale_x(go: Gameobject, new_x: Num)``
static method with arity(2) and returns ``null``
### ``set_scale_y(go: Gameobject, new_y: Num)``
static method with arity(2) and returns ``null``
### ``update_scale(go: Gameobject, new_scale: Vec2)``
static method with arity(2) and returns ``null``
### ``update_scale_x(go: Gameobject, new_x: Num)``
static method with arity(2) and returns ``null``
### ``update_scale_y(go: Gameobject, new_y: Num)``
static method with arity(2) and returns ``null``
### ``set_rotation(go: Gameobject, new_rot: Num)``
static method with arity(2) and returns ``null``
### ``update_rotation(go: Gameobject, new_rot: Num)``
static method with arity(2) and returns ``null``
## GameObject
> - new(name: String)
> - add(x: Component)
> - get(x: String)
> - set(x: String, y: Component)
> - id
> - uuid
> - name
> - name = v: String
### ``new(name: String)``
constructor with arity(1) and returns ``Gameobject``
### ``add(x: Component)``
method with arity(1) and returns ``null``
### ``get(x: String)``
method with arity(1) and returns ``Component``
### ``set(x: String, y: Component)``
method with arity(2) and returns ``null``
### ``id``
getter returns ``Map``
> Returns a map in the form "name": _, "uuid": _
### ``uuid``
getter returns ``String``
### ``name``
getter returns ``String``
### ``name = v: String``
setter
## Sfx
> - new(name: String, file: String)
> - as_component
> - name
> - name = v: String
> - volume
> - volume = v: Num
> - file
> - play()
> - get_volume(go: Gameobject, name: String)
> - get_volume(go: Gameobject, name: String, amt: Num)
> - play(go: Gameobject, name: String)
### ``new(name: String, file: String)``
constructor with arity(2) and returns ``Sfx``
### ``as_component``
getter returns ``Component``
### ``name``
getter returns ``String``
### ``name = v: String``
setter
### ``volume``
getter returns ``Num``
### ``volume = v: Num``
setter
### ``file``
getter returns ``String``
### ``play()``
method returns ``null``
### ``get_volume(go: Gameobject, name: String)``
static method with arity(2) and returns ``Num``
### ``get_volume(go: Gameobject, name: String, amt: Num)``
static method with arity(3) and returns ``null``
### ``play(go: Gameobject, name: String)``
static method with arity(2) and returns ``null``
## ComponentBehaviour
> - new(b: String)
> - as_component
> - uuid
### ``new(b: String)``
constructor with arity(1) and returns ``ComponentBehaviour``
### ``as_component``
getter returns ``Component``
### ``uuid``
getter returns ``String``
