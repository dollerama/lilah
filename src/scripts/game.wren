import "math" for Vec2
import "app" for Lilah, GameObjectRef, Tween
import "io" for Serializable
import "random" for Random
///{module} Game

///{class} Behaviour : Serializable
class Behaviour is Serializable {
    ///{static getter} data -> Map
    static data { __data }
    ///{static setter} data = v: Map
    static data=(v) { __data = v }

    ///{static getter} [i: Any] -> Any
    static [i] {
        return __data[i]
    }

    ///{static setter} [i: Any] = v: Any
    static [i] = (v) {
        return __data[i] = v
    }

    ///{getter} frame -> Num
    frame {
        if(_frame == null) {
            _frame = 0
        }
        return _frame
    }
    ///{setter} frame = v: Num
    frame=(v) {_frame=v}
    ///{getter} as_behaviour -> ComponentBehaviour
    ///Example:
    ///```js
    ///gameobject.add(ParticleSystem.new(gameobject).as_behaviour)
    ///```
    as_behaviour { _behaviour }

    ///{constructor} new(g: Gameobject, c: Behaviour) -> Behaviour
    construct new(g, c) {
        if(__data == null) {
            __data = {}
        }
        if(__data[g.uuid] == null) {
            __data[g.uuid] = {}
        }
        if(__data[g.uuid]["%(c)"] == null) {
            __data[g.uuid]["%(c)"] = {}
        }
        

        var b = ComponentBehaviour.new("%(c)")
        _behaviour = b.as_component
        __data[g.uuid]["%(c)"][b.uuid] = c.new()
    }
    ///{static method} start() -> null
    ///Runs the frame after setup.
    static start() {}
    ///{static method} update() -> null
    ///Run every frame.
    static update() {}
    ///{static method} onCollision() -> null
    ///Runs every frame after start that the Behaviour has a collision given a Rigidbody and Transform is attached.
    static onCollision(collision) {}

    ///{method} setup() -> null
    ///Runs the first frame regardless of whether or not the Behaviour is attached.
    setup() {}
    ///{method} start() -> null
    ///Runs the second frame regardless of whether or not the Behaviour is attached.
    start() {}
    ///{method} update() -> null
    ///Runs every frame after start regardless of whether or not the Behaviour is attached.
    update() {}
}

///{class} Component
///Rust dyn obj that all components derive from
foreign class Component {}

///{class} Text
foreign class Text {
    ///{constructor} new(text: String, font: String) -> Text
    construct new(text, font) {}
    ///{getter} as_component -> Component
    foreign as_component 
    ///{getter} text -> String
    foreign text
    ///{getter} font -> String
    foreign font
    ///{getter} font_size -> Num
    foreign font_size
    ///{setter} text = v: String
    foreign text=(v)
    ///{setter} font = v: String
    foreign font=(v)
    ///{setter} font_size = v: Num
    foreign font_size=(v)
    ///{static method} get_text(go: GameObject) -> String
    foreign static get_text(go)
    ///{static method} get_font(go: GameObject) -> String
    foreign static get_font(go)
    ///{static method} get_font_size(go: GameObject) -> Num
    foreign static get_font_size(go)
    ///{static method} set_text(go: GameObject, text: String) -> null
    foreign static set_text(go, text)
    ///{static method} set_font(go: GameObject, font: String) -> null
    foreign static set_font(go, font)
    ///{static method} set_font_size(go: GameObject, fs: Num) -> null
    foreign static set_font_size(go, fs)
}

