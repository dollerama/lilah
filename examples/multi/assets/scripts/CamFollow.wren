import "game" for GameObject, Transform, Behaviour, ComponentBehaviour
import "app" for Lilah, GameObjectRef
import "math" for Vec2

class CamFollow is Behaviour {
    static gameobject { __gameobject }
    static gameobject=(v) { __gameobject = GameObjectRef.new(v) }
    static self { gameobject.behaviourData(CamFollow, __uuid) }
    static self=(v) {__uuid = v}

    speed { _speed }
    speed=(v) { _speed = v }

    construct new() {
        speed = 1
    }

    construct new(g) {
        super(g, CamFollow)
    }

    static start() { 
    }

    static update() {
        Transform.set_position(
            Lilah.camera.ref, 
            Vec2.lerp(
                Lilah.camera.ref.get("Transform").position, 
                gameobject.ref.get("Transform").position, Lilah.delta_time * self.speed
            )
        )
    }
}