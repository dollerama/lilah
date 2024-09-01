import "math" for Vec2
import "app" for Lilah, Input, GameObjectRef, Audio
import "game" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour, Text, Sfx, Debug
import "Paddle" for Paddle
import "Ball" for Ball
import "ParticleSystem" for ParticleSystem, ParticleField
import "Trail" for Trail

class Game is Behaviour {
    construct new() { }

    setup() {
        Input.update_binding("Vertical1", Input.Keycode.S, Input.Keycode.W)
        Input.update_binding("Vertical2", Input.Keycode.Down, Input.Keycode.Up)
        
        var player1 = GameObject.new("P1")
        player1.add(Transform.new(Vec2.new(-400+16,0))) 
        player1.add(Sprite.new("assets/paddle.png"))  
        player1.add(Rigidbody.new())
        player1.add(Paddle)

        var player2 = GameObject.new("P2")
        player2.add(Transform.new(Vec2.new(400-16,0)))
        player2.add(Sprite.new("assets/paddle.png"))  
        player2.add(Rigidbody.new())
        player2.add(Paddle)

        var ball = GameObject.new("Ball")
        ball.add(Transform.new(Vec2.new(0, 0)))
        ball.add(Sprite.new("assets/ball.png"))  
        ball.add(Rigidbody.new())
        ball.add(Ball)
        //ball.add(ParticleSystem)
        ball.add(Trail)

        var line = GameObject.new("line")
        line.add(Transform.new(Vec2.new((0)-4, 0)))
        line.add(Sprite.new("assets/line.png"))  
        Sprite.set_tint(line, [1,1,1,0.5])

        var score_1 = GameObject.new("Score1")
        score_1.add(Transform.new(Vec2.new(-(400/4), 270)))
        score_1.add(Text.new("0", "assets/Lora-Regular.ttf"))
        Text.set_font_size(score_1, 32)

        var score_2 = GameObject.new("Score2")
        score_2.add(Transform.new(Vec2.new((400/4), 270)))
        score_2.add(Text.new("0", "assets/Lora-Regular.ttf"))
        Text.set_font_size(score_2, 32)

        Lilah.instantiate(player1, {"controls": "Vertical1", "score": 0})
        Lilah.instantiate(player2, {"controls": "Vertical2", "score": 0})
        Lilah.instantiate(ball)
        Lilah.instantiate(line)
        Lilah.instantiate(score_1)
        Lilah.instantiate(score_2)
    }

    update() {
        //System.print(Lilah.fps)
    }
}
