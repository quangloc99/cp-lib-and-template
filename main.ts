declare namespace process {
    namespace stdin {
        function on(event: 'data', callback: (s: string) => void): void;
        function on(event: 'end', callback: () => void): void;
    }
};

const inp: string[] = [];
const out: string[] = [];
let lines: string[];
let lineCounter = 0;
const readline = () => lines[lineCounter++];
const writeline = (line: string) => out.push(line);
process.stdin.on('data', c => inp.push(c));
process.stdin.on('end', () => {
    lines = inp.join('').split('\n');
    myMain();
    console.log(out.join('\n'));
});

function myMain() {
}
