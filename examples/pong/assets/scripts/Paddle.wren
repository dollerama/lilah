import "math" for Vec2
import "app" for Lilah, Input, GameObjectRef, Audio
import "game" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour, Text, Sfx

class Paddle is Behaviour {
    static gameobject { __gameobject }
    static gameobject=(v) { __gameobject = GameObjectRef.new(v) }

    construct new() {
        super(Paddle)
    }

    static start() { }
    
    static update() {
        Rigidbody.set_velocity_y(gameobject.ref, Input.binding(gameobject["controls"])*200)
    }
}