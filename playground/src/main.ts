import { Interpretter, init } from "./interpretter";
import { OutputQueue } from "./output_queue";

const inputBox = document.getElementById("code")! as HTMLTextAreaElement;
const outputBox = document.getElementById("output-box")! as HTMLDivElement;

function delay(ms: number): Promise<void> {
    return new Promise((res) => {
        setTimeout(res, ms);
    });
}
async function outputText(output: string): Promise<void> {
    outputBox.append(document.createTextNode(output));
    outputBox.append(document.createElement("br"));
    await delay(10);
}
async function run(code: string): Promise<void> {
    while (outputBox.childNodes.length > 0) {
        outputBox.removeChild(outputBox.firstChild!);
    }
    const queue = new OutputQueue();
    const interpretter = new Interpretter(
        () =>
            new Promise((res) => {
                const inputText = document.createElement("input");
                inputText.type = "text";
                outputBox.append(inputText);
                inputText.focus();
                const handler = (event: KeyboardEvent) => {
                    if (event.key === "Enter") {
                        outputBox.removeChild(inputText);
                        res(inputText.value);
                        inputText.removeEventListener("keypress", handler);
                    }
                };
                inputText.addEventListener("keypress", handler);
            }),
        async (output) => {
            for (const line of queue.insert(output)) {
                await outputText(line);
            }
        },
    );
    await interpretter.run(code);
    await outputText(queue.left());
}
init().then(() => {
    const button = document.getElementById("run-button")! as HTMLInputElement;
    button.disabled = false;
    button.addEventListener("click", async () => {
        button.disabled = true;
        await run(inputBox.value);
        button.disabled = false;
    });
});
