import "math" for Vec2
import "app" for Lilah, Input, GameObjectRef, Audio, Tween
import "game" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour, Text, Sfx
import "io" for Fs, Json, Serializable
import "CamFollow" for CamFollow
import "Message" for Message

class Player is Behaviour {
    static gameobject { __gameobject }
    static gameobject=(v) { __gameobject = GameObjectRef.new(v) }
    static self { gameobject.behaviourData(Player, __uuid) }
    static self=(v) {__uuid = v}

    construct new() {
    }

    construct new(g) {
        super(g, Player)
    }

    setup() {
        Input.update_binding("Horizontal", "A", "D")
        Input.update_binding("Vertical", "S", "W")

        var gameobject = GameObject.new("A")

        gameobject.add(Transform.new(Vec2.new(100,100)))
        gameobject.add(Sprite.new("assets/test.png"))
        gameobject.add(Rigidbody.new())
        gameobject.add(Player.new(gameobject).as_behaviour)
        gameobject.add(CamFollow.new(gameobject).as_behaviour)
        gameobject.add(Message.new(gameobject).as_behaviour)
        gameobject.add(Message.new(gameobject).as_behaviour)
        var gf = Lilah.instantiate(gameobject)
        gf.behaviourData(CamFollow).speed = 4
        gf.behaviourData(Message)[0].msg = "A"
        gf.behaviourData(Message)[1].msg = "B"
        // gf.behaviourData(Message) { |index, item| 
        //     if(index == 0) {
        //         item.msg = "A"
        //     } else {
        //         item.msg = "B"
        //     }
        // }   

        var gameobject2 = GameObject.new("B")

        gameobject2.add(Transform.new(Vec2.new(400,100)))
        gameobject2.add(Sprite.new("assets/test.png"))
        gameobject2.add(Rigidbody.new())
        //gameobject.add(this.as_behaviour)

        Lilah.instantiate(gameobject2) 
    }

    static start() {
        Sprite.cut_sprite_sheet(gameobject.ref, Vec2.new(0, 0), Vec2.new(3, 3))
        Sprite.set_sort(gameobject.ref, 2)
    }

    static update() {
        Rigidbody.set_velocity(gameobject.ref, Input.binding2D("Horizontal", "Vertical")*100)
    }
}
