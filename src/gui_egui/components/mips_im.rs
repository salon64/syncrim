// use crate::common::{EguiComponent, Ports, SignalUnsigned, Simulator};
// use crate::components::InstrMem;
// use crate::gui_egui::component_ui::{
//     drag_logic, input_change_id, input_selector, pos_drag_value, properties_window,
//     rect_with_hover, visualize_ports,
// };
// use crate::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
// use crate::gui_egui::gui::EguiExtra;
// use crate::gui_egui::helper::offset_helper;
// use egui::{
//     Align2, Area, Color32, Order, Pos2, Rect, Response, RichText, Shape, Slider, Stroke,
//     TextWrapMode, Ui, Vec2,
// };

// impl InstrMem {
//     fn side_panel(&self, ctx: &Context, simulator: Option<&mut Simulator>) {
//         Window::new("Instruction Memory")
//             .resizable(true)
//             .show(ctx, |ui| {
//                 TableBuilder::new(ui)
//                     .striped(true)
//                     .column(Column::initial(75.0).at_least(75.0))
//                     .column(Column::initial(10.0).resizable(false))
//                     .column(Column::initial(75.0).at_least(75.0))
//                     .column(Column::initial(75.0).at_least(50.0))
//                     .column(Column::initial(150.0).at_least(85.0).resizable(true))
//                     .column(Column::initial(5.0).at_least(5.0).resizable(false))
//                     .header(30.0, |mut header| {
//                         header.col(|ui| {
//                             ui.heading("Label");
//                         });
//                         header.col(|_ui| {});
//                         header.col(|ui| {
//                             ui.heading("Address");
//                         });
//                         header.col(|ui| {
//                             ui.heading("HEX");
//                         });
//                         header.col(|ui| {
//                             ui.heading("Instruction");
//                         });
//                     })
//                     .body(|body| {
//                         body.rows(
//                             15.0,
//                             (self.range.end - self.range.start) / 4,
//                             |index, mut row| {
//                                 let address = index * 4 + self.range.start;
//                                 let pc: u32 = {
//                                     if simulator.as_ref().is_some() {
//                                         simulator
//                                             .as_ref()
//                                             .unwrap()
//                                             .get_input_value(&self.pc)
//                                             .try_into()
//                                             .unwrap_or(0)
//                                     } else {
//                                         0
//                                     }
//                                 };
//                                 let (bg_color, fg_color) = {
//                                     if pc as usize == address {
//                                         (Color32::DARK_GRAY, Color32::WHITE)
//                                     } else {
//                                         (Color32::TRANSPARENT, Color32::LIGHT_GRAY)
//                                     }
//                                 };
//                                 let breakpoint_color = {
//                                     if self.breakpoints.borrow_mut().contains(&address) {
//                                         Color32::RED
//                                     } else {
//                                         Color32::TRANSPARENT
//                                     }
//                                 };
//                                 row.col(|ui| match &self.symbols.get(&address) {
//                                     Some(s) => {
//                                         ui.add(Label::new(format!("{}:", s)).truncate(true));
//                                     }
//                                     None => {}
//                                 });
//                                 //breakpoint
//                                 row.col(|ui| {
//                                     ui.label(RichText::new("•").color(breakpoint_color));
//                                 });
//                                 //address
//                                 row.col(|ui| {
//                                     ui.add(Label::new(format!("0x{:08x}", address)).truncate(true));
//                                 });
//                                 let mut bytes = [0u8; 4];
//                                 if !self.le {
//                                     bytes[3] = *self.bytes.get(&address).unwrap();
//                                     bytes[2] = *self.bytes.get(&(address + 1)).unwrap();
//                                     bytes[1] = *self.bytes.get(&(address + 2)).unwrap();
//                                     bytes[0] = *self.bytes.get(&(address + 3)).unwrap();
//                                 } else {
//                                     bytes[0] = *self.bytes.get(&address).unwrap();
//                                     bytes[1] = *self.bytes.get(&(address + 1)).unwrap();
//                                     bytes[2] = *self.bytes.get(&(address + 2)).unwrap();
//                                     bytes[3] = *self.bytes.get(&(address + 3)).unwrap();
//                                 }
//                                 let instr = ((bytes[3] as u32) << 24)
//                                     | ((bytes[2] as u32) << 16)
//                                     | ((bytes[1] as u32) << 8)
//                                     | (bytes[0] as u32);

//                                 let instr_fmt = match asm_riscv::I::try_from(instr) {
//                                     Ok(i) => riscv_asm_strings::StringifyUpperHex::to_string(&i),
//                                     Err(_) => "Unknown instruction".to_string(),
//                                 };
//                                 //hex instr
//                                 row.col(|ui| {
//                                     ui.add(Label::new(format!("0x{:08X}", instr)).truncate(true));
//                                 });
//                                 row.col(|ui| {
//                                     if ui
//                                         .add(
//                                             Label::new(
//                                                 RichText::new(instr_fmt)
//                                                     .color(fg_color)
//                                                     .background_color(bg_color),
//                                             )
//                                             .truncate(true)
//                                             .sense(Sense::click()),
//                                         )
//                                         .clicked()
//                                     {
//                                         trace!("clicked");
//                                         if !self.breakpoints.borrow_mut().remove(&address) {
//                                             self.breakpoints.borrow_mut().insert(address);
//                                         }
//                                     };
//                                 });
//                                 row.col(|_| {});
//                             },
//                         );
//                     });
//             });
//     }
// }

