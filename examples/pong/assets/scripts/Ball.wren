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
        Rigidbody.set_velocity_x(gameobject.ref, -200)
    }
    
    static update() {
        if(gameobject.ref.get("Transform").position.y > 300-10 || gameobject.ref.get("Transform").position.y < -300+10) {
            Rigidbody.set_velocity_y(gameobject.ref, gameobject.ref.get("Rigidbody").velocity.y*-1)
        }

        var p2_side = gameobject.ref.get("Transform").position.x > 400
        var p1_side = gameobject.ref.get("Transform").position.x < -400

        if(p1_side || p2_side) {
            Lilah.start_fiber(Fiber.new {
                Rigidbody.set_position(gameobject.ref, Vec2.new(0, 0))
                
                if(p2_side) {
                    Lilah.find("P2").data["score"] = Lilah.find("P2").data["score"]+1
                    Text.set_text(Lilah.find("Score1").ref, "%(Lilah.find("P2").data["score"])")
                } else if(p1_side) {
                    Lilah.find("P1").data["score"] = Lilah.find("P1").data["score"]+1
                    Text.set_text(Lilah.find("Score2").ref, "%(Lilah.find("P1").data["score"])")
                }

                if(Lilah.find("P2").data["score"] > 4 || Lilah.find("P1").data["score"] > 4) {
                    Lilah.find("P1").data["score"] = 0
                    Lilah.find("P2").data["score"] = 0
                }
                
                Rigidbody.set_velocity(gameobject.ref, Vec2.new(0, 0))

                Fiber.yield(1)

                if(p2_side) Rigidbody.set_velocity(gameobject.ref, Vec2.new(-200, 0))
                if(p1_side) Rigidbody.set_velocity(gameobject.ref, Vec2.new(200, 0))
            })
        }
    }

    static onCollision(c) {
        var paddle = Lilah.find(c["uuid"])
        Rigidbody.set_velocity_x(gameobject.ref, gameobject.ref.get("Rigidbody").velocity.x*-1)

        var paddle_pos = paddle.ref.get("Transform").position
        var ball_pos = gameobject.ref.get("Transform").position
        var dist = (ball_pos-paddle_pos).normalized()

        var dot = Vec2.dot(Vec2.up, dist)
        Rigidbody.set_velocity_y(gameobject.ref, dot*200)
    }
}