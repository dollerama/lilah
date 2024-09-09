import "math" for Vec2, Util
import "random" for Random

class GameObjectRef {
    static create_ref(id) {
        return GameObjectRef.new(id)
    }

    ref {
        return Lilah.gameobjects[_ref] 
    }

    behaviourData(b) {
        if(b.supertype[ref.uuid]["%(b)"].count == 1) {
            return b.supertype[ref.uuid]["%(b)"].values.toList[0]
        } else if(b.supertype[ref.uuid]["%(b)"].count > 1) {
            return b.supertype[ref.uuid]["%(b)"].values.toList
        }
    }

    behaviourData(b, uuid) {
        if(uuid is String) {
            return b.supertype[ref.uuid]["%(b)"][uuid]
        } else {
            return null
        }
    }

    behaviourData(b, u, mut) {
        mut.call(b.supertype[ref.uuid]["%(b)"][u])
    }

    data=(v) {
       Lilah.data[ref.uuid] = v 
    }

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

    [key]=(v) {
        data[key]=v
    } 

    construct new(i) {
        _ref = i
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
    static destroy_internal { __destroy_int }

    static gameobjects { __gameobjects }
    static gameobjects=(v) { __gameobjects=v }
    static gameobjects_values { __gameobjects.values.toList }

    static data { __data }
    static data=(v) { __data=v }

    static delta_time { __delta_time }
    static delta_time=(v) { __delta_time = v }

    static time { __time }
    static time=(v) { __time = v }

    static fps { __fps }
    static fps=(v) { __fps = v }

    static fullscreen { __fullscreen }
    static fullscreen=(v) { __fullscreen = v }

    static screen_size { __screen_size }
    static screen_size=(v) { __screen_size = v }

    static fiberCount {
        if(__fibers == null) {
            return 0
        } else {
            return __fibers.count
        }
    }

    static tick_fibers() {
        if(Tween.tweens != null) {
            for(t in Tween.tweens) {
                var val = t[1].call()
                if(val != null) {
                    t[0].call(val)
                }
            }

            Tween.tweens = Tween.tweens.where {|i| !i[1].isDone }.toList
        }

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
            __gameobjects = {}
        }

        if(__data == null) {
            __data = {}
        }

        __gameobjects[go.uuid] = go
        __data[go.uuid] = d
        return GameObjectRef.new(go.uuid)
    }

    static instantiate(go) {
        if(__gameobjects == null) {
            __gameobjects = {} 
        }

        if(__data == null) {
            __data = {}
        }

        __gameobjects[go.uuid] = go
        __data[go.uuid] = {}
        return GameObjectRef.new(go.uuid)
    }

    static clear() {
        __destroy = []
        Audio.clear()
    }

    static destroy(key) {
        if(__destroy == null) {
            __destroy = []
        }
        var j = null
        if(key is GameObjectRef) {
            j = key.ref.uuid
        } else if(key is String) {
            j = key
        }

        if(j != null) {
            __destroy.add(j)
            __data.remove(j)
            __gameobjects.remove(j)
        }
    }

