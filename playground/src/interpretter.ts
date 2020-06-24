import upperInit, {
    Interpretter as GenInterpretter,
    Output,
    OutputStatus,
} from "stringed-wasm-core";

export class Interpretter {
    constructor(
        private input: () => Promise<string>,
        private output: (output: string) => Promise<void>,
    ) {}
    async run(code: string): Promise<void> {
        const interpretter = GenInterpretter.start(code);
        let result: null | Output = null;
        while (true) {
            let input: null | string = null;
            if (result) {
                switch (result.status()) {
                    case OutputStatus.Output:
                        await this.output(result.value()!);
                        break;
                    case OutputStatus.Input:
                        input = await this.input();
                        result.free();
                        break;
                    case OutputStatus.Error:
                        interpretter.free();
                        throw new Error(result.value()!);
                    case OutputStatus.Done:
                        result.free();
                        interpretter.free();
                        return;
                }
            }
            result = interpretter.next(input ?? undefined);
        }
    }
}
export async function init(): Promise<void> {
    await upperInit(fetch("./interpretter.wasm"));
}
