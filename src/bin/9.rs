use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
enum ExitCode {
  Output(i64),
  AwaitInput,
  Halted
}

#[derive(Clone)]
struct IntCoder {
  program: HashMap<i64,i64>,
  has_halted: bool,
  pc: i64,
  ptr_offset: i64,
}

impl IntCoder {
  fn new(p: &[i64]) -> Self {
    let program = p.iter()
      .enumerate()
      .map(|(k,v)| (k as i64, *v))
      .collect();
    Self {
      program,
      ptr_offset: 0,
      pc: 0,
      has_halted: false,
    }
  }

  fn get(&mut self, pos: i64) -> i64 {
    assert!(pos >= 0, "fetch from negative address");
    *self.program.get(&pos).unwrap_or(&0)
  }

  fn parse_inst(&mut self) -> (i64, i64, i64, i64) {
    let code = self.get(self.pc);
    let opcode = code % 100;
    let mode1 = (code / 100)   % 10;
    let mode2 = (code / 1000)  % 10;
    let mode3 = (code / 10000) % 10;
    (opcode, mode1, mode2, mode3)
  }

  fn fetch(&mut self, offset: i64, mode: i64) -> i64 {
    let pos = self.pc + offset;
    match mode {
      0 => {
        let v = self.get(pos);
        self.get(v)
      },
      1 => { self.get(pos) },
      2 => {
        let v = self.get(pos) + self.ptr_offset;
        self.get(v)
      },
      _ => unreachable!(format!("invalid mode {}", mode)),
    }
  }

  fn fetch_set_adr(&mut self, offset: i64, mode: i64) -> i64 {
    let pos = self.pc + offset;
    match mode {
      2 => self.get(pos) + self.ptr_offset,
      _ => self.get(pos),
    }
  }

  fn execute(&mut self, input: i64) -> ExitCode {
    let mut has_used_input = false;
    while !self.has_halted {
      let (opcode, mode1, mode2, mode3) = self.parse_inst();
      match opcode {
        1 => { // add
          let a = self.fetch(1, mode1);
          let b = self.fetch(2, mode2);
          let c = self.fetch_set_adr(3, mode3);
          self.program.insert(c, a + b);
          self.pc += 4;
        },
        2 => { // mul
          let a = self.fetch(1, mode1);
          let b = self.fetch(2, mode2);
          let c = self.fetch_set_adr(3, mode3);
          self.program.insert(c, a * b);
          self.pc += 4;
        },
        3 => { // input
          if has_used_input {
            return ExitCode::AwaitInput;
          }
          has_used_input = true;

          let a = self.fetch_set_adr(1, mode1);
          self.program.insert(a, input);
          self.pc += 2;
        },
        4 => { // output
          let output = self.fetch(1, mode1);
          self.pc += 2;
          return ExitCode::Output(output);
        },
        5 => { // jnz
          let a = self.fetch(1, mode1);
          let b = self.fetch(2, mode2);
          self.pc = if a != 0 { b } else { self.pc + 3 };
        },
        6 => { // jz
          let a = self.fetch(1, mode1);
          let b = self.fetch(2, mode2);
          self.pc = if a == 0 { b } else { self.pc + 3 };
        },
        7 => { // slt
          let a = self.fetch(1, mode1);
          let b = self.fetch(2, mode2);
          let c = self.fetch_set_adr(3, mode3);
          self.program.insert(c, (a < b) as i64);
          self.pc += 4;
        },
        8 => { // seq
          let a = self.fetch(1, mode1);
          let b = self.fetch(2, mode2);
          let c = self.fetch_set_adr(3, mode3);
          self.program.insert(c, (a == b) as i64);
          self.pc += 4;
        },
        9 => { // update ptr_offset
          let a = self.fetch(1, mode1);
          self.ptr_offset += a;
          self.pc += 2;
        },
        99 => {
          self.has_halted = true;
          break;
        }
        _ => unreachable!(format!("invalid opcode {}", opcode)),
      }
    }
    ExitCode::Halted
  }
}

