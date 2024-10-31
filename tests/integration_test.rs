
mod lib {
    use std::io::prelude::{*};
    use stama::{Execution, Machine};
    use stama::journal::{WriteGuaranteeLevel, Writer, StandardWriter};

    macro_rules! build_test_from_example {
        ($($name:ident, $path:expr, $transition_count: expr),* $(,)?) => {
            #[test]
            $(
            pub fn $name() {

                let mut file = std::fs::File::open($path).unwrap();
                let mut contents = String::new();

                file.read_to_string(&mut contents).unwrap();

                let machine: Machine = serde_json::from_str(&contents).unwrap();

                let mut execution: Execution = machine.execute(None);

                let mut jwriter = Writer::Standard(StandardWriter {});

                execution.journal.apply_writer(
                    WriteGuaranteeLevel::Writeback,
                    jwriter
                );

                let mut transitions = 0;

                for _state in execution {
                    transitions += 1;
                }

                assert_eq!(transitions, $transition_count);
            }
            )*
        };
    }

    build_test_from_example!{
        test_execution_basic, "examples/1_basic.json", 2,
    }
}

