import "math" for Vec2

class GameObjectRef {
    ref { Lilah.gameobjects[_ref] }
    data {
        if(_ref == null || Lilah.data == null || !Lilah.data.containsKey(ref.uuid)) {
            return null
        } 
        return Lilah.data[ref.uuid] 
    }

    [key] {
        if(!data.containsKey(key)) {
            return null
        }
        return data[key]
    }

    construct new(i) {
        _ref = i
    }
}

class Audio {
    static music { __music }
    static command { __command }
    static dirty { __dirty }
    static volume { __volume }
    static fade { __fade }
    static volume=(v) { 
        __dirty = true
        __volume = v 
    }

    static play(file) {
        __command = "start"
        __music = file
        __dirty = true
    }

    static play(file, fade_in_ms) {
        __command = "start_fade"
        __music = file
        __fade = fade_in_ms
        __dirty = true
    }

    static play() {
        __command = "play"
        __dirty = true
    }

    static pause() {
        __command = "pause"
        __dirty = true
    }

    static pause(fade_out_ms) {
        __command = "pause_fade"
        __fade = fade_out_ms
        __dirty = true
    }

    static clear() {
        __dirty = false
    }
}

class Lilah {
    static camera { 
        if(__camera == null) {
            __camera = Lilah.find("Camera")
        }
        return __camera 
    }
    static destroy { __destroy }

    static gameobjects { __gameobjects }
    static gameobjects=(v) { __gameobjects=v }

    static data { __data }
    static data=(v) { __data=v }

    static delta_time { __delta_time }
    static delta_time=(v) { __delta_time = v }

    static fullscreen { __fullscreen }
    static fullscreen=(v) { __fullscreen = v }

    static screen_size { __screen_size }
    static screen_size=(v) { __screen_size = v }

    static tick_fibers() {
        if(__fibers == null) return

        for(f in __fibers) {
            f["delay"] = f["delay"]-Lilah.delta_time
            if(f["delay"] < 0) {
                f["delay"] = f["fiber"].call()
            }
        }

        __fibers = __fibers.where {|i| i["delay"] != null }.toList
    }

    static start_fiber(f) {
        if(__fibers == null) {
            __fibers = []
        }

        __fibers.add({"fiber": f, "delay": f.call()})
    }

    static instantiate(go, d) {
        if(__gameobjects == null) {
            __gameobjects = []
        }

        if(__data == null) {
            __data = {}
        }

        __gameobjects.add(go)
        __data[go.uuid] = d
        return GameObjectRef.new(__gameobjects.count-1)
    }

    static instantiate(go) {
        if(__gameobjects == null) {
            __gameobjects = []
        }

        if(__data == null) {
            __data = {}
        }

        __gameobjects.add(go)
        __data[go.uuid] = {}
        return GameObjectRef.new(__gameobjects.count-1)
    }

    static clear() {
        __destroy = []
        Audio.clear()
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
        if(__gameobjects == null) return null 
        for(i in (0..__gameobjects.count)) {
            var id = __gameobjects[i].id
            if(id["uuid"] == key || id["name"] == key) {
                return GameObjectRef.new(i)
            }
        }
        return null
    }

    static to_screen_space(input) {
        return Vec2.new(input.x+camera.ref.get("Transform").position.x, (-input.y-screen_size.y)+camera.ref.get("Transform").position.y)
    }

    static to_world_space(input) {
        return Vec2.new(input.x-camera.ref.get("Transform").position.x, (-input.y+screen_size.y)-camera.ref.get("Transform").position.y)
    }
}

class Input {
    static mouse_pos { __mouse_pos }

    //private
    static is_pressed(key) {
        if (!__mappings.containsKey(key)) {
            return false
        }
        return __mappings[key]["pressed_down"]
    }
    //private
    static is_mouse_pressed(key) {
        if (!__mouse_mappings.containsKey(key)) {
            return false
        }
        return __mouse_mappings[key]["pressed_down"]
    }

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
        if(__bindings == null || __mappings == null) {
            return 0
        }   

        var x = 0

        if(__bindings.containsKey(bind)) {
            if(__mappings.containsKey(__bindings[bind][0]) && __mappings[__bindings[bind][0]]["pressed"]) {
                x = -1
            } else if( __mappings.containsKey(__bindings[bind][1]) && __mappings[__bindings[bind][1]]["pressed"]) {
                x = 1
            }
        }

        return x
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

class UI {
    static on_click_callbacks { 
        if(__on_click_callbacks == null) {
            __on_click_callbacks = []
        }
        return __on_click_callbacks 
    }

    static on_click_down_callbacks { 
        if(__on_click_down_callbacks == null) {
            __on_click_down_callbacks = []
        }
        return __on_click_down_callbacks 
    }

    static on_hover_callbacks { 
        if(__on_hover_callbacks == null) {
            __on_hover_callbacks = []
        }
        return __on_hover_callbacks
    }

    static on_click(gameobject, callback) {
        __on_click_callbacks.add({"gameobject":gameobject, "callback":callback})
    }

    static on_click_down(gameobject, callback) {
        __on_click_down_callbacks.add({"gameobject":gameobject, "callback":callback})
    }

    static on_hover(gameobject, callback) {
        __on_hover_callbacks.add({"gameobject":gameobject, "callback":callback})
    }

    static tick() {
        if(!Input.mouse_pos) return 
        var mouse = Lilah.to_world_space(Input.mouse_pos)

        for(i in on_click_callbacks) {
            var i_pos = i["gameobject"].ref.get("Transform").position
            var i_size = i["gameobject"].ref.get("Sprite").size
            
            if(mouse.x > i_pos.x && mouse.x < i_pos.x+i_size.x) {
                if(mouse.y > i_pos.y && mouse.y < i_pos.y-i_size.y) {
                    if(Input.mouse("Left")) {
                        i["callback"].call()
                    }
                }
            }
        }

        for(i in on_click_down_callbacks) {
            var i_pos = i["gameobject"].ref.get("Transform").position
            var i_size = i["gameobject"].ref.get("Sprite").size
            
            if(mouse.x > i_pos.x && mouse.x < i_pos.x+i_size.x) {
                if(mouse.y > i_pos.y && mouse.y < i_pos.y-i_size.y) {
                    if(Input.mouse_down("Left")) {
                        i["callback"].call()
                    }
                }
            }
        }

        for(i in on_hover_callbacks) {
            var i_pos = i["gameobject"].ref.get("Transform").position
            var i_size = i["gameobject"].ref.get("Sprite").size
            
            if(mouse.x > i_pos.x && mouse.x < i_pos.x+i_size.x) {
                if(mouse.y > i_pos.y && mouse.y < i_pos.y-i_size.y) {
                    i["callback"].call()
                }
            }
        }
    }
}