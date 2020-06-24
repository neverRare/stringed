export class OutputQueue {
    private queue = "";
    insert(output: string): string[] {
        let { queue } = this;
        queue += output;
        let arr: string[] = [];
        let i = 0;
        while (true) {
            let pos = queue.indexOf("\n", i);
            if (pos < 0) break;
            pos++;
            arr.push(queue.slice(i, pos).trimEnd());
            i = pos;
        }
        this.queue = queue.slice(i);
        return arr;
    }
    left(): string {
        let { queue } = this;
        this.queue = "";
        return queue;
    }
}
