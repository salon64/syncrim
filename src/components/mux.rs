use crate::common::{Component, Id, Input, InputId, OutputType, Ports, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Mux {
    pub id: Id,
    pub pos: (f32, f32),
    pub select: InputId,
    pub m_in: Vec<InputId>,
}

impl Mux {
    pub fn new(id: String, pos: (f32, f32), select: Input, m_in: Vec<Input>) -> Self {
        let mut v = vec![];
        for (i, input) in m_in.iter().enumerate() {
            v.push(InputId {
                id: format!("in{}", i),
                input: input.clone(),
            });
        }
        Mux {
            id: id.clone(),
            pos,
            select: InputId {
                id: String::from("select"),
                input: select,
            },
            m_in: v,
        }
    }
}

#[typetag::serde]
impl Component for Mux {
    fn to_(&self) {
        trace!("mux");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        let mut inputs = vec![self.select.clone()];
        let mut m = self.m_in.clone();
        inputs.append(&mut m);

        (
            self.id.clone(),
            Ports {
                inputs,
                out_type: OutputType::Combinatorial,
                outputs: vec!["out".into()],
            },
        )
    }

    // propagate selected input value to output
    fn clock(&self, simulator: &mut Simulator) {
        // get input value
        let select = simulator.get_input_val(&self.select.input) as usize;
        trace!("select {}", select);
        let value = simulator.get_input_val(&self.m_in[select].input);

        // set output
        simulator.set_out_val(&self.id, "out", value);
    }
}
