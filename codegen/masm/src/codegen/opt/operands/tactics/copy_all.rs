use super::*;

/// This tactic simply copies all expected operands right-to-left.
///
/// As a precondition, this tactic requires that all expected operands are copies.
#[derive(Default)]
pub struct CopyAll;
impl Tactic for CopyAll {
    fn cost(&self, context: &SolverContext) -> usize {
        core::cmp::max(context.copies().len(), 1)
    }

    fn apply(&mut self, builder: &mut SolutionBuilder) -> TacticResult {
        // We can't apply this tactic if any values should be moved
        let arity = builder.arity();
        if builder.num_copies() != arity {
            log::debug!(
                "expected all operands to require copying; but only {} out of {} operands are \
                 copied",
                builder.num_copies(),
                arity
            );
            return Err(TacticError::PreconditionFailed);
        }

        // Visit the expected operands in bottom-up order copying them as we go
        for index in (0..(arity as u8)).rev() {
            let expected_value = builder.unwrap_expected(index);
            // Because we create aliases for all copies we expect, as well as copies already
            // present on the stack, we won't find the expected value (which is an alias) unless
            // a copy already exists. As things are today, we should never even hit this branch,
            // since we should be copying-on-demand, and thus never leaving copies on the stack
            // across instructions, however we gracefully handle the case here, should we ever
            // add passes which proactively introduce copies on the stack during lowering.
            //
            // In short, if we find a copy on the stack, we don't make another copy, we use
            // the existing one. Otherwise, we copy as usual.
            if let Some(current_position) = builder.get_current_position(&expected_value) {
                if current_position == index {
                    log::trace!("{expected_value:?} is at its expected index {current_position}");
                    continue;
                }

                log::trace!(
                    "moving {expected_value:?} at index {index} up to top of stack, shifting {:?} \
                     down one",
                    builder.unwrap_current(0)
                );
                builder.movup(current_position);
            } else {
                let current_position = builder
                    .get_current_position(&expected_value.unaliased())
                    .unwrap_or_else(|| {
                        panic!(
                            "expected {:?} on the stack, but it was not found",
                            expected_value.unaliased()
                        )
                    });
                // A copy already exists, so use it
                log::trace!(
                    "copying {expected_value:?} at index {index} to top of stack, shifting {:?} \
                     down one",
                    builder.unwrap_current(0)
                );
                builder.dup(current_position, expected_value.unwrap_alias());
            }
        }

        Ok(())
    }
}
