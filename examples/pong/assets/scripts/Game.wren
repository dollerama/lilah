import "math" for Vec2
import "app" for State, Input, GameObjectRef, Audio
import "engine" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour, Text, Sfx
import "Paddle" for Paddle
import "Ball" for Ball

class Game is Behaviour {
    construct new() {
        super(Game)

        //State.fullscreen = true

        Input.update_binding("Vertical1", "W", "S")
        Input.update_binding("Vertical2", "Up", "Down")
        
        var player1 = GameObject.new("P1")
        player1.add(Transform.new(Vec2.new(0,300-16))) 
        player1.add(Sprite.new("assets/paddle.png"))  
        player1.add(Rigidbody.new())
        player1.add(Paddle.new().as_behaviour)

        var player2 = GameObject.new("P2")
        player2.add(Transform.new(Vec2.new(800-16,300-16))) 
        player2.add(Sprite.new("assets/paddle.png"))  
        player2.add(Rigidbody.new())
        player2.add(Paddle.new().as_behaviour)

        var ball = GameObject.new("Ball")
        ball.add(Transform.new(Vec2.new((800/2), (600/2)))) 
        ball.add(Sprite.new("assets/ball.png"))  
        ball.add(Rigidbody.new())
        ball.add(Ball.new().as_behaviour)

        var line = GameObject.new("line")
        line.add(Transform.new(Vec2.new((800/2)-4, 0))) 
        line.add(Sprite.new("assets/line.png"))  

        var score_1 = GameObject.new("Score1")
        score_1.add(Transform.new(Vec2.new((800/4), 10))) 
        score_1.add(Text.new("0", "assets/Lora-Regular.ttf"))

        var score_2 = GameObject.new("Score2")
        score_2.add(Transform.new(Vec2.new(800-(800/4), 10))) 
        score_2.add(Text.new("0", "assets/Lora-Regular.ttf"))

        State.instantiate(player1, {"controls": "Vertical1", "score": 0})
        State.instantiate(player2, {"controls": "Vertical2", "score": 0})
        State.instantiate(ball)
        State.instantiate(line)
        State.instantiate(score_1)
        State.instantiate(score_2)
    }
}