///{class} Sprite
foreign class Sprite {
    ///{constructor} new(id: String) -> Sprite
    construct new(id) {}
    ///{getter} as_component -> Component
    foreign as_component
    ///{getter} size -> Vec2
    foreign size
    ///{getter} texture_id -> String
    foreign texture_id
    ///{getter} current_index -> Vec2
    foreign current_index
    ///{getter} tint -> List
    ///Returns in the form [r,g,b,a]
    foreign tint
    ///{method} cut_sprite_sheet(i: Vec2, j: Vec2) -> null
    foreign cut_sprite_sheet(i, j)
    ///{method} cut_sprite_sheet(go: Gameobject, i: Vec2, j: Vec2) -> null
    foreign static cut_sprite_sheet(go, i, j)
    ///{method} set_sort(go: Gameobject, i: Num) -> null
    foreign static set_sort(go, i)
    ///{method} set_tint(go: Gameobject, color: List) -> null
    foreign static set_tint(go, color)
}

///{class} Scene
foreign class Scene {
    ///{constructor} new(i: String) -> Scene
    construct new(i) {}
    ///{getter} as_component -> Component
    foreign as_component
    ///{getter} markers -> [{String: Vec2}]
    foreign markers
    ///{method} getMarker(index: String) -> [Vec2] | Vec2
    ///either returns the Vec2 that is mapped to the String or a list of Vec2's if the String has multiple mappings.
    getMarker(index) {
        var result = []
        for(i in markers) {
            if(i[index] != null) {
                result.add( i[index] )
            }
        }

        if(result.count == 1) {
            return result[0]
        } else {
            return result
        }
    }
}

///{class} Rigidbody
foreign class Rigidbody {
    ///{constructor} new() -> Rigidbody
    construct new() {}
    ///{getter} as_component -> Component
    foreign as_component
    ///{getter} position -> Vec2
    foreign position
    ///{getter} velocity -> Vec2
    foreign velocity
    ///{setter} velocity = value: Vec2
    foreign velocity=(value)
    ///{getter} solid -> bool
    foreign solid
    ///{setter} solid = value: bool
    foreign solid=(value)
    ///{getter} colliding -> Map/null
    ///returns a map in the form "name": _, "uuid": _ or null if no collision
    foreign colliding
    ///{static method} colliding(go: Gameobject) -> Map/null
    ///returns a map in the form "name": _, "uuid": _ or null if no collision
    foreign static colliding(go)
    ///{static method} set_solid(go: Gameobject, solid: bool) -> null
    foreign static set_solid(go, solid)
    ///{static method} set_position(go: Gameobject, new_pos: Vec2) -> null
    foreign static set_position(go, new_pos)
    ///{static method} set_position_x(go: Gameobject, new_x: Num) -> null
    foreign static set_position_x(go, new_x)
    ///{static method} set_position_y(go: Gameobject, new_y: Num) -> null
    foreign static set_position_y(go, new_y)
    ///{static method} set_velocity(go: Gameobject, vel: Vec2) -> null
    foreign static set_velocity(go, vel)
    ///{static method} set_velocity_x(go: Gameobject, new_x: Num) -> null
    foreign static set_velocity_x(go, new_x)
    ///{static method} set_velocity_y(go: Gameobject, new_y: Num) -> null
    foreign static set_velocity_y(go, new_y)
    ///{static method} update_velocity(go: Gameobject, vel: Vec2) -> null
    foreign static update_velocity(go, vel)
    ///{static method} update_velocity_x(go: Gameobject, new_x: Num) -> null
    foreign static update_velocity_x(go, new_x)
    ///{static method} set_velocity(go: Gameobject, new_y: Num) -> null
    foreign static update_velocity_y(go, new_y)
    ///{static method} set_rotation(go: Gameobject, new_rot: Num) -> null
    foreign static set_rotation(go, new_rot)
}

