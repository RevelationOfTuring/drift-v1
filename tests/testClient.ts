import { AnchorProvider, web3 } from "@coral-xyz/anchor";

export class TestClient {
    provider: AnchorProvider;
    signers: Array<web3.Keypair>;

    static async create(provider: AnchorProvider, signersNum: number): Promise<TestClient> {
        const tc = new TestClient();
        tc.provider = provider;
        tc.signers = new Array<web3.Keypair>(signersNum);
        for (let index = 0; index < signersNum; index++) {
            const key = web3.Keypair.generate();
            tc.signers[index] = key;

            const tx = await tc.provider.connection.requestAirdrop(key.publicKey, 100 * web3.LAMPORTS_PER_SOL);
            await tc.provider.connection.confirmTransaction(tx);
            console.log(`signer${index} [${key.publicKey}]: ${await provider.connection.getBalance(key.publicKey)}`)
        }

        return tc;
    }
}