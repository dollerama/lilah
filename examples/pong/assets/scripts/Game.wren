import "math" for Vec2
import "app" for Lilah, Input, GameObjectRef, Audio
import "game" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour, Text, Sfx
import "Paddle" for Paddle
import "Ball" for Ball

class Game is Behaviour {
    construct new() {
        super(Game)
    }

    setup() {
        Input.update_binding("Vertical1", Input.Keycode.S, Input.Keycode.W)
        Input.update_binding("Vertical2", Input.Keycode.Down, Input.Keycode.Up)
        
        var player1 = GameObject.new("P1")
        player1.add(Transform.new(Vec2.new(-400+16,0))) 
        player1.add(Sprite.new("assets/paddle.png"))  
        player1.add(Rigidbody.new())
        player1.add(Paddle.new().as_behaviour)

        var player2 = GameObject.new("P2")
        player2.add(Transform.new(Vec2.new(400-16,0)))
        player2.add(Sprite.new("assets/paddle.png"))  
        player2.add(Rigidbody.new())
        player2.add(Paddle.new().as_behaviour)

        var ball = GameObject.new("Ball")
        ball.add(Transform.new(Vec2.new(0, 0)))
        ball.add(Sprite.new("assets/ball.png"))  
        ball.add(Rigidbody.new())
        ball.add(Ball.new().as_behaviour)

        var line = GameObject.new("line")
        line.add(Transform.new(Vec2.new((0)-4, 0)))
        line.add(Sprite.new("assets/line.png"))  
        Sprite.set_tint(line, [1,1,1,0.5])

        var score_1 = GameObject.new("Score1")
        score_1.add(Transform.new(Vec2.new(-(400/4), 270)))
        score_1.add(Text.new("0", "assets/Lora-Regular.ttf"))

        var score_2 = GameObject.new("Score2")
        score_2.add(Transform.new(Vec2.new((400/4), 270)))
        score_2.add(Text.new("0", "assets/Lora-Regular.ttf"))

        Lilah.instantiate(player1, {"controls": "Vertical1", "score": 0})
        Lilah.instantiate(player2, {"controls": "Vertical2", "score": 0})
        Lilah.instantiate(ball)
        Lilah.instantiate(line)
        Lilah.instantiate(score_1)
        Lilah.instantiate(score_2)
    }
}