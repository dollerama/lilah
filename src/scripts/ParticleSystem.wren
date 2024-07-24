import "math" for Vec2, Util
import "app" for Lilah, GameObjectRef, Curve
import "io" for Serializable
import "game" for Behaviour, GameObject, Transform, Sprite
import "random" for Random

class ParticleField {
    value { 
        if(_value is Fn) {
            return _value.call()
        } else {
            return _value
        } 
    }
    value=(v) { _value = v}
    [t]=(v) {_value = v}
    [t] {
        if(_value is List) {
            if(_value[0] is Vec2) {
                return Vec2.lerp(_value[0], _value[1], curve.call(t))
            } else if(_value[0] is List) {
                var result = []
                for(i in (0.._value[0].count-1)) {
                    if(_value[0] is Vec2) {
                        result.add(Vec2.lerp(_value[0][i], _value[1][i], curve.call(t)))
                    } else {
                        result.add(Util.lerp(_value[0][i], _value[1][i], curve.call(t)))
                    }
                }
                return result
            } else {
                return Util.lerp(_value[0], _value[1], curve.call(t))
            }
        } else {
            if(_value is Fn) {
                return _value.call()
            } else {
                return _value
            }
        }
    }
    curve{_curve}
    curve=(v) {_curve = v}

    construct new(v, c) {
        _curve = c
        _value = v
    }

    construct new(v) {
        _curve = Curve.linear
        _value = v
    }
}

class ParticleSystem is Behaviour {
    static gameobject { __gameobject }
    static gameobject=(v) { __gameobject = GameObjectRef.new(v) }
    static gamebehaviour { gameobject.behaviourData(ParticleSystem, __uuid) }
    static gamebehaviour=(v) {__uuid = v}
    
    construct new(g) { super(g, ParticleSystem) }

    static default { ParticleSystem.new() }

    #!lifeSpan(ord = 0)
    lifeSpan { _lifeSpan }
    lifeSpan=(v) { _lifeSpan = v }

    #!rate(ord = 0)
    rate { _rate }
    rate=(v) { _rate = v }

    #!speed(ord = 0)
    speed { _speed }
    speed=(v) { _speed = v }

    #!rotation(ord = 0)
    rotation { _rotation }
    rotation=(v) { _rotation = v }

    #!direction(ord = 0)
    direction { _direction }
    direction=(v) { _direction = v }

    #!scale(ord = 0)
    scale { _scale }
    scale=(v) { _scale = v }

    #!color(ord = 0)
    color { _color }
    color=(v) { _color = v }

    internal_time {_internal_time}
    internal_time=(v) {_internal_time = v}
    parts {_parts}
    parts=(v) {_parts = v}

    construct new() {
        var random = Random.new()

        lifeSpan = ParticleField.new(0.5)
        speed = ParticleField.new([500, 0], Curve.inOutQuart)
        direction = ParticleField.new(Fn.new { Vec2.new(random.float(-1.0, 1.0), random.float(-1.0, 1.0)) })
        color = ParticleField.new([[1,1,1,1], [1,1,1,0]])
        scale = ParticleField.new([Vec2.new(1,1), Vec2.new(0,0)])
        rate = ParticleField.new(100)
        rotation = ParticleField.new(10)

        internal_time = 0
        parts = []
    }

    static emit() {
        var p = GameObject.new("p_sys")
        p.add(Transform.new(gameobject.ref.get("Transform").position))
        p.add(Sprite.new("assets/test.png"))

        var pp = Lilah.instantiate(p, {"life": gamebehaviour.lifeSpan.value, "direction": gamebehaviour.direction.value})
        Sprite.cut_sprite_sheet(pp.ref, Vec2.new(0, 0), Vec2.new(3, 3))
        Sprite.set_tint(pp.ref, gamebehaviour.color[1])
        gamebehaviour.parts.add(pp)
    }

    static update() {
        gamebehaviour.parts = gamebehaviour.parts.where {|v| v.ref != null }.toList
        gamebehaviour.internal_time = gamebehaviour.internal_time + Lilah.delta_time

        if(gamebehaviour.internal_time > gamebehaviour.rate.value/1000) {
            gamebehaviour.internal_time = 0
            ParticleSystem.emit()
        }
        
        for(i in gamebehaviour.parts) {
            var t = 1-(i["life"]/gamebehaviour.lifeSpan.value)

            i["life"] = i["life"] - Lilah.delta_time
            Transform.update_position(i.ref, i["direction"] * gamebehaviour.speed[t] * Lilah.delta_time)
            Transform.set_scale(i.ref, gamebehaviour.scale[t])
            Transform.update_rotation(i.ref, gamebehaviour.rotation[t] * Lilah.delta_time)
            Sprite.set_tint(i.ref, gamebehaviour.color[t])

            if(i["life"] < 0) {
                Lilah.destroy(i)
            }
        }
    }
}

var particlesystem = ParticleSystem.new()