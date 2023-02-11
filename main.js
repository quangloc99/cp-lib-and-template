;
var inp = [];
var lines;
var lineCounter = 0;
var readline = function () { return lines[lineCounter++]; };
process.stdin.on('data', function (c) { return inp.push(c); });
process.stdin.on('end', function () {
    lines = inp.join('').split('\n');
    myMain();
});
function myMain() {
    console.log(readline());
}
