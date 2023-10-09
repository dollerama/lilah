import "math" for Vec2
import "app" for State, Input, GameObjectRef, Audio
import "engine" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour, Text, Sfx

class Player is Behaviour {
    construct new() {
        super(Player)
        
        var gameobject = GameObject.new("C")

        Input.update_binding("Horizontal", "A", "D")
        Input.update_binding("Vertical", "W", "S")

        gameobject.add_component(Transform.new(Vec2.new(500,0))) 
        gameobject.add_component(Sprite.new("assets/test.png").as_component)  
        gameobject.add_component(Rigidbody.new().as_component)
        gameobject.add_component(Animator.new().as_component)
        gameobject.add_component(Text.new("Hello Lilah!", "assets/Lora-Regular.ttf").as_component)
        gameobject.add_component(Sfx.new("sfx", "assets/sfx.wav").as_component)
        gameobject.add_component(this.behaviour)

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

        if(gameobject.ref.get_component("Rigidbody").velocity.magnitude() > 0.0) {
            Animator.play(gameobject.ref)
        } else {
            Animator.stop(gameobject.ref)
        }

        Text.set_text(gameobject.ref, "%(gameobject.ref.get_component("Rigidbody").velocity.x) x %(gameobject.ref.get_component("Rigidbody").velocity.y)")

        if(gameobject.ref.get_component("Rigidbody").colliding != null) {
            //State.destroy(gameobject.ref.get_component("Rigidbody").colliding["uuid"])
            Sfx.play(gameobject.ref, "sfx")
        }
        
        if(Input.key("Space")) {
            //State.fullscreen = !State.fullscreen
            Transform.update_position_x(State.camera.ref, 2)
        }

        if(Input.key_down("O")) {
            Audio.play("assets/test.mp3", 5000)
        }
    }
}