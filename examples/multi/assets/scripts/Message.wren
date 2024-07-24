import "game" for GameObject, Transform, Behaviour, ComponentBehaviour
import "app" for Lilah, GameObjectRef
import "math" for Vec2

class Message is Behaviour {
    #!msg(ord = 0)
    msg { _msg }
    msg=(v) { _msg = v }

    construct new() {
        msg = "default"
    }

    static default { this.new() }

    getProperty() {
        return properties([this.msg])
    }

    setProperty() {
        return properties { |v|
            this.msg = v.call()
        }
    }

    static start() { 
        System.print(gamebehaviour.msg)
        System.print(gamebehaviour.serialize())
    }
}