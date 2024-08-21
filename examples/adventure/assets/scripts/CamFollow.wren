import "game" for GameObject, Transform, Behaviour, ComponentBehaviour
import "app" for Lilah, GameObjectRef
import "math" for Vec2

class CamFollow is Behaviour {
    speed { _speed }
    speed=(v) { _speed = v }

    construct new() {
        speed = 1
    }

    static start() { 
        Transform.set_scale(
            Lilah.camera.ref, 
            Vec2.one*2
        )
    }

    static update() {
        Transform.set_position(
            Lilah.camera.ref, 
            Vec2.lerp(
                Lilah.camera.ref.get("Transform").position, 
                gameobject.ref.get("Transform").position, Lilah.delta_time * gamebehaviour.speed
            )
        )

        // Transform.set_position_x(
        //     Lilah.camera.ref, 
        //     Lilah.camera.ref.get(Transform).position.x.round()
        // )
        // Transform.set_position_y(
        //     Lilah.camera.ref, 
        //     Lilah.camera.ref.get(Transform).position.y.round()
        // )
    }
}