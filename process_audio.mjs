// @ts-check
import fs from "fs/promises";
import util from "util";
import { exec as nativeExec } from "child_process";
import audioBase from "./baseJson.json" assert { type: "json" };

const exec = util.promisify(nativeExec);

async function processFile(filePath, i) {
  await exec(`stable-ts ${filePath} -o audio.json -y`);
  let jsonContent = await fs.readFile("audio.json", "utf-8");
  /** @type {typeof audioBase} */
  let audio = JSON.parse(jsonContent);

  let whole_word_timestamps = audio.segments.flatMap(
    (segment) => segment.whole_word_timestamps
  );

  let processed_words_with_timestamps = whole_word_timestamps.map((word, i) => {
    return {
      word: word.word,
      end: word.timestamp,
      start:
        whole_word_timestamps[i - 1]?.timestamp ??
        audio.segments[0].alt_start_timestamps?.[0] ??
        0,
    };
  });

  await fs.writeFile(
    `words-${i}.json`,
    JSON.stringify(processed_words_with_timestamps, null, 2)
  );
}

let [, , ...files] = process.argv;

for (let i = 0; i < files.length; i++) {
  await processFile(files[i], i);
}
