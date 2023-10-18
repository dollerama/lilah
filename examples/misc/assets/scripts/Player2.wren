import "math" for Vec2
import "app" for State, Input, GameObjectRef, UI
import "game" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour

class Player2 is Behaviour {
    construct new() {
        super(Player2)
        
        var gameobject = GameObject.new("D")

        gameobject.add(Transform.new(Vec2.new(800/2, 600/2))) 
        gameobject.add(Sprite.new("assets/test.png"))  
        gameobject.add(Rigidbody.new())
        gameobject.add(this.as_behaviour)

        gameobject = State.instantiate(gameobject, {"message": "hi2"})
    }

    static start(id) {
        var gameobject = GameObjectRef.new(id)
        //Transform.set_scale(gameobject.ref, Vec2.new(2,2))
        //Transform.set_pivot(gameobject.ref, gameobject.ref.get("Sprite").size/2)
    }
    
    static update(id) {
        var gameobject = GameObjectRef.new(id)
    }
}