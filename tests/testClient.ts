import { AnchorProvider, web3, Program, IdlTypes } from "@coral-xyz/anchor";
import { createMint } from '@solana/spl-token';
import { createAccounts } from './utils';
import { ClearingHouse } from "../target/types/clearing_house";
type PublicKey = web3.PublicKey;

export class TestClient {
    provider: AnchorProvider;
    signers: Array<web3.Keypair>;
    currentSignerIndex: number;
    program: Program<ClearingHouse>;

    state: PublicKey;
    collateralMint: PublicKey;
    collateralVault: PublicKey;
    collateralVaultAuthority: PublicKey;
    insuranceVault: PublicKey;
    insuranceVaultAuthority: PublicKey;
    markets: PublicKey;


    static async create(provider: AnchorProvider, clearingHouse: Program<ClearingHouse>, signersNum: number): Promise<TestClient> {
        const tc = new TestClient();
        tc.provider = provider;
        tc.program = clearingHouse;
        tc.signers = new Array<web3.Keypair>(signersNum);
        tc.currentSignerIndex = 0;
        for (let index = 0; index < signersNum; index++) {
            const key = web3.Keypair.generate();
            tc.signers[index] = key;

            const tx = await tc.provider.connection.requestAirdrop(key.publicKey, 100 * web3.LAMPORTS_PER_SOL);
            await tc.provider.connection.confirmTransaction(tx);
            console.log(`signer${index} [${key.publicKey}]: ${await provider.connection.getBalance(key.publicKey)}`)
        }

        return tc;
    }

    async initializeRelevantAccounts(mintDecimal: number, logAddrs = false) {
        this.collateralMint = await this.createMint(mintDecimal);
        [this.collateralVault,] = web3.PublicKey.findProgramAddressSync([Buffer.from('collateral_vault')], this.program.programId);
        [this.collateralVaultAuthority,] = web3.PublicKey.findProgramAddressSync([this.collateralVault.toBuffer()], this.program.programId);
        [this.insuranceVault,] = web3.PublicKey.findProgramAddressSync([Buffer.from('insurance_vault')], this.program.programId);
        [this.insuranceVaultAuthority,] = web3.PublicKey.findProgramAddressSync([this.insuranceVault.toBuffer()], this.program.programId);

        // create state && markets accounts
        [this.state, this.markets] = await createAccounts(
            this.provider,
            [8 + 1200, 8 + 31744],
            this.program.programId
        );

        if (logAddrs) {
            console.log(`collateral mint: ${this.collateralMint}
state: ${this.state}
collateral vault: ${this.collateralVault}
collateral vault authority: ${this.collateralVaultAuthority}
insurance vault: ${this.insuranceVault}
insurance vault authority: ${this.insuranceVaultAuthority}
markets: ${this.markets}`);
        }
    }

    async initialize(adminControlsPrices: boolean) {
        const signer = this.getCurrentSigner();
        await this.program.methods.initialize(adminControlsPrices)
            .accounts({
                admin: signer.publicKey,
                state: this.state,
                collateralMint: this.collateralMint,
                collateralVaultAuthority: this.collateralVaultAuthority,
                insuranceVaultAuthority: this.insuranceVaultAuthority,
                markets: this.markets,
            })
            .signers([signer])
            .rpc();
    }

    async getState(): Promise<IdlTypes<ClearingHouse>['state']> {
        return await this.program.account.state.fetch(this.state);
    }

    async getMarkets(): Promise<IdlTypes<ClearingHouse>['markets']> {
        return await this.program.account.markets.fetch(this.markets);
    }

    getCurrentSigner(): web3.Keypair {
        return this.signers[this.currentSignerIndex];
    }

    async createMint(decimal: number): Promise<PublicKey> {
        const currentSigner = this.getCurrentSigner();
        return await createMint(
            this.provider.connection,
            currentSigner,
            currentSigner.publicKey,
            null,
            decimal);
    }
}