import readline from "readline";

function askQuestion(query: string): Promise<string> {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  return new Promise<string>((resolve) =>
    rl.question(query, (ans) => {
      rl.close();
      resolve(ans);
    })
  );
}

export async function promptContinue() {
  const answer = await askQuestion("Continue? (y/n): ");
  if (answer.toLowerCase() !== "y") {
    console.log("Aborting...");
    process.exit(0);
  }
}

export async function promptContinueOrSkip(prompt: string) {
  let answer = await askQuestion(`${prompt} (y/skip): `);
  while (answer !== "y" && answer !== "skip") {
    console.log("Invalid input");
    answer = await askQuestion(`${prompt} (y/skip): `);
  }
  if (answer === "skip") {
    return false;
  } else if (answer === "y") {
    return true;
  }
}
