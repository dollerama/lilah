import "math" for Vec2
import "app" for State, Input, GameObjectRef, Audio
import "engine" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour, Text, Sfx

class Ball is Behaviour {
    construct new() {
        super(Ball)
    }

    static start(id) {
        var gameobject = GameObjectRef.new(id)
        Rigidbody.set_velocity_x(gameobject.ref, -5)
        Transform.set_pivot(gameobject.ref, gameobject.ref.get("Sprite").size/2)
    }
    
    static update(id) {
        var gameobject = GameObjectRef.new(id)

        if(gameobject.ref.get("Transform").position.y > 600-10 || gameobject.ref.get("Transform").position.y < 0) {
            Rigidbody.set_velocity_y(gameobject.ref, gameobject.ref.get("Rigidbody").velocity.y*-1)
        }

        if(gameobject.ref.get("Transform").position.x > 800) {
            State.start_fiber(Fiber.new {
                Rigidbody.set_position(gameobject.ref, Vec2.new(800/2, 600/2))
                State.find("P2").data["score"] = State.find("P2").data["score"]+1
                if(State.find("P2").data["score"] > 4) {
                    State.find("P1").data["score"] = 0
                    State.find("P2").data["score"] = 0
                }

                Text.set_text(State.find("Score1").ref, "%(State.find("P2").data["score"])")
                Rigidbody.set_velocity(gameobject.ref, Vec2.new(0, 0))

                Fiber.yield(1)

                Rigidbody.set_velocity(gameobject.ref, Vec2.new(-5, 0))
            })
        }

        if(gameobject.ref.get("Transform").position.x < 0) {
            State.start_fiber(Fiber.new {
                Rigidbody.set_position(gameobject.ref, Vec2.new(800/2, 600/2))
                State.find("P1").data["score"] = State.find("P1").data["score"]+1
                if(State.find("P1").data["score"] > 4) {
                    State.find("P1").data["score"] = 0
                    State.find("P2").data["score"] = 0
                }

                Text.set_text(State.find("Score2").ref, "%(State.find("P1").data["score"])")
                Rigidbody.set_velocity(gameobject.ref, Vec2.new(0, 0))

                Fiber.yield(1)
                Rigidbody.set_velocity(gameobject.ref, Vec2.new(5, 0))
            })
        }

        if(gameobject.ref.get("Rigidbody").colliding != null) {
            var paddle = State.find(gameobject.ref.get("Rigidbody").colliding["uuid"])
            Rigidbody.set_velocity_x(gameobject.ref, gameobject.ref.get("Rigidbody").velocity.x*-1)

            var paddle_pos = paddle.ref.get("Transform").position+Vec2.new(7, 50)
            var ball_pos = gameobject.ref.get("Transform").position+Vec2.new(10, 10)
            var dist = (ball_pos-paddle_pos).normalized()

            var dot = Vec2.dot(Vec2.up, dist)
            Rigidbody.set_velocity_y(gameobject.ref, dot*5)
        }
    }
}