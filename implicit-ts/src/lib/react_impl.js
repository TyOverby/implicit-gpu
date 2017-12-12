var impl = require('implicit');

var Impl = {
    createElement: function (type, props, ...children) {
        try {
            return type(props, children);
        } catch (e) {
            if (e.prototype === TypeError) {
                return new type(props, children).render();
            } else {
                throw e
            }
        }
    }
};

function __childless__(f) {
    return function (props, children) {
        if (children && children.length > 0) {
            throw new Error("Element must not have any children.");
        }
        return f(props);
    }
}
function __forward_all__(f) {
    return function (props, children) {
        return f(...children);
    }
}

var Circle = __childless__(props =>
    impl.circle(props.x, props.y, props.r));
var Rect = __childless__(props =>
    impl.rect(props.x, props.y, props.w, props.h));
var And = __forward_all__(impl.and);
var Or = __forward_all__(impl.and);
var Not = __forward_all__(impl.and);
var Break = __forward_all__(impl.and);
var Freeze = __forward_all__(impl.and);

var Translate = (props, children) => {
    impl.translate(props.dx, props.dy, ...children);
};

var Modulate = (props, children) => {
    impl.translate(props.by, ...children);
};
