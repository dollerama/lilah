import "math" for Vec2, Util
import "app" for Lilah, GameObjectRef, Curve
import "io" for Serializable
import "game" for Behaviour, GameObject, Transform, Sprite, Debug, Line
import "random" for Random

class TrailField {
    raw { _value }
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

class Trail is Behaviour {
    static gameobject { __gameobject }
    static gameobject=(v) { __gameobject = GameObjectRef.new(v) }
    static gamebehaviour { gameobject.behaviourData(Trail, __uuid) }
    static gamebehaviour=(v) {__uuid = v}
    
    construct new(g) { super(g, Trail) }

    static default { Trail.new() }

    minDistance {_minDistance }
    minDistance=(v) { _minDistance = v }
    maxCount {_maxCount}
    maxCount=(v) {_maxCount = v}
    lastPos { _lastPos }
    lastPos=(v) { _lastPos = v }

    construct new() {
    }

    static start() {
        gameobject.ref.add(Line.new())
        Line.set_thickness(gameobject.ref, [0.0, 10.0])
        
        gamebehaviour.minDistance = 50 
        gamebehaviour.maxCount = 5 
        gamebehaviour.lastPos = gameobject.ref.get(Transform).position
    }

    static update() {
        if(gameobject.ref.get(Line).points.count < gamebehaviour.maxCount) {
            Line.add_point(gameobject.ref, gameobject.ref.get(Transform).position) 
        }
        
        Line.set_point(gameobject.ref, gameobject.ref.get(Line).points.count-1, gameobject.ref.get(Transform).position)

        var loop_len = gameobject.ref.get(Line).points.count-2 
        
        for(i in loop_len..0) {
              if((gameobject.ref.get(Line).points[i]-gameobject.ref.get(Line).points[i+1]).magnitude() > gamebehaviour.minDistance) {
                  Line.set_point(gameobject.ref, i, gameobject.ref.get(Line).points[i+1] + (gameobject.ref.get(Line).points[i]-gameobject.ref.get(Line).points[i+1]).normalized()*10)    
              }
        }

        lastPos = gameobject.ref.get(Transform).position
    }
}

var trail = Trail.new()
