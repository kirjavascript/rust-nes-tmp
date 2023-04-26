const symbols = require('child_process').execSync('nm -anC ./target/mos-nes-cnrom-none/release/deps/*.elf').toString();
const lines = [];
const usedAddr = {};
symbols.toString().trim().split('\n').forEach(line => {
    const [addr, type, name] = line.split(' ');
    if (true || 'at'.includes(type.toLowerCase())) {
        if (parseInt(addr, 16) < 0x10000 && !usedAddr[addr]) {
            usedAddr[addr] = true;
            lines.push(`LABEL { ADDR $${addr}; NAME ${JSON.stringify(name)}; };`);
        }
    }
});
console.log(lines.join('\n'));
