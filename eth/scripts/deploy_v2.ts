import { ethers, upgrades } from "hardhat";

const STARKNET_CONTRACT = "0xde29d060D45901Fb19ED6C6e959EB22d8626708e";

async function main() {
  const [deployer] = await ethers.getSigners();

  console.log("using deployer", deployer);

  // We get the contract to deploy
  const Terabethia = await ethers.getContractFactory("TerabethiaV2");

  const impl = await Terabethia.deploy();
  await impl.deployed();

  // we only support sequenceNumber=1 as state init
  const initialState = ethers.utils.defaultAbiCoder.encode(["uint256"], [1]);
  console.log({ initialState });

  // const tera = await Proxy.deploy(300);
  const tera = await upgrades.deployProxy(Terabethia, [STARKNET_CONTRACT]);
  await tera.deployed();

  // set proxy
  const EthProxy = await ethers.getContractFactory("EthProxy");
  const ethProxy = await EthProxy.deploy(tera.address);

  console.log("Terabethia deployed to:", impl.address);
  console.log("Terabethia proxy deployed to:", tera.address);
  console.log("Eth Bridge deployed to:", ethProxy.address);

  console.log("Execute these commands to verify contracts on Etherscan:");
  console.log(`npx hardhat verify --network goerli ${impl.address}`);
  console.log(
    `npx hardhat verify --network goerli ${ethProxy.address} ${tera.address}`
  );
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