// #[typetag::serde]
// impl EguiComponent for InstrMem {
//     fn render(
//         &self,
//         ui: &mut Ui,
//         _ctx: &mut EguiExtra,
//         simulator: Option<&mut Simulator>,
//         offset: Vec2,
//         scale: f32,
//         clip_rect: Rect,
//         editor_mode: EditorMode,
//     ) -> Option<Vec<Response>> {
//         // 21x41
//         // middle: 11x 21y (0 0)
//         let oh: fn((f32, f32), f32, Vec2) -> Pos2 = offset_helper;
//         let offset_old = offset;
//         let mut offset = offset;
//         offset.x += self.pos.0 * scale;
//         offset.y += self.pos.1 * scale;
//         let s = scale;
//         let o = offset;
//         //self.side_panel(ui.ctx(), simulator);
//         // The shape
//         let rect = Rect {
//             min: oh((-self.width / 2f32, -self.height / 2f32), s, o),
//             max: oh((self.width / 2f32, self.height / 2f32), s, o),
//         };
//         ui.painter().add(Shape::rect_stroke(
//             rect,
//             Rounding::ZERO,
//             Stroke {
//                 width: scale,
//                 color: Color32::BLACK,
//             },
//         ));
//         ui.painter().text(
//             o.to_pos2(),
//             egui::Align2::CENTER_CENTER,
//             "InstrMem",
//             FontId::monospace(14.0),
//             Color32::BLACK,
//         );
//         let r = rect_with_hover(rect, clip_rect, editor_mode, ui, self.id.clone(), |ui| {
//             ui.label(format!("Id: {}", self.id.clone()));
//             ui.label("InstrMem");
//         });
//         match editor_mode {
//             EditorMode::Simulator => {
//                 self.side_panel(ui.ctx(), simulator);
//             }
//             _ => visualize_ports(ui, self.ports_location(), offset_old, scale, clip_rect),
//         }
//         Some(vec![r])
//     }

//     fn render_editor(
//         &mut self,
//         ui: &mut Ui,
//         context: &mut EguiExtra,
//         simulator: Option<&mut Simulator>,
//         offset: Vec2,
//         scale: f32,
//         clip_rect: Rect,
//         id_ports: &[(syncrim::common::Id, Ports)],
//         grid: &GridOptions,
//         editor_mode: EditorMode,
//     ) -> EditorRenderReturn {
//         let r_vec = InstrMem::render(
//             self,
//             ui,
//             context,
//             simulator,
//             offset,
//             scale,
//             clip_rect,
//             editor_mode,
//         )
//         .unwrap();
//         let resp = &r_vec[0];
//         let delete = drag_logic(
//             ui.ctx(),
//             resp,
//             &mut self.pos,
//             &mut context.pos_tmp,
//             scale,
//             offset,
//             grid,
//         );

//         properties_window(
//             ui,
//             self.id.clone(),
//             resp,
//             &mut context.properties_window,
//             |ui| {
//                 let mut clicked_dropdown = false;
//                 input_change_id(ui, &mut context.id_tmp, &mut self.id, id_ports);
//                 pos_drag_value(ui, &mut self.pos);
//                 clicked_dropdown |= input_selector(
//                     ui,
//                     &mut self.pc,
//                     crate::components::INSTR_MEM_PC_ID.to_string(),
//                     id_ports,
//                     self.id.clone(),
//                 );
//                 clicked_dropdown
//             },
//         );

//         EditorRenderReturn {
//             delete,
//             resp: Some(r_vec),
//         }
//     }

//     fn ports_location(&self) -> Vec<(syncrim::common::Id, Pos2)> {
//         let own_pos = Vec2::new(self.pos.0, self.pos.1);
//         vec![
//             (
//                 crate::components::INSTR_MEM_PC_ID.to_string(),
//                 Pos2::new(
//                     self.width / 10f32 * 1f32 - self.width / 2f32,
//                     -self.height / 2f32,
//                 ) + own_pos,
//             ),
//             (
//                 crate::components::INSTR_MEM_INSTRUCTION_ID.to_string(),
//                 Pos2::new(
//                     -self.width / 10f32 * 2f32 + self.width / 2f32,
//                     -self.height / 2f32,
//                 ) + own_pos,
//             ),
//         ]
//     }

//     fn top_padding(&self) -> f32 {
//         self.height / 4f32
//     }

//     fn set_pos(&mut self, pos: (f32, f32)) {
//         self.pos = pos;
//     }

//     fn get_pos(&self) -> (f32, f32) {
//         self.pos
//     }
// }