///{class} Animator
foreign class Animator {
    ///{constructor} new() -> Animator
    construct new() {}
    ///{getter} as_component -> Component
    foreign as_component
    ///{getter} playing -> bool
    foreign playing
    ///{getter} speed -> Num
    foreign speed
    ///{setter} speed = value: Num
    foreign speed=(value)
    ///{getter} frame -> Num
    foreign frame
    ///{setter} frame = value: Num
    foreign frame=(value)
    ///{method} play() -> null
    foreign play()
    ///{method} stop() -> null
    foreign stop()
    ///{method} get_state(s: String) -> Map
    ///returns map in the form state: value:Vec2
    foreign get_state(s)
    ///{method} set_state(s: String) -> null
    foreign set_state(s)
    ///{method} insert_state(s: String, i: Vec2) -> null
    foreign insert_state(s, i)
    ///{static method} play(go: Gameobject) -> null
    foreign static play(g)
    ///{static method} stop(go: Gameobject) -> null
    foreign static stop(g)
    ///{static method} set_state(go: Gameobject, s: String) -> null
    foreign static set_state(g, s)
    ///{static method} get_state(go: Gameobject, s: String) -> String
    foreign static get_state(g, s)
    ///{static method} insert_state(go: Gameobject, s: String, i: Vec2) -> null
    foreign static insert_state(g, s, i)
    ///{static method} set_speed(go: Gameobject, s: Num) -> null
    foreign static set_speed(g, s)
    ///{static method} set_frame(go: Gameobject, f: Num) -> null
    foreign static set_frame(g, f)
}

///{class} Transform : Serializable
foreign class Transform is Serializable {
    ///{constructor} new(p: Vec2) -> Transform
    construct new(p) {}
    ///{getter} as_component -> Component
    foreign as_component
    ///{getter} position -> Vec2
    foreign position
    ///{getter} scale -> Vec2
    foreign scale
    ///{getter} rotation -> Num
    foreign rotation
    ///{getter} pivot -> Vec2
    foreign pivot
    ///{setter} position = value: Vec2
    #!position(ord = 0)
    foreign position=(value)
    ///{setter} scale = value: Vec2
    #!scale(ord = 1)
    foreign scale=(value)
    ///{setter} rotation = value: Num
    #!rotation(ord = 2)
    foreign rotation=(value)
    ///{setter} pivot = value: Vec2
    #!pivot(ord = 3)
    foreign pivot=(value)
    ///{static getter} default -> Transform
    static default { Transform.new(Vec2.new(0, 0)) }

    ///{method} getProperty() -> List/Fn
    getProperty() {
        return properties([[this.position, Vec2], [this.scale, Vec2], this.rotation, [this.pivot, Vec2]])
    }
    ///{method} setProperty() -> null
    setProperty() {
        return properties {|v|
            this.position = v.call()
            this.scale = v.call()
            this.rotation = v.call()
            this.pivot = v.call()
        }
    }
    ///{static method} set_pivot(go: Gameobject, new_pivot: Vec2) -> null
    foreign static set_pivot(go, new_pivot)
    ///{static method} set_position(go: Gameobject, new_pos: Vec2) -> null
    foreign static set_position(go, new_pos)
    ///{static method} set_position_x(go: Gameobject, new_x: Num) -> null
    foreign static set_position_x(go, new_x)
    ///{static method} set_position_y(go: Gameobject, new_y: Num) -> null
    foreign static set_position_y(go, new_y)
    ///{static method} update_position(go: Gameobject, new_pos: Vec2) -> null
    foreign static update_position(go, new_pos)
    ///{static method} update_position_x(go: Gameobject, new_x: Num) -> null
    foreign static update_position_x(go, new_x)
    ///{static method} update_position_y(go: Gameobject, new_y: Num) -> null
    foreign static update_position_y(go, new_y)
    ///{static method} set_scale(go: Gameobject, new_scale: Vec2) -> null
    foreign static set_scale(go, new_scale)
    ///{static method} set_scale_x(go: Gameobject, new_x: Num) -> null
    foreign static set_scale_x(go, new_x)
    ///{static method} set_scale_y(go: Gameobject, new_y: Num) -> null
    foreign static set_scale_y(go, new_y)
    ///{static method} update_scale(go: Gameobject, new_scale: Vec2) -> null
    foreign static update_scale(go, new_scale)
    ///{static method} update_scale_x(go: Gameobject, new_x: Num) -> null
    foreign static update_scale_x(go, new_x)
    ///{static method} update_scale_y(go: Gameobject, new_y: Num) -> null
    foreign static update_scale_y(go, new_y)
    ///{static method} set_rotation(go: Gameobject, new_rot: Num) -> null
    foreign static set_rotation(go, new_rot)
    ///{static method} update_rotation(go: Gameobject, new_rot: Num) -> null
    foreign static update_rotation(go, new_rot)
}

