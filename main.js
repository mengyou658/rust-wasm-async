const a = require("./pkg");
a.test(console.log).then(res => {
    console.log('res 1', res)
});
a.test2(console.log).then(res => {
    console.log('res 2', res)
});
