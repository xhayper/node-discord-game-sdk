const Discord = require("./index.js");

(async () => {
    const client = new Discord.Client("310270644849737729");

    const connect = client.connect();

    console.log(connect);
    await connect;

    console.log(client);
    console.log(`Logged in as ${client.user?.username}#${client.user?.discriminator} (${client.user?.id})`);
})();
