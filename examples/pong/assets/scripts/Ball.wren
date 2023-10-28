import "math" for Vec2
import "app" for Lilah, Input, GameObjectRef, Audio
import "game" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour, Text, Sfx

class Ball is Behaviour {
    static gameobject { __gameobject }
    static gameobject=(v) { __gameobject = GameObjectRef.new(v) }

    construct new() {
        super(Ball)
    }

    static start() {
        Rigidbody.set_velocity_x(gameobject.ref, -1)
    }
    
    static update() {
        if(gameobject.ref.get("Transform").position.y > 600-10 || gameobject.ref.get("Transform").position.y < 0) {
            Rigidbody.set_velocity_y(gameobject.ref, gameobject.ref.get("Rigidbody").velocity.y*-1)
        }

        if(gameobject.ref.get("Transform").position.x > 800) {
            Lilah.start_fiber(Fiber.new {
                Rigidbody.set_position(gameobject.ref, Vec2.new(800/2, 600/2))
                Lilah.find("P2").data["score"] = Lilah.find("P2").data["score"]+1
                if(Lilah.find("P2").data["score"] > 4) {
                    Lilah.find("P1").data["score"] = 0
                    Lilah.find("P2").data["score"] = 0
                }

                Text.set_text(Lilah.find("Score1").ref, "%(Lilah.find("P2").data["score"])")
                Rigidbody.set_velocity(gameobject.ref, Vec2.new(0, 0))

                Fiber.yield(1)

                Rigidbody.set_velocity(gameobject.ref, Vec2.new(-1, 0))
            })
        }

        if(gameobject.ref.get("Transform").position.x < 0) {
            Lilah.start_fiber(Fiber.new {
                Rigidbody.set_position(gameobject.ref, Vec2.new(800/2, 600/2))
                Lilah.find("P1").data["score"] = Lilah.find("P1").data["score"]+1
                if(Lilah.find("P1").data["score"] > 4) {
                    Lilah.find("P1").data["score"] = 0
                    Lilah.find("P2").data["score"] = 0
                }

                Text.set_text(Lilah.find("Score2").ref, "%(Lilah.find("P1").data["score"])")
                Rigidbody.set_velocity(gameobject.ref, Vec2.new(0, 0))

                Fiber.yield(1)
                Rigidbody.set_velocity(gameobject.ref, Vec2.new(1, 0))
            })
        }

        if(gameobject.ref.get("Rigidbody").colliding != null) {
            var paddle = Lilah.find(gameobject.ref.get("Rigidbody").colliding["uuid"])
            Rigidbody.set_velocity_x(gameobject.ref, gameobject.ref.get("Rigidbody").velocity.x*-1)

            var paddle_pos = paddle.ref.get("Transform").position-Vec2.new(7, 50)
            var ball_pos = gameobject.ref.get("Transform").position-Vec2.new(10, 10)
            var dist = (ball_pos-paddle_pos).normalized()

            var dot = Vec2.dot(Vec2.up, dist)
            Rigidbody.set_velocity_y(gameobject.ref, dot)
        }
    }
}