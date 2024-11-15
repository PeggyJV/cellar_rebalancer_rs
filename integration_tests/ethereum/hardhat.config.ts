import '@nomiclabs/hardhat-ethers';
import '@nomiclabs/hardhat-waffle';
import {task} from "hardhat/config";
import * as constants from "./addresses";

task(
    'integration_test_setup',
    'Sets up contracts for the integration test',
    async (args, hre) => {
        const whaleSigner = await hre.ethers.getSigner(constants.WHALE);

        for (let addr of constants.VALIDATORS) {
            await whaleSigner.sendTransaction({
                to: addr,
                value: hre.ethers.utils.parseEther('100'),
            });
        }

        let powers: number[] = [1073741823,1073741823,1073741823,1073741823];
        let powerThreshold: number = 6666;

        const Gravity = await hre.ethers.getContractFactory("Gravity");
        const gravity = (await Gravity.deploy(
            hre.ethers.utils.formatBytes32String("gravitytest"),
            powerThreshold,
            constants.VALIDATORS,
            powers
        ));

        await gravity.deployed();
        console.log(`gravity contract deployed at - ${gravity.address}`)

        const AaveCellar = await hre.ethers.getContractFactory("MockAaveV2StablecoinCellar");
        const aaveCellar = (await AaveCellar.deploy());
        await aaveCellar.deployed();
        console.log(`aave contract deployed at - ${aaveCellar.address}`);

        const VaultCellar = await hre.ethers.getContractFactory("Cellar");
        const vaultCellar = (await VaultCellar.deploy());
        await vaultCellar.deployed();
        console.log(`vault cellar contract deployed at - ${vaultCellar.address}`);

        const CellarV2_2 = await hre.ethers.getContractFactory("CellarV2_2");
        const cellarV2_2 = (await CellarV2_2.deploy());
        await cellarV2_2.deployed();
        console.log(`cellar v2.2 contract deployed at - ${cellarV2_2.address}`);

        const Adaptor = await hre.ethers.getContractFactory("Adaptor");
        const adaptor = (await Adaptor.deploy());
        await adaptor.deployed();
        console.log(`adaptor contract deployed at - ${adaptor.address}`);

        const Manager = await hre.ethers.getContractFactory("ManagerWithMerkleVerification");
        const manager = (await Manager.deploy(
            gravity.address,
            "0x0000000000000000000000000000000000000000",
            "0x0000000000000000000000000000000000000000"
        ));
        await manager.deployed();
        console.log(`manager contract deployed at - ${manager.address}`);

        let cellarSignerAddress = await aaveCellar.signer.getAddress()
        await hre.network.provider.request({
            method: 'hardhat_impersonateAccount',
            params: [cellarSignerAddress],
        });

        let { aaveHash } = await aaveCellar.transferOwnership(gravity.address, {
            gasPrice: hre.ethers.BigNumber.from('99916001694'),
            from: cellarSignerAddress
        });
        console.log(
            `Aave Cellar contract at ${aaveCellar.address} is now owned by Gravity contract at ${gravity.address} with hash ${aaveHash}`,
        );

        let { cellarV2_2Hash } = await cellarV2_2.setOwner(gravity.address, {
            gasPrice: hre.ethers.BigNumber.from('99916001694'),
            from: cellarSignerAddress
        });
        console.log(
            `CellarV2.2 contract at ${cellarV2_2.address} is now owned by Gravity contract at ${gravity.address} with hash ${cellarV2_2Hash}`,
        );

        let { vaultHash } = await vaultCellar.setOwner(gravity.address, {
            gasPrice: hre.ethers.BigNumber.from('99916001694'),
            from: cellarSignerAddress
        });
        console.log(
            `Vault Cellar contract at ${vaultCellar.address} is now owned by Gravity contract at ${gravity.address} with hash ${vaultHash}`,
        );

        let { adaptorHash } = await adaptor.setOwner(gravity.address, {
            gasPrice: hre.ethers.BigNumber.from('99916001694'),
            from: cellarSignerAddress
        });
        console.log(
            `Adaptor contract at ${adaptor.address} is now owned by Gravity contract at ${gravity.address} with hash ${adaptorHash}`,
        );

        let { managerHash } = await manager.setOwner(gravity.address, {
            gasPrice: hre.ethers.BigNumber.from('99916001694'),
            from: cellarSignerAddress
        });
        console.log(
            `Manager Cellar contract at ${manager.address} is now owned by Gravity contract at ${gravity.address} with hash ${managerHash}`,
        )

        await hre.network.provider.send("evm_setIntervalMining", [1000]);

        await hre.run('node');
    });


/**
 * @type import('hardhat/config').HardhatUserConfig
 */

module.exports = {
    solidity: {
        compilers: [
            {
                version: '0.8.0',
                settings: {
                    optimizer: {
                        enabled: true,
                        runs: 200,
                    },
                },
            },
            {
                version: '0.8.16',
                settings: {
                    optimizer: {
                        enabled: true,
                    },
                },
            },
        ],
    },
    typechain: {
        outDir: 'typechain',
        target: 'ethers-v5',
        runOnCompile: true,
    },
    gasReporter: {
        enabled: true,
    },
};
