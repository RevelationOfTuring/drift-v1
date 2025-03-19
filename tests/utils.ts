import { AnchorError, BN, web3 } from "@coral-xyz/anchor";
import { expect } from "chai";

function requireBNEq(a: BN, b: BN) {
    expect(a.toString()).eq(b.toString());
}

function requirePublickeyEq(a: web3.PublicKey, b: web3.PublicKey) {
    expect(a.toBase58()).eq(b.toBase58());
}

async function requireCustomError(p: Promise<any>) {
    try {
        await p;
    } catch (e) {
        let error = e as AnchorError;
        console.log(error)

    }
}

export { requireBNEq, requirePublickeyEq };