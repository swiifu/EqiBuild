// Load Web3.js
const Web3 = require('web3');
const web3 = new Web3('https://YOUR_BLOCKCHAIN_PROVIDER_URL');

// ABI and Contract Address
const contractABI = [/* Your Contract ABI Here */];
const contractAddress = 'YOUR_CONTRACT_ADDRESS';

const eqiBuildContract = new web3.eth.Contract(contractABI, contractAddress);

// Function to create a project
async function createProject(projectName, milestoneGoal, deadline) {
    const accounts = await web3.eth.getAccounts();
    return eqiBuildContract.methods.create_project(projectName, milestoneGoal, deadline)
        .send({ from: accounts[0] });
}

// Function to donate
async function donateToProject(projectName, amount) {
    const accounts = await web3.eth.getAccounts();
    return eqiBuildContract.methods.donate(projectName, amount)
        .send({ from: accounts[0] });
}

// Example call to create a project
createProject('New Park', 1000, 1692335369)
    .then(receipt => console.log('Project Created:', receipt))
    .catch(err => console.error('Error:', err));

// Example call to donate to a project
donateToProject('New Park', 50)
    .then(receipt => console.log('Donation Successful:', receipt))
    .catch(err => console.error('Error:', err));
