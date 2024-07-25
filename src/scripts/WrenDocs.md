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
1
### ``new(text: String, font: String)``
constructor with arity(2) and returns ``Text``
