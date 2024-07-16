import "io" for Serializable

foreign class Vec2 is Serializable {
    construct new(x, y) {}
 
    #!x(ord = 0)
    foreign x
    #!y(ord = 1)
    foreign y
    foreign x=(x)
    foreign y=(y)

    serialize() {
        return super.serialize([this.x, this.y])
    }

    deserialize(obj) {
        return super.deserialize(obj, [Fn.new {|v| this.x = v}, Fn.new {|v| this.y = v}])
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