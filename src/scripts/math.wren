import "io" for Serializable
import "random" for Random

class Util {
    static lerp(a, b, t) {
        return (a + (b - a) * t)
    }
}

foreign class Vec2 is Serializable {
    construct new(x, y) {}
 
    #!x(ord = 0)
    foreign x
    #!y(ord = 1)
    foreign y
    foreign x=(x)
    foreign y=(y)

    static default { Vec2.new(0, 0) }

    getProperty() {
        return properties([this.x, this.y])
    }

    setProperty() {
        return properties { |v| 
            this.x = v.call()
            this.y = v.call()
        }
    }

    foreign static one
    foreign static zero
    foreign static up
    foreign static down
    foreign static left
    foreign static right

    foreign magnitude()
    foreign magnitude_sqr()
    foreign normalized()
    foreign normalize()

    foreign static cross(a, b)
    foreign static dot(a, b)
    foreign static lerp(a, b, t)
    foreign static screen_to_world_space(pos)
    foreign static world_to_screen_space(pos)

    +(other) {
        other.x = x+other.x
        other.y = y+other.y
        return other
    }

    -(other) {
        other.x = x-other.x
        other.y = y-other.y
        return other
    }

    *(other) {
        var o = Vec2.new(other, other)
        o.x = x*o.x
        o.y = y*o.y
        return o
    }

    /(other) {
        var o = Vec2.new(other, other)
        o.x = x/o.x
        o.y = y/o.y
        return o
    }

    - {
        return other*-1
    }

    ==(other) {
        return (x == other.x && y == other.y)
    }

    toString {
        return "Vec2(%(this.x), %(this.y))"
    }
}