import "game" for GameObject, Transform, Behaviour, ComponentBehaviour
import "app" for Lilah, GameObjectRef
import "math" for Vec2

class Message is Behaviour {
    msg { _msg }
    msg=(v) { _msg = v }

    construct new() {
        msg = "default"
    }

    static start() { 
        System.print(gamebehaviour.msg)
    }
}