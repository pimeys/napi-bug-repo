const api = require(`./libnapi_test.so.node`)

async function main() {
    let engine = new api.Engine((_err, log) => {
        console.log(log);
    });

    for (let i = 0; i < 10000; i++) {
        await engine.call("call_no_" + i);
    }

    await engine.off();
}
main()
