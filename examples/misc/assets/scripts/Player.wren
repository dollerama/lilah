import "math" for Vec2
import "app" for Lilah, Input, GameObjectRef, Audio, Tween
import "game" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour, Text, Sfx, Debug, Scene, Line
import "ParticleSystem" for ParticleSystem, ParticleField
import "Trail" for Trail
import "io" for Fs, Json, Serializable

class Player is Behaviour {
    construct new() {
    }

    setup() {
        Input.update_binding("Horizontal", "A", "D")
        Input.update_binding("Vertical", "S", "W")

        var gameobject = GameObject.new("C")
        gameobject.add(Transform.new(Vec2.new(100,100)))
        gameobject.add(Sprite.new("assets/test.png"))
        gameobject.add(Rigidbody.new())
        gameobject.add(Animator.new())
        gameobject.add(Text.new("Hello Lilah!", "assets/Lora-Regular.ttf"))
        gameobject.add(Sfx.new("sfx", "assets/sfx.wav"))
        gameobject.add(Line.new())
        gameobject.add(Player)
        gameobject.add(ParticleSystem)
        gameobject.add(Trail)

        var g = Lilah.instantiate(gameobject)

        g.behaviourData(ParticleSystem).partSetup = ParticleField.new(Fn.new { |p|
            p.add(Sprite.new("assets/test.png"))
        })

        g.behaviourData(ParticleSystem).partStart = ParticleField.new(Fn.new { |p|
            Sprite.cut_sprite_sheet(p.ref, Vec2.new(0, 0), Vec2.new(3, 3))
        })

        g.behaviourData(ParticleSystem).play()
    }

    static start() {
        Animator.insert_state(gameobject.ref, "Row0", Vec2.new(3, 0))
        Animator.insert_state(gameobject.ref, "Row1", Vec2.new(3, 2))
        Animator.set_speed(gameobject.ref, 2)
        Animator.set_state(gameobject.ref, "Row1")
        Animator.play(gameobject.ref)
        Sprite.cut_sprite_sheet(gameobject.ref, Vec2.new(0, 0), Vec2.new(3, 3))
        Sprite.set_sort(gameobject.ref, 0)
        
        gameobject.data = Json.parse(Fs.read("examples/misc/pos.json"))
        Rigidbody.set_position(gameobject.ref, Serializable.wrapper({"math": "Vec2"}, "data", [["pos", Vec2]]).deserialize(gameobject.data).pos)

        gameobject["rot"] = 0   
    }

    static update() {
        Rigidbody.set_velocity(gameobject.ref, Input.binding2D("Horizontal", "Vertical")*120)
        gameobject["rot"] = gameobject["rot"] + 0.5 * Lilah.delta_time
        //Rigidbody.set_rotation(gameobject.ref, gameobject["rot"])

        if(gameobject.ref.get(Rigidbody).velocity.magnitude() > 0.0) {
            Animator.play(gameobject.ref)
        } else {
            Animator.stop(gameobject.ref)
        }
        
        if(gameobject.ref.get(Rigidbody).colliding != null) {
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
                Lilah.camera.ref.get(Transform).position, 
                gameobject.ref.get(Transform).position, Lilah.delta_time * 10
            )
        )

        Debug.drawLine(gameobject.ref.get(Rigidbody).position, Vec2.zero, [1,1,1,1])

        if(Input.key_down("Space")) {
            // gameobject.data["pos"] = gameobject.ref.get("Transform").position
            // var p = Serializable.serialize(gameobject.data)
            // Fs.write("examples/misc/pos.json", Json.stringify(p))
            //gameobject.behaviourData(ParticleSystem).toggle()
            //Rigidbody.set_position(gameobject.ref, Lilah.find("scene").ref.get(Scene).getMarker("Start"))
            
        } 
    }
}