///{class} GameObject
foreign class GameObject {
    ///{constructor} new(name: String) -> Gameobject
    construct new(name) {}
    ///{method} add(x: Component) -> null
    foreign addComponent(x)
    ///{method} getComponent(x: String) -> Component
    foreign getComponent(x)
    ///{method} set(x: Type, y: Component) -> null
    get(x) {
        return getComponent("%(x)")
    }

    add(x) {
        if(x.toString.contains("instance")) {
            addComponent(x)
        } else {
            addComponent(x.new(this).as_behaviour)
        }
    }
    ///{method} set(x: String, y: Component) -> null
    foreign set(x, y)
    ///{getter} id -> Map
    ///Returns a map in the form "name": _, "uuid": _
    foreign id
    ///{getter} uuid -> String
    foreign uuid
    ///{getter} name -> String
    foreign name
    ///{setter} name = v: String
    foreign name=(v)
    ///{getter} components -> [Component]
    foreign components

    toString {
        var result = "%(id)"
        return result
    }
}

///{class} Sfx
foreign class Sfx {
    ///{constructor} new(name: String, file: String) -> Sfx
    construct new(name, file) {}
    ///{getter} as_component -> Component
    foreign as_component
    ///{getter} name -> String
    foreign name
    ///{setter} name = v: String
    foreign name=(v)
    ///{getter} volume -> Num
    foreign volume
    ///{setter} volume = v: Num
    foreign volume=(v)
    ///{getter} file -> String
    foreign file
    ///{method} play() -> null
    foreign play()
    ///{static method} get_volume(go: Gameobject, name: String) -> Num
    foreign static get_volume(go, name)
    ///{static method} get_volume(go: Gameobject, name: String, amt: Num) -> null
    foreign static set_volume(go, name, amt)
    ///{static method} play(go: Gameobject, name: String) -> null
    foreign static play(go, name)
}

///{class} ComponentBehaviour
foreign class ComponentBehaviour {
    ///{constructor} new(b: String) -> ComponentBehaviour
    construct new(b) { }
    ///{getter} as_component -> Component
    foreign as_component
    ///{getter} uuid -> String
    foreign uuid
}

///{class} Debug
foreign class Debug {
    ///{static method} drawLine(start: Vec2, end: Vec2, color: [num]) -> null
    foreign static drawLine(start, end, color)

    // debugRigidbody() {
    //     for(g in Lilah.gameobjects) {
    //         var r = g.get(Rigidbody)
    //         if(r != null) {
    //             Debug.drawLine(r.)
    //         }
    //     }
    // }

    static printFrameInfo() {
        System.print("Debug {")
        System.print("\tFps: %(Lilah.fps),")
        System.print("\tDelta: %(Lilah.delta_time),")
        System.print("\tGameobjects: %(Lilah.gameobjects.count),")
        System.print("\tDataCount: %(Lilah.data.count),")
        System.print("\tTweens: %(Tween.tweenCount),")
        System.print("\tFibers: %(Lilah.fiberCount),")
        System.print("\tGameobjects: {")
        System.print("\t\t%(Lilah.gameobjects)")
        System.print("\t},")
        System.print("\tData: {")
        System.print("\t\t%(Lilah.data)")
        System.print("\t},")
        System.print("\tTweens: {")
        System.print("\t\t%(Tween.tweens)")
        System.print("\t}")
        System.print("}")
    }
}