fn main() {
  let program = [1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1102,1,3,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1101,0,493,1024,1102,1,38,1015,1101,20,0,1011,1101,0,509,1026,1101,0,32,1018,1101,0,333,1022,1102,1,0,1020,1101,326,0,1023,1101,0,33,1010,1101,21,0,1016,1101,25,0,1004,1102,28,1,1008,1102,1,506,1027,1102,488,1,1025,1101,0,27,1013,1101,1,0,1021,1101,0,34,1019,1101,607,0,1028,1102,1,23,1003,1102,26,1,1007,1102,29,1,1009,1101,31,0,1000,1102,37,1,1012,1101,30,0,1005,1101,602,0,1029,1101,36,0,1002,1102,1,22,1001,1102,1,35,1014,1102,24,1,1006,1102,39,1,1017,109,4,21102,40,1,6,1008,1010,40,63,1005,63,203,4,187,1106,0,207,1001,64,1,64,1002,64,2,64,109,13,1206,3,221,4,213,1106,0,225,1001,64,1,64,1002,64,2,64,109,-5,1208,-9,22,63,1005,63,241,1106,0,247,4,231,1001,64,1,64,1002,64,2,64,109,-5,21107,41,40,3,1005,1010,263,1106,0,269,4,253,1001,64,1,64,1002,64,2,64,109,-1,1202,3,1,63,1008,63,29,63,1005,63,295,4,275,1001,64,1,64,1106,0,295,1002,64,2,64,109,16,21108,42,42,-8,1005,1014,313,4,301,1105,1,317,1001,64,1,64,1002,64,2,64,109,-4,2105,1,5,1001,64,1,64,1105,1,335,4,323,1002,64,2,64,109,-5,1207,-4,28,63,1005,63,355,1001,64,1,64,1105,1,357,4,341,1002,64,2,64,109,2,21102,43,1,-1,1008,1014,45,63,1005,63,377,1106,0,383,4,363,1001,64,1,64,1002,64,2,64,109,-10,1208,-3,36,63,1005,63,401,4,389,1106,0,405,1001,64,1,64,1002,64,2,64,109,6,21107,44,45,1,1005,1012,423,4,411,1105,1,427,1001,64,1,64,1002,64,2,64,109,4,21101,45,0,3,1008,1018,45,63,1005,63,453,4,433,1001,64,1,64,1105,1,453,1002,64,2,64,109,-23,2101,0,10,63,1008,63,36,63,1005,63,475,4,459,1106,0,479,1001,64,1,64,1002,64,2,64,109,26,2105,1,6,4,485,1105,1,497,1001,64,1,64,1002,64,2,64,109,4,2106,0,5,1105,1,515,4,503,1001,64,1,64,1002,64,2,64,109,-25,1201,10,0,63,1008,63,26,63,1005,63,537,4,521,1105,1,541,1001,64,1,64,1002,64,2,64,109,18,21101,46,0,-1,1008,1014,43,63,1005,63,565,1001,64,1,64,1106,0,567,4,547,1002,64,2,64,109,-6,1201,-4,0,63,1008,63,33,63,1005,63,587,1105,1,593,4,573,1001,64,1,64,1002,64,2,64,109,22,2106,0,-3,4,599,1105,1,611,1001,64,1,64,1002,64,2,64,109,-28,2102,1,-2,63,1008,63,22,63,1005,63,633,4,617,1105,1,637,1001,64,1,64,1002,64,2,64,109,-1,21108,47,44,9,1005,1011,653,1105,1,659,4,643,1001,64,1,64,1002,64,2,64,109,10,2107,24,-8,63,1005,63,681,4,665,1001,64,1,64,1105,1,681,1002,64,2,64,109,-11,2107,31,4,63,1005,63,697,1106,0,703,4,687,1001,64,1,64,1002,64,2,64,109,8,2101,0,-8,63,1008,63,23,63,1005,63,727,1001,64,1,64,1105,1,729,4,709,1002,64,2,64,109,-16,2108,21,10,63,1005,63,749,1001,64,1,64,1106,0,751,4,735,1002,64,2,64,109,17,2108,36,-8,63,1005,63,769,4,757,1105,1,773,1001,64,1,64,1002,64,2,64,109,-10,1207,1,23,63,1005,63,791,4,779,1105,1,795,1001,64,1,64,1002,64,2,64,109,-3,2102,1,6,63,1008,63,22,63,1005,63,815,1106,0,821,4,801,1001,64,1,64,1002,64,2,64,109,16,1205,7,837,1001,64,1,64,1105,1,839,4,827,1002,64,2,64,109,-5,1202,0,1,63,1008,63,30,63,1005,63,863,1001,64,1,64,1106,0,865,4,845,1002,64,2,64,109,4,1205,9,883,4,871,1001,64,1,64,1106,0,883,1002,64,2,64,109,16,1206,-7,899,1001,64,1,64,1106,0,901,4,889,4,64,99,21102,1,27,1,21101,915,0,0,1105,1,922,21201,1,47633,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21102,942,1,0,1105,1,922,22102,1,1,-1,21201,-2,-3,1,21101,957,0,0,1106,0,922,22201,1,-1,-2,1105,1,968,22101,0,-2,-2,109,-3,2106,0,0];
  let mut coder = IntCoder::new(&program);
  while !coder.has_halted {
    if let ExitCode::Output(i) = coder.execute(1) {
      println!("{}",i);
    }
  }
}
