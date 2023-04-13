const a = require("./pkg");
a.test(console.log).then(res => {
    console.log('1')
});
a.test2(console.log).then(res => {
    console.log('2')
});
