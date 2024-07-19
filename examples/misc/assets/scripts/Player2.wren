import "math" for Vec2
import "app" for Lilah, Input, GameObjectRef, UI
import "game" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour, Scene

class Player2 is Behaviour {
    static gameobject { __gameobject }
    static gameobject=(v) { __gameobject = GameObjectRef.new(v) }

    construct new() {
        super(Player2)
    }

    setup() {
        var gameobject = GameObject.new("D")

        gameobject.add(Transform.new(Vec2.new(100,100)))
        gameobject.add(Sprite.new("assets/test.png"))
        gameobject.add(Rigidbody.new())
        gameobject.add(Animator.new())
        gameobject.add(this.as_behaviour)

        gameobject = Lilah.instantiate(gameobject, {})

        var scene = GameObject.new("scene")
        scene.add(Transform.new(Vec2.new(0,0)))
        scene.add(Scene.new("assets/Untitled.json"))
        scene.add(Rigidbody.new())
        scene = Lilah.instantiate(scene, {})
    }

    static start() {
        Animator.insert_state(gameobject.ref, "Row0", Vec2.new(3, 0))
        Animator.insert_state(gameobject.ref, "Row1", Vec2.new(3, 2))
        Animator.set_speed(gameobject.ref, 2)
        Animator.set_state(gameobject.ref, "Row1")
        Animator.play(gameobject.ref)
        Sprite.cut_sprite_sheet(gameobject.ref, Vec2.new(0, 0), Vec2.new(3, 3))
        Sprite.set_sort(gameobject.ref, 2)
        Rigidbody.set_rotation(gameobject.ref, 4)
    }

    static update() {
        if(gameobject.ref.get("Rigidbody").velocity.magnitude() > 0.0) {
            Animator.play(gameobject.ref)
        } else {
            Animator.stop(gameobject.ref)
        }
    }
}
