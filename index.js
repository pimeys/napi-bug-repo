const api = require(`./libnapi_test.so.node`)

async function main() {
    while (true) {
        const q = `query {
            findFirstBooking(where: { id: "ckovh15xa104945sj64rdk8oas" }) {
                id
                name
                forename
                description
                email
                phone
                arrivalDate
                departureDate
                price
                advance
                advanceDueDate
                kids
                adults
                status
                nourishment
                createdAt
                room {
                id
                name
                }
            }
        }`;

        let query = JSON.stringify({
            query: q,
            variables: {}
        });

        let res = await api.test(query);
        console.log(JSON.parse(res));
    }
}
main()
