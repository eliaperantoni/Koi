const {exec} = require('child_process');
const gulp = require('gulp');
const replace = require('gulp-replace');
const htmlmin = require('gulp-htmlmin');
const sass = require('gulp-sass');
const cleanCSS = require('gulp-clean-css');
const imagemin = require('gulp-imagemin');
const liveServer = require("live-server");
const fs = require('fs');
const path = require('path');

function generateSnippets() {
    return exec('python highlighter.py', {cwd: 'snippets'});
}

function html() {
    return gulp.src('index.html')
        .pipe(replace(/<!-- SNIPPET (.+) -->/g, function (match, name) {
            return fs.readFileSync(path.join('snippets/out', `${name}.koi`));
        }))
        .pipe(htmlmin({collapseWhitespace: true}))
        .pipe(gulp.dest('dist/'));
}

function css() {
    return gulp.src('style.scss')
        .pipe(sass().on('error', sass.logError))
        .pipe(cleanCSS())
        .pipe(gulp.dest('dist/'));
}

function assets() {
    return gulp.src('assets/**')
        .pipe(imagemin())
        .pipe(gulp.dest('dist/assets/'));
}

exports.build = gulp.parallel([
    gulp.series(generateSnippets, html),
    css,
    assets,
]);

exports.watch = function () {
    liveServer.start({root: 'dist'});

    gulp.watch(['index.html', 'snippets/*.koi'], {ignoreInitial: false}, gulp.series(generateSnippets, html));
    gulp.watch('style.scss', {ignoreInitial: false}, css);
    gulp.watch('assets/', {ignoreInitial: false}, assets);
};
