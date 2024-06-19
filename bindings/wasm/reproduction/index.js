const { Client } = require("@iota/sdk-wasm/node");

(async () => {

  const client = new Client({
    primaryNode: "https://api.testnet.shimmer.network/",
    localPow: true,
  });
  
  const result = await client.getOutput("0x9e190f35188fa7986205a8fed2dd69554d7a3218805709ee04cc73a87bc9cd090000");

  console.log(
    result
  );
})();