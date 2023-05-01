/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    record Token {
        owner: address,
        gates: u64,
        amount: u128,
    }
    
    mapping amounts: address => u128;
    mapping tokens: address => Token;
    
    transition decrease_self(amount: u128) {
        return then finalize(self.caller, amount);
    }
    
    finalize decrease_self(addr: address, amount: u128) {
        Mapping::get_or_init(tokens, addr, amount, 1u128);
        tokens.get_or_init(addr, amount, 1u128);
        Mapping::get_or_init(amounts, 1u8);
        amounts.get_or_init(1u8);
        Mapping::get_or_init(amounts);
        amounts.get_or_init();
    }
}