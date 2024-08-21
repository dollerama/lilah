import "math" for Vec2
import "app" for Lilah, Input, GameObjectRef, Audio, Tween
import "game" for GameObject, Animator, Transform, Behaviour, Sprite, Rigidbody, ComponentBehaviour, Text, Sfx, Debug, Scene
import "ParticleSystem" for ParticleSystem, ParticleField
import "io" for Fs, Json, Serializable
import "CamFollow" for CamFollow

class Player is Behaviour {
    construct new() {
    }

    setup() {
        Input.update_binding("Horizontal", "A", "D")
        Input.update_binding("Vertical", "S", "W")

        var gameobject = GameObject.new("Player")
        gameobject.add(Transform.new(Vec2.new(0,0)))
        gameobject.add(Sprite.new("assets/skullboy.png"))
        gameobject.add(Rigidbody.new())
        gameobject.add(Player)
        gameobject.add(CamFollow)

        var scene = GameObject.new("scene")
        scene.add(Transform.new(Vec2.new(0,0)))
        scene.add(Scene.new("assets/Untitled.json"))
        scene.add(Rigidbody.new())
        Lilah.instantiate(scene)

        var g = Lilah.instantiate(gameobject)
        g.behaviourData(CamFollow).speed = 10
    }

    static start() {
        Sprite.set_sort(gameobject.ref, 10)  
        Rigidbody.set_position(gameobject.ref, Lilah.find("scene").ref.get(Scene).getMarker("Start"))  
    }

    static update() {
        //System.print(Lilah.time)

        Rigidbody.set_velocity(gameobject.ref, Input.binding2D("Horizontal", "Vertical")*50)
        

        if(Input.key_down("Space")) {
            Sprite.set_sort(gameobject.ref, 0)
        }
    }
}
