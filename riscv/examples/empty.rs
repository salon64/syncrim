use clap::Parser;
use riscv::components::*;
use std::{collections::BTreeMap, path::PathBuf, rc::Rc};
use syncrim::common::{ComponentStore, Input};

#[derive(Parser, Debug)]
struct Args {
    /// Use a pre-compiled elf file instead of compiling one
    #[arg(short, long, default_value = "false")]
    use_elf: bool,
    /// Path to the pre-compiled elf file
    #[arg(short, long, default_value = "")]
    elf_path: String,
    /// Path to the assembly source file
    #[arg(short, long, default_value = "asm.s")]
    asm_path: String,
    /// Path to the linker script
    #[arg(short, long, default_value = "memory.x")]
    ls_path: String,
}

fn main() {
    let cs = ComponentStore { store: vec![] };
    let path = PathBuf::from("riscv.json");
    cs.save_file(&path);
    let dummy = Input::new("id", "field");
    #[cfg(feature = "gui-egui")]
    {
        let lib = ComponentStore {
            store: vec![
                Rc::new(InstrMem {
                    width: INSTR_MEM_WIDTH,
                    height: INSTR_MEM_HEIGHT,
                    id: "dummy_instr_mem".to_string(),
                    pos: (0.0, 0.0),
                    pc: dummy.clone(),
                    bytes: BTreeMap::new(),
                }),
                Rc::new(ALU {
                    id: "dummy_alu".to_string(),
                    pos: (0.0, 0.0),
                    operator_i: dummy.clone(),
                    operand_a_i: dummy.clone(),
                    operand_b_i: dummy.clone(),
                }),
                Rc::new(BranchLogic {
                    width: BRANCH_LOGIC_WIDTH,
                    height: BRANCH_LOGIC_HEIGHT,
                    id: "dummy_blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: dummy.clone(),
                    rs2: dummy.clone(),
                    ctrl: dummy.clone(),
                    enable: dummy.clone(),
                }),
            ],
        };
        let library = syncrim::gui_egui::editor::Library(lib.store);
        syncrim::gui_egui::gui(cs, &path, syncrim::gui_egui::editor::Library(library.0)).ok();
    }

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
