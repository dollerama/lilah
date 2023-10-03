import "math" for Vec2

class GameObjectRef {
    ref { State.gameobjects[_ref] }
    data { State.data[_ref] }

    construct new(i) {
        _ref = i
    }
}

class State {
    static destroy { __destroy }

    static gameobjects { __gameobjects }
    static gameobjects=(v) { __gameobjects=v }
    static data { __data }
    static data=(v) { __data=v }
    
    static textures { __textures }

    static delta_time { __delta_time }
    static delta_time=(v) { __delta_time = v }

    static load_texture(file) {
        if(__textures == null) {
            __textures = []
        }

        __textures.add(file)
    }

    static instantiate(go, d) {
        if(__gameobjects == null) {
            __gameobjects = []
        }

        if(__data == null) {
            __data = []
        }

        __gameobjects.add(go)
        __data.add(d)
        return GameObjectRef.new(__gameobjects.count-1)
    }

    static instantiate(go) {
        if(__gameobjects == null) {
            __gameobjects = []
        }

        if(__data == null) {
            __data = []
        }

        __gameobjects.add(go)
        __data.add({})
        return GameObjectRef.new(__gameobjects.count-1)
    }

    static load() {
        __textures = []
        __destroy = []
    }

    static destroy(key) {
        if(__destroy == null) {
            __destroy = []
        }

        var d = null
        var j = 0
        for(i in (0..__gameobjects.count)) {
            var id = __gameobjects[i].id
            if(id["uuid"] == key || id["name"] == key) {
                d = GameObjectRef.new(i)
                j=i
                break
            }
        }

        if(d != null) {
            __destroy.add(d.ref)
            __gameobjects.removeAt(j)
        }
    }

    static find(key) {
        for(i in (0..__gameobjects.count)) {
            var id = __gameobjects[i].id
            if(id["uuid"] == key || id["name"] == key) {
                return GameObjectRef.new(i)
            }
        }
        return null
    }
}

class Input {
    static mouse_pos { __mouse_pos }

    static mappings { 
        if(__mappings == null) {
            __mappings = {}
        }
        return __mappings
    }
    static mouse_mappings { 
        if(__mouse_mappings == null) {
            __mouse_mappings = {}
        }
        return __mouse_mappings
    }
    static bindings { 
        if(__bindings == null) {
            __bindings = {}
        }
        __bindings 
    }

    static set_mouse_pos(pos) {
        __mouse_pos = Vec2.new(pos.x, pos.y)
    }

    static get_mapping(key) {
        return __mappings[key]
    }

    static update_mapping(key, pressed, pressed_down) {
        if(__mappings == null) {
            __mappings = {}
        }
        __mappings[key] = {"pressed": pressed, "pressed_down": pressed_down}
    }

    static update_mouse_mapping(button, pressed, pressed_down) {
        if(__mouse_mappings == null) {
            __mouse_mappings = {}
        }
        __mouse_mappings[button] = {"pressed": pressed, "pressed_down": pressed_down}
    }

    static update_binding(bind, neg, pos) {
        if(__bindings == null) {
            __bindings = {}
        }
        __bindings[bind] = [neg, pos]
    }

    static key(key) {
        if(__mappings == null) {
            __mappings = {}
        }
        if(!__mappings.containsKey(key)) return false
        return __mappings[key]["pressed"]
    }

    static mouse(button) {
        if(__mouse_mappings == null) {
            __mouse_mappings = {}
        }
        if(!__mouse_mappings.containsKey(button)) return false
        return __mouse_mappings[button]["pressed"]
    }

    static key_down(key) {
        if(__mappings == null) {
            __mappings = {}
        }

        if(!__mappings.containsKey(key)) return false

        if(__mappings[key]["pressed_down"]) {
            __mappings[key]["pressed_down"] = false
            return true
        } else {
            return false
        }
    }

    static mouse_down(button) {
        if(__mouse_mappings == null) {
            __mouse_mappings = {}
        }

        if(!__mouse_mappings.containsKey(button)) return false

        if(__mouse_mappings[button]["pressed_down"]) {
            __mouse_mappings[button]["pressed_down"] = false
            return true
        } else {
            return false
        }
    }

    static binding(bind) {
        if(__bindings == null) {
            return 0
        }   

        var val = 0

        if(!__bindings.containsKey(bind)) {
            return 0
        }

        if(__mappings.containsKey(__bindings[bind][0]) && __mappings[__bindings[bind][0]]["pressed"]) {
            val = -1
        } else if(__mappings.containsKey(__bindings[bind][1]) && __mappings[__bindings[bind][1]]["pressed"]) {
            val = 1
        }

        return val
    }

    static binding2D(bind1, bind2) {
        if(__bindings == null || __mappings == null) {
            return Vec2.new(0,0)
        }   

        var x = 0
        var y = 0

        if(__bindings.containsKey(bind1)) {
            if(__mappings.containsKey(__bindings[bind1][0]) && __mappings[__bindings[bind1][0]]["pressed"]) {
                x = -1
            } else if( __mappings.containsKey(__bindings[bind1][1]) && __mappings[__bindings[bind1][1]]["pressed"]) {
                x = 1
            }
        }

        if(__bindings.containsKey(bind2)) {
            if(__mappings.containsKey(__bindings[bind2][0]) && __mappings[__bindings[bind2][0]]["pressed"]) {
                y = -1
            } else if( __mappings.containsKey(__bindings[bind2][1]) && __mappings[__bindings[bind2][1]]["pressed"]) {
                y = 1
            }
        }

        return Vec2.new(x,y)
    }
}