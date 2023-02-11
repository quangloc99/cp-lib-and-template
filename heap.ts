class Heap<T> {
    readonly data = [];
    
    constructor(readonly cmp: (u: T, v: T) => number) {}
    
    get length() {
        return this.data.length;
    }
    
    private swap(u: number, v: number) {
        const tmp = this.data[u];
        this.data[u] = this.data[v];
        this.data[v] = tmp;
    }
    
    push(x: T) {
        this.data.push(x);
        let id = this.data.length - 1;
        while (id > 0) {
            const par = ((id - 1) / 2) >> 0;
            if (this.cmp(this.data[par], this.data[id]) >= 0) break;
            this.swap(id, par);
            id = par;
        }
    }
    
    get top() {
        return this.data[0];
    }
    
    pop(): T | undefined {
        if (this.length === 0) return undefined;
        const res = this.data[0];
        this.data[0] = this.data[this.length - 1];
        this.data.pop();
        let id = 0;
        while (id < this.length) {
            const left = id * 2 + 1;
            const right = id * 2 + 2;
            let target = id;
            if (left < this.length && this.cmp(this.data[left], this.data[target]) < 0) target = left;
            if (right < this.length && this.cmp(this.data[right], this.data[target]) < 0) target = right;
            if (target === id) break;
            this.swap(id, target);
            id = target;
        }
        return res;
    }
}
