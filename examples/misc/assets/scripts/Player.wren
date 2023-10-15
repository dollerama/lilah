import "math" for Vec2
import "app" for State, Input, GameObjectRef, Audio
import "engine" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour, Text, Sfx

class Player is Behaviour {
    construct new() {
        super(Player)
        
        var gameobject = GameObject.new("C")

        Input.update_binding("Horizontal", "A", "D")
        Input.update_binding("Vertical", "W", "S")

        gameobject.add(Transform.new(Vec2.new(500,0))) 
        gameobject.add(Sprite.new("assets/test.png"))  
        gameobject.add(Rigidbody.new())
        gameobject.add(Animator.new())
        gameobject.add(Text.new("Hello Lilah!", "assets/Lora-Regular.ttf"))
        gameobject.add(Sfx.new("sfx", "assets/sfx.wav"))
        gameobject.add(this.as_behaviour)

        gameobject = State.instantiate(gameobject)
    }

    static start(id) {
        var gameobject = GameObjectRef.new(id)
        Animator.insert_state(gameobject.ref, "Row0", Vec2.new(3, 0))
        Animator.insert_state(gameobject.ref, "Row1", Vec2.new(3, 1))
        Animator.set_speed(gameobject.ref, 2)
        Animator.set_state(gameobject.ref, "Row0")
        Animator.play(gameobject.ref)
        Sprite.cut_sprite_sheet(gameobject.ref, Vec2.new(0, 0), Vec2.new(3, 3))
    }
    
    static update(id) {
        var gameobject = GameObjectRef.new(id)
        
        Rigidbody.set_velocity(gameobject.ref, Input.binding2D("Horizontal", "Vertical")*5)

        if(gameobject.ref.get("Rigidbody").velocity.magnitude() > 0.0) {
            Animator.play(gameobject.ref)
        } else {
            Animator.stop(gameobject.ref)
        }

        Text.set_text(gameobject.ref, "%(gameobject.ref.get("Rigidbody").velocity.x) x %(gameobject.ref.get("Rigidbody").velocity.y)")

        if(gameobject.ref.get("Rigidbody").colliding != null) {
            Sfx.play(gameobject.ref, "sfx")
        }
        
        if(Input.key("Space")) {
            State.fullscreen = !State.fullscreen
        }

        if(Input.key_down("O")) {
            Audio.play("assets/test.mp3", 5000)
        }
    }
}