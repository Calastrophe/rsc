
// The debugger is used by web targets.

// The purpose of our debugger is to plainly allow for seamless introspection of the emulator.
// We want to populate a memory view through egui with their table, need to figure that out.
// There should be two options, a pretty (annotated) view and just number view.

// Allow for easy symbol lookup and find location of variables in memory view.
// Implement backtracking in the debugger, need to keep track of all register/memory changes.
// NEED ABSOLUTE PROGRAM COUNTER

// Provide a current view of the registers in binary, hexadecimal, or decimal.
// Set breakpoints on certain program counters or labels.
// Be able to enable and disable said breakpoints.