import "math" for Vec2
import "app" for State, Input, GameObjectRef, Audio
import "engine" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour, Text, Sfx

class Paddle is Behaviour {
    construct new() {
        super(Paddle)
    }

    static start(id) {
        var gameobject = GameObjectRef.new(id)
    }
    
    static update(id) {
        var gameobject = GameObjectRef.new(id)
        Rigidbody.set_velocity_y(gameobject.ref, Input.binding(gameobject["controls"])*5)
    }
}