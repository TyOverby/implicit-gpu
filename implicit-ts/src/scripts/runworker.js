onmessage = async ev => {
    try {
        const [implicit_js, react_impl] = await Promise.all([
            fetch('./implicit.js').then(f => f.text()),
            fetch('./lib/react_impl.js').then(f => f.text()),
        ]);

        var modules = {};

        var require = s => {
            if (modules[s]) {
                return modules[s];
            }

            var ret;
            if (s === 'implicit') {
                ret = eval(implicit_js);
            } else {
                throw new Error("could not require " + s);
            }

            modules[s] = ret;
            return ret;
        };

        (function () {
            var exports = {};
            eval(react_impl);
            eval(ev.data);
            postMessage({
                status: 'ok',
                exports: exports,
            });
        })();
    }
    catch (e) {
        var stack_string = e.stack;
        var r = /at.*:([0-9]+):([0-9]+)/;
        var matches = r.exec(stack_string);
        if (matches.length >= 3) {
            postMessage({
                status: 'err',
                error: {
                    line_num: matches[1],
                    col_num: matches[2],
                    message: e.message
                }
            });
        } else {
            console.error(e);
        }
    }
};
