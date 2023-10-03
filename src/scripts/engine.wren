import "math" for Vec2
import "app" for State

class Behaviour {
    frame {
        if(_frame == null) {
            _frame = 0
        }
        return _frame
    }
    frame=(v) {_frame=v}
    behaviour { _behaviour }

    construct new(c) {
        _behaviour = ComponentBehaviour.new("%(c)").as_component
    }

    static start(id) {}
    static update(id) {}

    start() {}
    update() {}
}

foreign class Component {}

foreign class Sprite {
    construct new(i) {}
    foreign as_component
    foreign size
    foreign texture_id
    foreign current_index
    foreign cut_sprite_sheet(i, j)

    foreign static cut_sprite_sheet(go, i, j)
}

foreign class Rigidbody {
    construct new() {}
    foreign as_component
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

foreign class Transform {
    construct new(p) {}

    foreign as_component
    
    foreign position
    foreign scale
    foreign rotation
    foreign position=(value)
    foreign scale=(value)
    foreign rotation=(value)

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
    
    foreign add_component(x)
    foreign get_component(x)
    foreign set_component(x, y)
    foreign id
    foreign uuid
    foreign name
    foreign name=(v)
}

foreign class ComponentBehaviour {
    construct new(b) { }
    foreign as_component
}