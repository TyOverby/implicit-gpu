import { Error } from './types/error';

let availableWorker: Worker | null = null;

type WorkerMessage =
    { status: 'ok', exports: any } |
    { status: 'err', error: Error }

export async function getResult(source: string): Promise<WorkerMessage> {
    let resolve: (s: WorkerMessage) => void;

    const promise = new Promise<WorkerMessage>((ok, fail) => {
        resolve = ok;
    });

    let worker: Worker;
    if (availableWorker !== null) {
        worker = availableWorker;
        availableWorker = null;
    } else {
        worker = new Worker("./runworker.js");
    }

    worker.postMessage(source);

    worker.onmessage = ev => {
        let message: WorkerMessage = ev.data;
        resolve(message);
        availableWorker = worker;
    };

    worker.onerror = ev => {
        resolve({
            status: 'err',
            error: {
                line_num: ev.lineno,
                col_num: ev.colno,
                message: ev.message,
            }
        })
    };

    return promise;
}

async function getFromServer(jsonProgram: string): Promise<string> {
    let res = await fetch("/api/process", {
        method: "POST",
        body: JSON.stringify(jsonProgram)
    });
    return res.text();
}
