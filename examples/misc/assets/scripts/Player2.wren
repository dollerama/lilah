import "math" for Vec2
import "app" for State, Input, GameObjectRef
import "engine" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour

class Player2 is Behaviour {
    construct new() {
        super(Player2)
        
        var gameobject = GameObject.new("D")

        gameobject.add_component(Transform.new(Vec2.zero)) 
        gameobject.add_component(Sprite.new("assets/test.png").as_component)  
        gameobject.add_component(Rigidbody.new().as_component)
        gameobject.add_component(this.behaviour)

        gameobject = State.instantiate(gameobject, {"message": "hi2"})
    }

    static start(id) {
        var gameobject = GameObjectRef.new(id)
    }
    
    static update(id) {
        var gameobject = GameObjectRef.new(id)
        var dir = Input.mouse_pos-gameobject.ref.get_component("Transform").position
        dir.normalize()
        Rigidbody.set_velocity(gameobject.ref, dir)
    }
}