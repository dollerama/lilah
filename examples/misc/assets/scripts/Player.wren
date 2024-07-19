import "math" for Vec2
import "app" for Lilah, Input, GameObjectRef, Audio, Tween
import "game" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour, Text, Sfx
import "io" for Fs, Json, Serializable

class Player is Behaviour {

    static gameobject { __gameobject }
    static gameobject=(v) { __gameobject = GameObjectRef.new(v) }

    construct new() {
        super(Player)
    }

    setup() {
        var gameobject = GameObject.new("C")

        Input.update_binding("Horizontal", "A", "D")
        Input.update_binding("Vertical", "S", "W")

        gameobject.add(Transform.new(Vec2.new(100,100)))
        gameobject.add(Sprite.new("assets/test.png"))
        gameobject.add(Rigidbody.new())
        gameobject.add(Animator.new())
        gameobject.add(Text.new("Hello Lilah!", "assets/Lora-Regular.ttf"))
        gameobject.add(Sfx.new("sfx", "assets/sfx.wav"))
        gameobject.add(this.as_behaviour)

        gameobject = Lilah.instantiate(gameobject)
    }

    static start() {
        Animator.insert_state(gameobject.ref, "Row0", Vec2.new(3, 0))
        Animator.insert_state(gameobject.ref, "Row1", Vec2.new(3, 2))
        Animator.set_speed(gameobject.ref, 2)
        Animator.set_state(gameobject.ref, "Row1")
        Animator.play(gameobject.ref)
        Sprite.cut_sprite_sheet(gameobject.ref, Vec2.new(0, 0), Vec2.new(3, 3))
        Sprite.set_sort(gameobject.ref, 2)
        
        gameobject.data = Json.parse(Fs.read("examples/misc/pos.json"))
        Rigidbody.set_position(gameobject.ref, Serializable.wrapper({"math": "Vec2"}, "data", [["pos", Vec2]]).deserialize(gameobject.data).pos)

       gameobject["rot"] = 0

        Tween.new(0, 10).time(10).onComplete{ System.print("Complete") }.play { |v| System.print(v) }
    }

    static update() {
        Rigidbody.set_velocity(gameobject.ref, Input.binding2D("Horizontal", "Vertical")*100)
        gameobject["rot"] = gameobject["rot"] + 0.5 * Lilah.delta_time
        Rigidbody.set_rotation(gameobject.ref, gameobject["rot"])

        if(gameobject.ref.get("Rigidbody").velocity.magnitude() > 0.0) {
            Animator.play(gameobject.ref)
        } else {
            Animator.stop(gameobject.ref)
        }
        
        if(gameobject.ref.get("Rigidbody").colliding != null) {
            //Sfx.play(gameobject.ref, "sfx")
            Sprite.set_tint(gameobject.ref, [1,0,0,1])
        } else {
            Sprite.set_tint(gameobject.ref, [1,1,1,1]) 
        }

        // if(Input.key("Space")) {
        //     Lilah.fullscreen = !Lilah.fullscreen
        // }

        Transform.set_position(
            Lilah.camera.ref, 
            Vec2.lerp(
                Lilah.camera.ref.get("Transform").position, 
                gameobject.ref.get("Transform").position, Lilah.delta_time * 10
            )
        )

        if(Input.key_down("Space")) {
            //gameobject.data["pos"] = gameobject.ref.get("Transform").position
            //var p = Serializable.serialize(gameobject.data)
            //Fs.write("examples/misc/pos.json", Json.stringify(p))
            
        }
    }
}
