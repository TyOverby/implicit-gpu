onmessage = async ev => {
    try {
        var implicit_js = await (await fetch('./implicit.js')).text();

        var exports = {};

        var require = s => {
            if (s === 'implicit') {
                return eval(implicit_js);
            } else {
                throw new Error("could not require " + s);
            }
        };
        console.log(ev.data);
        eval(ev.data);
        postMessage({
            status: 'ok',
            exports: exports,
        });
    }
    catch (e) {
        var stack_string = e.stack;
        console.log(stack_string);
        var r = /at.*:([0-9]+):([0-9]+)/;
        var matches = r.exec(stack_string);

        postMessage({
            status: 'err',
            error: {
                line_num: matches[1],
                col_num: matches[2],
                message: e.message
            }
        });
    }
};
