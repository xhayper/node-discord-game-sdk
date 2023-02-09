const Discord = require("./index.js");

(async () => {
    const client = new Discord.Client("310270644849737729");

    // console.log(client.user?.id);

    const connect = client.connect();

    console.log(connect);
    await connect;

    console.log(client.user?.id);

    console.log(client);
})();
