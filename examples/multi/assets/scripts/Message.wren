import "game" for GameObject, Transform, Behaviour, ComponentBehaviour
import "app" for Lilah, GameObjectRef
import "math" for Vec2

class Message is Behaviour {
    static gameobject { __gameobject }
    static gameobject=(v) { __gameobject = GameObjectRef.new(v) }
    static self=(v) {__uuid = v}
    static self { gameobject.behaviourData(Message, __uuid) }
    

    msg { _msg }
    msg=(v) { _msg = v }

    construct new() {
        msg = "default"
    }

    construct new(g) {
        super(g, Message)
    }

    static start() { 
        System.print(self.msg)
    }
}