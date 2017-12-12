var CopyWebpackPlugin = require('copy-webpack-plugin');

module.exports = {
    entry: {
        implicit: "./src/lib/implicit.ts",
        bundle: "./src/scripts/index.tsx"
    },
    output: {
        filename: "[name].js",
        path: __dirname + "/dist"
    },

    // Enable sourcemaps for debugging webpack's output.
    devtool: "source-map",

    resolve: {
        // Add '.ts' and '.tsx' as resolvable extensions.
        extensions: [".ts", ".tsx", ".js", ".json"]
    },

    plugins: [
        new CopyWebpackPlugin([
            { from: "./src/index.html" },
            { from: "./src/scripts/runworker.js" },
            { from: "./src/lib/", to: 'lib'},
            { from: "./src/styles/", to: "styles" },
            { from: "./node_modules/react/dist/react.js", to: "deps" },
            { from: "./node_modules/react-dom/dist/react-dom.js", to: "deps" },
            { from: "./res/", to: "res" },
            { from: 'node_modules/monaco-editor', to: 'monaco-editor'}
        ])
    ],

    module: {
        rules: [
            // All files with a '.ts' or '.tsx' extension will be handled by 'awesome-typescript-loader'.
            { test: /\.tsx?$/, loader: "awesome-typescript-loader" },

            // All output '.js' files will have any sourcemaps re-processed by 'source-map-loader'.
            { enforce: "pre", test: /\.js$/, loader: "source-map-loader" }
        ]
    },

    // When importing a module whose path matches one of the following, just
    // assume a corresponding global variable exists and use that instead.
    // This is important because it allows us to avoid bundling all of our
    // dependencies, which allows browsers to cache those libraries between builds.
    externals: {
        "react": "React",
        "react-dom": "ReactDOM"
    },
};
