import "math" for Vec2
import "app" for Lilah, Input, GameObjectRef, UI, Tween, Curve
import "game" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour, Scene

class Player2 is Behaviour {
    dead { _dead }
    dead=(v) {_dead = v}

    construct new() {
    }

    setup() {
        var gameobject = GameObject.new("D")

        gameobject.add(Transform.new(Vec2.new(200,-200)))
        gameobject.add(Sprite.new("assets/test.png"))
        //gameobject.add(Rigidbody.new())
        gameobject.add(Animator.new())
        gameobject.add(Player2)

        Lilah.instantiate(gameobject, {})

        //var scene = GameObject.new("scene")
        //scene.add(Transform.new(Vec2.new(0,0)))
        //scene.add(Scene.new("assets/Untitled.json"))
        //scene.add(Rigidbody.new())
        //Lilah.instantiate(scene)
    }

    static start() { 
        Animator.insert_state(gameobject.ref, "Row0", Vec2.new(3, 0))
        Animator.insert_state(gameobject.ref, "Row1", Vec2.new(3, 2))
        Animator.set_speed(gameobject.ref, 2)
        Animator.set_state(gameobject.ref, "Row1")
        Animator.play(gameobject.ref)
        Sprite.cut_sprite_sheet(gameobject.ref, Vec2.new(0, 0), Vec2.new(3, 3))
        Sprite.set_sort(gameobject.ref, 10)
        //Rigidbody.set_rotation(gameobject.ref, 4)
        gamebehaviour.dead = false
    }

    static update() {
      gameobject.ref.get(Transform).rotation = gameobject.ref.get(Transform).rotation + 10 * Lilah.delta_time
    }

    static onCollision(c) {
        /*
        if(!gamebehaviour.dead) {
            Rigidbody.set_solid(gameobject.ref, false)
            gamebehaviour.dead = true

            Tween.new(Vec2.one, Vec2.zero)
            .time(2)
            .curve(Curve.inOutElastic)
            .onComplete {
                Lilah.destroy(gameobject)

                var gg = GameObject.new("DD")

                gg.add(Transform.new(Vec2.new(200,-200)))
                gg.add(Sprite.new("assets/test.png"))
                gg.add(Rigidbody.new())
                gg.add(Animator.new())
                gg.add(Player2)

                Lilah.instantiate(gg, {}) 
            }
            .play { |v|
                Transform.set_scale(gameobject.ref, v)
            }
        }
        */
    }
}
