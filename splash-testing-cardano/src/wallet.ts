import { Lucid } from '@lucid-evolution/lucid' ;

export async function setupWallet(lucid: Lucid) {
  const seed = await Deno.readTextFile('./seed.txt');
  lucid.selectWallet.fromSeed(seed);
}

