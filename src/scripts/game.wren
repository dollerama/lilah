import "math" for Vec2
import "app" for Lilah, GameObjectRef, Tween
import "io" for Serializable
import "random" for Random

class Behaviour is Serializable {
    ///_ -> Map
    static data { __data }
    ///Any -> Null 
    static data=(v) { __data = v }

    ///_ -> Any
    static [i] {
        return __data[i]
    }

    ///Any, Any -> Null
    static [i] = (v) {
        __data[i] = v
    }

    ///_ -> Num
    frame {
        if(_frame == null) {
            _frame = 0
        }
        return _frame
    }
    ///Num -> Null
    frame=(v) {_frame=v}
    ///_ -> ComponentBehaviour
    ///Example:
    ///```js
    ///gameobject.add(ParticleSystem.new(gameobject).as_behaviour)
    ///```
    as_behaviour { _behaviour }

    ///GameObject, ComponentBehaviour -> Behaviour
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
    ///_ -> Null
    ///Runs the frame after setup.
    static start() {}
    ///_ -> Null
    ///Run every frame.
    static update() {}
    ///Map -> Null
    //Default value Map takes form {"id": id, "name": name}
    ///Runs every frame after start that the Behaviour has a collision given a Rigidbody and Transform is attached.
    static onCollision(collision) {}

    ///_ -> Null
    ///Runs the first frame regardless of whether or not the Behaviour is attached.
    setup() {}
    ///_ -> Null
    ///Runs the second frame regardless of whether or not the Behaviour is attached.
    start() {}
    ///_ -> Null
    ///Runs every frame after start regardless of whether or not the Behaviour is attached.
    update() {}
}

///Rust dyn obj that all components derive from
foreign class Component {}

foreign class Text {
    construct new(text, font) {}
    foreign as_component 
    foreign text
    foreign font
    foreign font_size
    text=(value) { Text.set_text(Lilah.find(this.parent).ref, value) }
    font=(value) { Text.set_font(Lilah.find(this.parent).ref, value) }
    font_size=(value) { Text.set_font_size(Lilah.find(this.parent).ref, value) }
    foreign static get_text(go)
    foreign static get_font(go)
    foreign static get_font_size(go)
    foreign static set_text(go, text)
    foreign static set_font(go, font)
    foreign static set_font_size(go, fs)
}

foreign class Sprite {
    construct new(id) {}
    foreign parent
    foreign as_component
    foreign size
    foreign texture_id
    foreign current_index
    ///Returns in the form [r,g,b,a]
    foreign tint
    tint=(value) { Sprite.set_tint(Lilah.find(this.parent).ref, value) }
    foreign cut_sprite_sheet(i, j)
    foreign static cut_sprite_sheet(go, i, j)
    foreign static set_sort(go, i)
    foreign static set_tint(go, color)
}

foreign class Scene {
    construct new(i) {}
    foreign parent
    foreign as_component
    foreign markers
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

foreign class Rigidbody {
    construct new() {}
    foreign parent
    foreign as_component
    foreign position
    foreign velocity
    foreign solid
    ///returns a map in the form "name": _, "uuid": _ or null if no collision
    foreign colliding
    velocity=(value) { Rigidbody.set_velocity(Lilah.find(this.parent).ref, value) }
    solid=(value) { Rigidbody.set_solid(Lilah.find(this.parent).ref, value) }
    position=(value) { Rigidbody.set_position(Lilah.find(this.parent).ref, value) }
    ///returns a map in the form "name": _, "uuid": _ or null if no collision
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
    foreign parent
    foreign as_component
    foreign playing
    foreign speed
    speed=(value) { Animator.set_speed(Lilah.find(this.parent).ref, value) }
    foreign frame
    frame=(value) { Animator.set_frame(Lilah.find(this.parent).ref, value) }
    foreign play()
    foreign stop()
    ///returns map in the form state: value:Vec2
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
    foreign parent
    #!position(ord = 0)
    position=(value) { Transform.set_position(Lilah.find(this.parent).ref, value) }
    #!scale(ord = 1)
    scale=(value) { Transform.set_scale(Lilah.find(this.parent).ref, value) }
    #!rotation(ord = 2)
    rotation=(value) { Transform.set_rotation(Lilah.find(this.parent).ref, value) }
    #!pivot(ord = 3)
    pivot=(value) { Transform.set_pivot(Lilah.find(this.parent).ref, value) }
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
    foreign static inverse_point(go, point)
}

foreign class GameObject {
    construct new(name) {}
    foreign addComponent(x)
    foreign getComponent(x)
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
    ///Returns a map in the form "name": _, "uuid": _
    foreign id
    foreign uuid
    foreign name
    foreign name=(v)
    foreign components

    toString {
        var result = "%(id), Component Count: %(components.count)"
        return result
    }
}

foreign class Sfx {
    construct new(name, file) {}
    foreign parent
    foreign as_component
    foreign name
    foreign name=(v)
    foreign volume
    volume=(value) { Sfx.set_volume(Lilah.find(this.parent).ref, value) }
    foreign file
    foreign play()
    foreign static get_volume(go, name)
    foreign static set_volume(go, name, amt)
    foreign static play(go, name)
}

foreign class Line {
    construct new() {}
    foreign parent
    foreign as_component
    foreign color
    foreign opacity
    foreign sort
    ///Gets line thickness in form [start, end]
    foreign thickness
    foreign points
    sort=(value) { Line.set_sort(Lilah.find(this.parent).ref, value) }
    color=(value) { Line.set_color(Lilah.find(this.parent).ref, value) }
    opacity=(value) { Line.set_opacity(Lilah.find(this.parent).ref, value) }
    thickness=(value) { Line.set_thickness(Lilah.find(this.parent).ref, value) }
    foreign static set_sort(go, sort)
    foreign static get_sort(go)
    foreign static set_thickness(go, thickness)
    foreign static get_thickness(go)
    foreign static set_color(go, color)
    foreign static set_opacity(go, opacity)
    foreign static add_point(go, point)
    foreign static remove_point(go, index)
    foreign static pop_point(go)
    foreign static insert_point(go, point, index)
    foreign static set_point(go, index, point)
}

foreign class ComponentBehaviour {
    construct new(b) { }
    foreign as_component
    foreign parent
    foreign uuid
}

foreign class Debug {
    foreign static drawLine(start, end, color)
    
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
