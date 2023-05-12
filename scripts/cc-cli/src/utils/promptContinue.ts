const readline = require('readline');

function askQuestion(query: string): Promise<string> {
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout,
    });

    //@ts-ignore
    return new Promise<string>(resolve => rl.question(query, ans => {
        rl.close();
        resolve(ans);
    }))
}

export async function promptContinue() {
    const answer = await askQuestion("Continue? (y/n): ");
    if (answer.toLowerCase() !== "y") {
        console.log("Aborting...");
        process.exit(0);
    }
}
