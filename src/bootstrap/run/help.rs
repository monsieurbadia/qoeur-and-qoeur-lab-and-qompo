use crate::cmd::{Cmd, CmdResult};

impl Cmd {
  pub fn run_help(&mut self) -> CmdResult<()> {
    println!(
      "
    qoeur is a tool for managing qoeur source code.
    
    use:
    
      qoeur <command> [arguments]
    
    These are common qoeur commands used in various situations:
      
      copyright     print the qoeur copyright
      license       print the qoeur LICENSE
      repl          start the repl to play with the language
      test          test capsules
    
    use 'qoeur help <command>' for more instructions about what you can dot.
    
    Additional help topics:
    
      cache         build and test caching
    
    use 'qoeur help <topic>' for more information about that topic.
      "
    );
    Ok(())
  }
}
