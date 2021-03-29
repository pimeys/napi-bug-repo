const api = require(`./libnapi_test.so.node`)

async function main() {
    const a = new api.A((s) => console.log(s));
    const b = new api.A((s) => console.log(s));
}
main()
