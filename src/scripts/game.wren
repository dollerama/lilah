import "math" for Vec2
import "app" for Lilah, GameObjectRef
import "io" for Serializable
///{module} Game

///{class} Behaviour
///Inherits from Serializable
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
    ///{static method} start() -> Null
    ///Runs the frame after setup.
    static start() {}
    ///{static method} update() -> Null
    ///Run every frame.
    static update() {}
    ///{static method} onCollision() -> Null
    ///Runs every frame after start that the Behaviour has a collision given a Rigidbody and Transform is attached.
    static onCollision(collision) {}

    ///{method} setup() -> Null
    ///Runs the first frame regardless of whether or not the Behaviour is attached.
    setup() {}
    ///{method} start() -> Null
    ///Runs the second frame regardless of whether or not the Behaviour is attached.
    start() {}
    ///{method} update() -> Null
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
    ///{static method} set_text(go: GameObject, text: String) -> Null
    foreign static set_text(go, text)
    ///{static method} set_font(go: GameObject, font: String) -> Null
    foreign static set_font(go, font)
    ///{static method} set_font_size(go: GameObject, fs: Num) -> Null
    foreign static set_font_size(go, fs)
}

foreign class Sprite {
    construct new(i) {}
    foreign as_component
    foreign size
    foreign texture_id
    foreign current_index
    foreign tint
    foreign cut_sprite_sheet(i, j)

    foreign static cut_sprite_sheet(go, i, j)
    foreign static set_sort(go, i)
    foreign static set_tint(go, color)
}

foreign class Scene {
    construct new(i) {}
    foreign as_component
}

foreign class Rigidbody {
    construct new() {}
    foreign as_component
    foreign position
    foreign velocity
    foreign velocity=(value)
    foreign solid
    foreign solid=(value)
    foreign colliding

    foreign static colliding(go)
    foreign static set_solid(go, solid)
    foreign static set_position(go, new_pos)
    foreign static set_position_x(go, new_x)
    foreign static set_position_y(go, new_y)
    foreign static set_velocity(go, vel)
    foreign static set_velocity_x(go, new_x)
    foreign static set_velocity_y(go, new_y)
    foreign static update_velocity(go, vel)
    foreign static update_velocity_x(go, new_x)
    foreign static update_velocity_y(go, new_y)
    foreign static set_rotation(go, new_rot)
}

foreign class Animator {
    construct new() {}
    foreign as_component
    foreign playing
    foreign speed
    foreign speed=(value)
    foreign frame
    foreign frame=(value)
    foreign play()
    foreign stop()
    foreign get_state(s)
    foreign set_state(s)
    foreign insert_state(s, i)

    foreign static play(g)
    foreign static stop(g)
    foreign static set_state(g, s)
    foreign static get_state(g, s)
    foreign static insert_state(g, s, i)
    foreign static set_speed(g, s)
    foreign static set_frame(g, f)
}

foreign class Transform is Serializable {
    construct new(p) {}

    foreign as_component

    foreign position
    foreign scale
    foreign rotation
    foreign pivot
    #!position(ord = 0)
    foreign position=(value)
    #!scale(ord = 1)
    foreign scale=(value)
    #!rotation(ord = 2)
    foreign rotation=(value)
    #!pivot(ord = 3)
    foreign pivot=(value)

    static default { Transform.new(Vec2.new(0, 0)) }

    getProperty() {
        return properties([[this.position, Vec2], [this.scale, Vec2], this.rotation, [this.pivot, Vec2]])
    }

    setProperty() {
        return properties {|v|
            this.position = v.call()
            this.scale = v.call()
            this.rotation = v.call()
            this.pivot = v.call()
        }
    }

    foreign static set_pivot(go, new_pivot)
    foreign static set_position(go, new_pos)
    foreign static set_position_x(go, new_x)
    foreign static set_position_y(go, new_y)
    foreign static update_position(go, new_pos)
    foreign static update_position_x(go, new_x)
    foreign static update_position_y(go, new_y)

    foreign static set_scale(go, new_scale)
    foreign static set_scale_x(go, new_x)
    foreign static set_scale_y(go, new_y)
    foreign static update_scale(go, new_scale)
    foreign static update_scale_x(go, new_x)
    foreign static update_scale_y(go, new_y)

    foreign static set_rotation(go, new_rot)
    foreign static update_rotation(go, new_rot)
}

foreign class GameObject {
    construct new(name) {}

    foreign add(x)
    foreign get(x)
    foreign set(x, y)
    foreign id
    foreign uuid
    foreign name
    foreign name=(v)
}

foreign class Sfx {
    construct new(name, file) {}

    foreign as_component
    foreign name
    foreign name=(v)
    foreign volume
    foreign volume=(v)
    foreign file
    foreign play()
    foreign static get_volume(go, name)
    foreign static set_volume(go, name, amt)
    foreign static play(go, name)
}

foreign class ComponentBehaviour {
    construct new(b) { }
    foreign as_component
    foreign uuid
}
