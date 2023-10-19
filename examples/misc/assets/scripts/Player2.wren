import "math" for Vec2
import "app" for Lilah, Input, GameObjectRef, UI
import "game" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour

class Player2 is Behaviour {
    static gameobject { __gameobject }
    static gameobject=(v) { __gameobject = GameObjectRef.new(v) }

    construct new() {
        super(Player2)
    }

    setup() {      
        var gameobject = GameObject.new("D")

        gameobject.add(Transform.new(Lilah.to_world_space(Vec2.new(800/2, 600/2)))) 
        gameobject.add(Sprite.new("assets/test.png"))  
        gameobject.add(Rigidbody.new())
        gameobject.add(this.as_behaviour)

        gameobject = Lilah.instantiate(gameobject, {"message": "hi2"})
    }

    static start() {
        //Transform.set_scale(gameobject.ref, Vec2.new(2,2))
        //Transform.set_pivot(gameobject.ref, gameobject.ref.get("Sprite").size/2)
    }
    
    static update() {}
}