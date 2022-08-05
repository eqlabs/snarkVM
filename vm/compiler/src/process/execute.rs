// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

impl<N: Network> Process<N> {
    /// Executes the given authorization.
    #[inline]
    pub fn execute<A: circuit::Aleo<Network = N>, R: Rng + CryptoRng>(
        &self,
        authorization: Authorization<N>,
        rng: &mut R,
    ) -> Result<(Response<N>, Execution<N>)> {
        // Retrieve the main request (without popping it).
        let request = authorization.peek_next()?;
        // Prepare the stack.
        let stack = self.get_stack(request.program_id())?;

        // Ensure the network ID matches.
        ensure!(
            **request.network_id() == N::ID,
            "Network ID mismatch. Expected {}, but found {}",
            N::ID,
            request.network_id()
        );
        // Ensure that the function exists.
        if !stack.program().contains_function(request.function_name()) {
            bail!("Function '{}' does not exist.", request.function_name())
        }

        #[cfg(feature = "aleo-cli")]
        println!("{}", format!(" • Executing '{}/{}'...", request.program_id(), request.function_name()).dimmed());

        // Initialize the execution.
        let execution = Arc::new(RwLock::new(Execution::new()));
        // Execute the circuit.
        let response = stack.execute_function::<A, R>(CallStack::execute(authorization, execution.clone())?, rng)?;
        // Extract the execution.
        let execution = execution.read().clone();
        // Ensure the execution is not empty.
        ensure!(!execution.is_empty(), "Execution of '{}/{}' is empty", request.program_id(), request.function_name());

        Ok((response, execution))
    }

    /// Verifies the given execution is valid.
    #[inline]
    pub fn verify_execution(&self, execution: &Execution<N>) -> Result<()> {
        // Retrieve the edition.
        let edition = execution.edition();
        // Ensure the edition matches.
        ensure!(edition == N::EDITION, "Executed the wrong edition (expected '{}', found '{edition}').", N::EDITION);

        // Ensure the execution contains transitions.
        ensure!(!execution.is_empty(), "There are no transitions in the execution");

        // Ensure the number of transitions matches the program function.
        {
            // Retrieve the transition (without popping it).
            let transition = execution.peek()?;
            // Retrieve the stack.
            let stack = self.get_stack(transition.program_id())?;
            // Ensure the number of calls matches the number of transitions.
            let number_of_calls = stack.get_number_of_calls(transition.function_name())?;
            ensure!(
                number_of_calls == execution.len(),
                "The number of transitions in the execution is incorrect. Expected {number_of_calls}, but found {}",
                execution.len()
            );
        }

        // Replicate the execution stack for verification.
        let mut queue = execution.clone();

        // Verify each transition.
        while let Ok(transition) = queue.pop() {
            #[cfg(debug_assertions)]
            println!("Verifying transition for {}/{}...", transition.program_id(), transition.function_name());

            // Ensure the transition ID is correct.
            ensure!(**transition.id() == transition.to_root()?, "The transition ID is incorrect");

            // Ensure the number of inputs is within the allowed range.
            ensure!(transition.inputs().len() <= N::MAX_INPUTS, "Transition exceeded maximum number of inputs");
            // Ensure the number of outputs is within the allowed range.
            ensure!(transition.outputs().len() <= N::MAX_INPUTS, "Transition exceeded maximum number of outputs");

            // Ensure each input is valid.
            if transition.inputs().iter().any(|input| !input.verify()) {
                bail!("Failed to verify a transition input")
            }
            // Ensure each output is valid.
            if transition.outputs().iter().any(|output| !output.verify()) {
                bail!("Failed to verify a transition output")
            }

            // Ensure the fee is correct.
            match Program::is_coinbase(transition.program_id(), transition.function_name()) {
                true => ensure!(transition.fee() < &0, "The fee must be negative in a coinbase transition"),
                false => ensure!(transition.fee() >= &0, "The fee must be zero or positive"),
            }

            // Compute the x- and y-coordinate of `tpk`.
            let (tpk_x, tpk_y) = transition.tpk().to_xy_coordinate();

            // Construct the public inputs to verify the proof.
            let mut inputs = vec![N::Field::one(), *tpk_x, *tpk_y, **transition.tcm()];
            // Extend the inputs with the input IDs.
            inputs.extend(transition.inputs().iter().flat_map(|input| input.verifier_inputs()));

            // Retrieve the stack.
            let stack = self.get_stack(transition.program_id())?;
            // Retrieve the function from the stack.
            let function = stack.get_function(transition.function_name())?;
            // Determine the number of function calls in this function.
            let mut num_function_calls = 0;
            for instruction in function.instructions() {
                if let Instruction::Call(call) = instruction {
                    // Determine if this is a function call.
                    if call.is_function_call(stack)? {
                        num_function_calls += 1;
                    }
                }
            }
            // If there are function calls, append their inputs and outputs.
            if num_function_calls > 0 {
                // This loop takes the last `num_function_call` transitions, and reverses them
                // to order them in the order they were defined in the function.
                for transition in (*queue).iter().rev().take(num_function_calls).rev() {
                    // Extend the inputs with the input and output IDs of the external call.
                    inputs.extend(transition.inputs().iter().flat_map(|input| input.verifier_inputs()));
                    inputs.extend(transition.output_ids().map(|id| **id));
                }
            }

            // Lastly, extend the inputs with the output IDs and fee.
            inputs.extend(transition.outputs().iter().flat_map(|output| output.verifier_inputs()));
            inputs.push(*I64::<N>::new(*transition.fee()).to_field()?);

            #[cfg(debug_assertions)]
            println!("Transition public inputs ({} elements): {:#?}", inputs.len(), inputs);

            // Retrieve the verifying key.
            let verifying_key = self.get_verifying_key(transition.program_id(), transition.function_name())?;
            // Ensure the proof is valid.
            ensure!(
                verifying_key.verify(transition.function_name(), &inputs, transition.proof()),
                "Transition is invalid"
            );
        }
        Ok(())
    }
}