    static find(key) {
        if(__gameobjects == null) return null 
        
        if(__gameobjects[key] != null) {
          return GameObjectRef.new(key)
        } else {
          for(i in __gameobjects) {
              var id = i.value.id
              if(id["uuid"] == key || id["name"] == key) {
                  return GameObjectRef.new(id["uuid"])
              }
          }
        }
        return null
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

class KeycodeLookup {
    construct new() {}

    W { "W" }
    A { "A" }
    S { "S" }
    D { "D" }
    Up { "Up" }
    Right { "Right" }
    Down { "Down" }
    Left { "Left" }
}

class Input {
    static mouse_pos { __mouse_pos }
    static Keycode {
        if(__keycodelookup == null) {
            __keycodelookup = KeycodeLookup.new()
        }
        return __keycodelookup
    }

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
        var mouse = Vec2.screen_to_world_space(Input.mouse_pos)

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

class Curve {
    static linear {
        return Fn.new { |x|
            return x
        }
    }

    static inQuad {
        return Fn.new { |x|
            return x * x
        }
    }

    static outQuad {
        return Fn.new { |x|
            return 1 - (1 - x) * (1 - x)
        }
    }

    static inOutQuad {
        return Fn.new { |x|
            if(x < 0.5) {
                return 2 * x * x
            } else {
                return 1 - (-2 * x + 2).pow(2) / 2
            }
        }
    }

    static inQuart {
        return Fn.new { |x|
            return x * x * x * x
        }
    }

    static outQuart {
        return Fn.new { |x|
            return 1 - (1 - x).pow(4)
        }
    }

    static inOutQuart {
        return Fn.new { |x|
            if(x < 0.5) {
                return 8 * x * x * x * x
            } else {
                return 1 - (-2 * x + 2).pow(4) / 2
            }
        }
    }

    static inBack {
        return Fn.new { |x|
            var c1 = 1.70158
            var c3 = c1 + 1
            return c3 * x * x * x - c1 * x * x
        }
    }

    static outBack {
        return Fn.new { |x|
            var c1 = 1.70158
            var c3 = c1 + 1

            return 1 + c3 * (x - 1).pow(3) + c1 * (x - 1).pow(2)
        }
    }

    static inOutBack {
        return Fn.new { |x|
            var c1 = 1.70158
            var c2 = c1 * 1.525

            if(x < 0.5) {
                return ((2 * x).pow(2) * ((c2 + 1) * 2 * x - c2)) / 2
            } else {
                return ((2 * x - 2).pow(2) * ((c2 + 1) * (x * 2 - 2) + c2) + 2) / 2
            }
        }
    }

    static inElastic {
        return Fn.new { |x|
            var c4 = (2 * Num.pi) / 3

            if(x == 0) {
                return 0
            } else if(x == 1) {
                return 1
            } else {
                return -(2).pow(10 * x - 10) * ((x * 10 - 10.75) * c4).sin
            }
        }
    }

    static outElastic {
        return Fn.new { |x|
            var c4 = (2 * Num.pi) / 3

            if(x == 0) {
                return 0
            } else if(x == 1) {
                return 1
            } else {
                return (2).pow(-10 * x) * ((x * 10 - 0.75) * c4).sin + 1
            }
        }
    }

    static inOutElastic {
        return Fn.new { |x|
            var c5 = (2 * Num.pi) / 4.5

            if(x == 0) {
                return 0
            } else if(x == 1) {
                return 1
            } else if(x < 0.5) {
                return -(2.pow(20 * x - 10) * ((20 * x - 11.125) * c5).sin) / 2
            } else {
                return (2.pow(-20 * x + 10) * ((20 * x - 11.125) * c5).sin) / 2 + 1
            }
        }
    }
}

class Tween {
    static tweens { __tweens }
    static tweens=(v) { __tweens = v }

    static tweenCount {
        if(__tweens == null) {
            return 0
        } else {
            return __tweens.count
        }
    }

    static insert_tween(t) {
        if(__tweens == null) {
            __tweens = []
        }

        __tweens.add(t)
    }

    duration { _duration }
    use_curve { _use_curve }
    from { _from }
    to { _to } 
    on_complete { _on_complete } 
    duration=(v) { _duration = v}
    use_curve=(v) { _use_curve = v }
    from=(v) { _from = v }
    to=(v) { _to = v } 
    on_complete=(v) { _on_complete = v } 
    
    construct new(f, t) {
        from = f
        to = t
        use_curve = Curve.linear
        duration = 1
    }

    toString { "Tween{from: %(from), to: %(to), curve: %(use_curve), duration: %(duration)}" }

    time(t) {
        duration = t
        return this
    }

    curve(c) {
        use_curve = c
        return this
    }

    onComplete(c) {
        on_complete = c
        return this
    }

    play(c) {
        var f = Fiber.new {
            var t = 0
            while(t < duration) {
                t = t + Lilah.delta_time
                if(from is Vec2 && to is Vec2) {
                    Fiber.yield(Vec2.lerp(from, to, use_curve.call(t/duration)))
                } else if(from is Num && to is Num) {
                    Fiber.yield(Util.lerp(from, to, use_curve.call(t/duration)))
                }
            }

            if(on_complete != null) {
                on_complete.call()
            }
        }

        Tween.insert_tween([c, f])
    }
}
