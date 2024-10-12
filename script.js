const projects = [
    {
        id: 1,
        name: "Community Park",
        description: "Funding for a new community park.",
        targetAmount: 5000,
        currentAmount: 1200,
        milestones: ["Planning", "Construction", "Opening"]
    },
    {
        id: 2,
        name: "Solar Panel Installation",
        description: "Install solar panels in underserved areas.",
        targetAmount: 10000,
        currentAmount: 4000,
        milestones: ["Funding", "Installation", "Activation"]
    }
];

function displayProjects() {
    const projectList = document.getElementById("project-list");
    projects.forEach(project => {
        const projectDiv = document.createElement("div");
        projectDiv.innerHTML = `
            <h3>${project.name}</h3>
            <p>${project.description}</p>
            <p>Target Amount: $${project.targetAmount}</p>
            <p>Current Amount: $${project.currentAmount}</p>
            <button onclick="contribute(${project.id})">Contribute</button>
        `;
        projectList.appendChild(projectDiv);
    });
}

function contribute(projectId) {
    const amount = prompt("Enter contribution amount:");
    const project = projects.find(p => p.id === projectId);
    if (project && amount) {
        project.currentAmount += parseInt(amount);
        alert(`You contributed $${amount} to ${project.name}`);
        displayProjects();
    }
}

displayProjects();
