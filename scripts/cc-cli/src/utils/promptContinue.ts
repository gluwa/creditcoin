import prompts from "prompts";

export async function promptContinue() {
  const promptResult = await prompts({
    type: "confirm",
    name: "confirm",
    message: "Continue?",
    initial: false,
  });

  if (!promptResult.confirm) {
    process.exit(0);
  }

  return promptResult.confirm;
}

export async function promptContinueOrSkip(prompt: string) {
  const promptResult = await prompts({
    type: "select",
    name: "continue",
    message: prompt,
    choices: [
      { title: "Continue", value: true },
      { title: "Skip", value: false },
    ],
    initial: 1,
  });

  return promptResult.continue;
}
