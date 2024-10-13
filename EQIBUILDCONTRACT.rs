#![no_std] // Soroban contract

use soroban_sdk::{contractimpl, Env, Address, Symbol, Vec, Map};

pub struct EqiBuildContract;

#[derive(Clone)]
pub struct Project {
    pub name: Symbol,
    pub total_raised: i128,      // Total amount of XLM raised
    pub milestone_goal: i128,     // Milestone goal in XLM
    pub deadline: u64,            // Deadline timestamp (block height or time)
    pub completed: bool,          // Whether the milestone is reached
}

#[contractimpl]
impl EqiBuildContract {
    // Initialize a new project
    pub fn create_project(
        env: Env,
        project_name: Symbol,
        milestone_goal: i128,
        deadline: u64,
    ) -> Address {
        let project = Project {
            name: project_name,
            total_raised: 0,
            milestone_goal,
            deadline,
            completed: false,
        };

        // Store the project in the contract's storage
        let key = project_name.clone();
        env.storage().set(&key, project);

        // Return the project's address for further interaction
        env.ledger().account().address()
    }

    // Donate XLM to a specific project
    pub fn donate(env: Env, project_name: Symbol, amount: i128) -> Project {
        let mut project: Project = env.storage().get(&project_name).expect("Project not found");

        let current_time = env.ledger().timestamp();

        // Check if the donation is within the deadline
        if current_time > project.deadline {
            panic!("The deadline has passed. No more donations accepted.");
        }

        // Update the total raised
        let new_total = project.total_raised + amount;

        // If the milestone is reached, mark the project as completed
        if new_total >= project.milestone_goal {
            env.events().publish(
                Symbol::short("MilestoneReached"),
                (project.name.clone(), new_total),
            );
            project.completed = true;
        }

        project.total_raised = new_total;

        // Save updated project back to storage
        env.storage().set(&project_name, project.clone());

        project
    }

    // Check milestone status and handle failure to meet it
    pub fn check_milestone(env: Env, project_name: Symbol) -> Option<String> {
        let project: Project = env.storage().get(&project_name).expect("Project not found");

        let current_time = env.ledger().timestamp();

        // If the deadline has passed and the milestone was not met
        if current_time > project.deadline && !project.completed {
            // Refund logic placeholder (to be implemented)
            env.events().publish(
                Symbol::short("MilestoneFailed"),
                project.name.clone(),
            );
            return Some("Milestone not reached. Donations will be refunded.".to_string());
        }

        // If milestone met
        if project.completed {
            return Some("Milestone reached! Funds will be released.".to_string());
        }

        // Otherwise, keep raising funds
        None
    }
}
