// @ts-check
import fs from "fs/promises";
import util from "util";
import { exec as nativeExec } from "child_process";
import path from "path";
import { fileURLToPath } from "url";

const exec = util.promisify(nativeExec);
const WHISPER_PATH = "/Users/dmtrkovalenko/dev/whisper.cpp";
const MEDIA_DIR = path.resolve(fileURLToPath(import.meta.url), "../media");

async function processFile(filePath, i) {
  await exec(
    `ffmpeg -y -i ${filePath} -acodec pcm_s16le -ac 1 -ar 16000 audio.wav`
  );

  const res = await exec(
    `${WHISPER_PATH}/main -m ${WHISPER_PATH}/models/ggml-base.en.bin -f ./audio.wav -ovtt -sow -ml 1 -of ${filePath}.word`
  );

  const res2 = await exec(
    `${WHISPER_PATH}/main -m ${WHISPER_PATH}/models/ggml-base.en.bin -f ./audio.wav -ovtt -of ${filePath}`
  );

  if (res2.stderr) {
    console.log(res.stderr);
  }
}

let [, , version] = process.argv;
console.log(`Processing version ${version} files`);

let dir = await fs.readdir(MEDIA_DIR);

for (const file of dir) {
  if (file.startsWith(`${version}-`) && file.endsWith(".mp3")) {
    console.log(`Processing ${file}`);
    await processFile(path.join(MEDIA_DIR, file));
  }
}
