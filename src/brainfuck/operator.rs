#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum Operator {
    IncrementPointer, // >  -  move the pointer right
    DecrementPointer, // <  -  move the pointer left
    IncrementCell,    // +  -  increment the current cell
    DecrementCell,    // -  -  decrement the current cell
    Output,           // .  -  output the value of the current cell
    Input,            // ,  -  replace the value of the current cell with input
    LoopStart,        // [  -  jump to the matching ] instruction if the current value is zero
    LoopEnd,          // ]  -  jump to the matching [ instruction if the current value is not zero
}
