use crate::shroud_editor::ShroudEditor;

type SelectionMirrorSplit = (Vec<usize>, Vec<usize>, Vec<usize>);

impl ShroudEditor {
    pub fn get_selection_mirror_split(&self) -> SelectionMirrorSplit {
        let original_selection = self.shroud_interaction.selection();
        let mut whole_selection = Vec::new();
        let mut selection = Vec::new();
        let mut mirrors = Vec::new();
        original_selection.iter().for_each(|idx| {
            if !whole_selection.contains(idx) {
                whole_selection.push(*idx);
            }
            if !mirrors.contains(idx) {
                selection.push(*idx);
                if let Some(mirror_idx) = self.shroud[*idx].mirror_index_option {
                    mirrors.push(mirror_idx);
                    whole_selection.push(mirror_idx);
                }
            }
        });
        (whole_selection, selection, mirrors)
    }
}
