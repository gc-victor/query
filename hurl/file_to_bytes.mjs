import { readFile, writeFileSync } from "node:fs";

if (process.argv.length < 4) {
    console.error("Please provide input and output file paths");
    process.exit(1);
}

readFile(process.argv[2], (err, data) => {
    if (err) console.error("Error reading file:", err);
    writeFileSync(process.argv[3], new Uint8Array(data).toString());
});
