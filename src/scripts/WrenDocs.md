# Game
### Classes
> - [Behaviour](##Behaviour)
> - [Component](##Component)
> - [Text](##Text)
## Behaviour
> - [data](###``data``)
> - [data = v: Map](###``data = v: Map``)
> - [[i: Any]](###``[i: Any]``)
> - [[i: Any] = v: Any](###``[i: Any] = v: Any``)
> - [frame](###``frame``)
> - [frame = v: Num](###``frame = v: Num``)
> - [as_behaviour](###``as_behaviour``)
> - [new(g: Gameobject, c: Behaviour)](###``new(g: Gameobject, c: Behaviour)``)
> - [start()](###``start()``)
> - [update()](###``update()``)
> - [onCollision()](###``onCollision()``)
> - [setup()](###``setup()``)
> - [start()](###``start()``)
> - [update()](###``update()``)> Inherits from Serializable
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
static method returns ``Null``
> Runs the frame after setup.
### ``update()``
static method returns ``Null``
> Run every frame.
### ``onCollision()``
static method returns ``Null``
> Runs every frame after start that the Behaviour has a collision given a Rigidbody and Transform is attached.
### ``setup()``
method returns ``Null``
> Runs the first frame regardless of whether or not the Behaviour is attached.
### ``start()``
method returns ``Null``
> Runs the second frame regardless of whether or not the Behaviour is attached.
### ``update()``
method returns ``Null``
> Runs every frame after start regardless of whether or not the Behaviour is attached.
## Component
> Rust dyn obj that all components derive from
## Text
> - [new(text: String, font: String)](###``new(text: String, font: String)``)
> - [as_component](###``as_component``)
> - [text](###``text``)
> - [font](###``font``)
> - [font_size](###``font_size``)
> - [](###````)
> - [](###````)
> - [](###````)
> - [get_text(go: GameObject)](###``get_text(go: GameObject)``)
> - [get_font(go: GameObject)](###``get_font(go: GameObject)``)
> - [get_font_size(go: GameObject)](###``get_font_size(go: GameObject)``)
> - [set_text(go: GameObject, text: String)](###``set_text(go: GameObject, text: String)``)
> - [set_font(go: GameObject, font: String)](###``set_font(go: GameObject, font: String)``)
> - [set_font_size(go: GameObject, fs: Num)](###``set_font_size(go: GameObject, fs: Num)``)### ``new(text: String, font: String)``
constructor with arity(2) and returns ``Text``
### ``as_component``
foreign getter returns ``Component``
### ``text``
foreign getter returns ``String``
### ``font``
foreign getter returns ``String``
### ``font_size``
foreign getter returns ``Num``
### ````
foreign setter returns ````
### ````
foreign setter returns ````
### ````
foreign setter returns ````
### ``get_text(go: GameObject)``
foreign static method with arity(1) and returns ``String``
### ``get_font(go: GameObject)``
foreign static method with arity(1) and returns ``String``
### ``get_font_size(go: GameObject)``
foreign static method with arity(1) and returns ``Num``
### ``set_text(go: GameObject, text: String)``
foreign static method with arity(2) and returns ``Null``
### ``set_font(go: GameObject, font: String)``
foreign static method with arity(2) and returns ``Null``
### ``set_font_size(go: GameObject, fs: Num)``
foreign static method with arity(2) and returns ``Null``
