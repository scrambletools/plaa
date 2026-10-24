//! A window's edit of its file: the edit, its history for undo and redo,
//! and what the bottom bar edits now.

use std::sync::Arc;
use std::time::Duration;

use plaa_media::info::Chapter;
use plaa_media::{Clip, Edit, Transform};

/// What the bottom bar edits in place of the playback controls.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    /// Choosing the part to keep, from and to seconds of the edited time:
    /// of the whole movie, or of one clip.
    Trim {
        clip: Option<usize>,
        from: f64,
        to: f64,
    },
    /// Arranging the clips, one of them chosen.
    Clips { selected: Option<usize> },
}

#[derive(Debug, Default)]
pub struct Editor {
    /// The edit, none while the file plays as it is.
    edit: Option<Edit>,
    undo: Vec<Option<Edit>>,
    redo: Vec<Option<Edit>>,
    pub mode: Option<Mode>,
    /// The edit as it was last exported, so closing warns only of changes
    /// not kept.
    exported: Option<Option<Edit>>,
}

impl Editor {
    pub fn edit(&self) -> Option<&Edit> {
        self.edit.as_ref()
    }

    pub fn is_edited(&self) -> bool {
        self.edit.is_some()
    }

    /// Whether there are changes no export has kept.
    pub fn unexported(&self) -> bool {
        self.edit.is_some() && self.exported.as_ref() != Some(&self.edit)
    }

    /// Notes that `edit` was exported.
    pub fn exported(&mut self, edit: Option<Edit>) {
        self.exported = Some(edit);
    }

    pub fn transform(&self) -> Transform {
        self.edit.as_ref().map(Edit::transform).unwrap_or_default()
    }

    /// The edit to change: the one there is, else the whole file
    /// `duration` seconds long.
    pub fn working(&self, duration: f64) -> Edit {
        self.edit.clone().unwrap_or_else(|| Edit::whole(duration))
    }

    /// Makes `edit` the edit, keeping the one before for undo. The whole
    /// file as it is counts as none. Returns whether it changed.
    pub fn commit(&mut self, edit: Edit, duration: f64) -> bool {
        let edit = (!edit.is_whole(duration)).then_some(edit);
        if edit == self.edit {
            return false;
        }
        self.undo.push(std::mem::replace(&mut self.edit, edit));
        self.redo.clear();
        true
    }

    /// Plays the file as it is again, as a change that can be undone.
    pub fn revert(&mut self) -> bool {
        if self.edit.is_none() {
            return false;
        }
        self.undo.push(self.edit.take());
        self.redo.clear();
        true
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    pub fn undo(&mut self) -> bool {
        let Some(previous) = self.undo.pop() else {
            return false;
        };
        self.redo.push(std::mem::replace(&mut self.edit, previous));
        true
    }

    pub fn redo(&mut self) -> bool {
        let Some(next) = self.redo.pop() else {
            return false;
        };
        self.undo.push(std::mem::replace(&mut self.edit, next));
        true
    }

    /// The edit the player plays: none while the clips are the whole
    /// file, as turning and flipping is the picture's alone.
    pub fn for_player(&self, duration: f64) -> Option<Arc<Edit>> {
        self.edit
            .as_ref()
            .filter(|edit| !edit.plays_whole(duration))
            .map(|edit| Arc::new(edit.clone()))
    }

    /// The clips, none while they are the whole file.
    pub fn clips(&self) -> Option<&[Clip]> {
        self.edit.as_ref().map(Edit::clips)
    }

    /// The file's time at `seconds` of the edited time.
    pub fn source_of(&self, seconds: f64) -> f64 {
        match &self.edit {
            Some(edit) => edit.source_of(seconds),
            None => seconds,
        }
    }

    /// The picture showing at `seconds` of the edited time: the file, as
    /// the edit numbers it, and its time there; none where it is black.
    pub fn picture_at(&self, seconds: f64) -> Option<(usize, f64)> {
        match &self.edit {
            Some(edit) => edit.fed_at(plaa_media::Track::Picture, seconds),
            None => Some((plaa_media::OWN, seconds)),
        }
    }

    /// Where the file's time `seconds` plays in the edited time, if it is
    /// kept.
    pub fn edited_of(&self, seconds: f64) -> Option<f64> {
        match &self.edit {
            Some(edit) => edit.edited_of(seconds),
            None => Some(seconds),
        }
    }

    /// Where clips meet, in the edited time.
    pub fn cuts(&self) -> Vec<f64> {
        self.edit
            .as_ref()
            .map(|edit| edit.starts().into_iter().skip(1).collect())
            .unwrap_or_default()
    }

    /// The file's chapters as they fall in the edit, in the order they
    /// play: each clip starts in the chapter its first moment is in, and
    /// a chapter carried on over a cut is not marked again.
    pub fn chapters(&self, chapters: &[Chapter]) -> Vec<Chapter> {
        let Some(edit) = &self.edit else {
            return chapters.to_vec();
        };
        let start_of = |chapter: &Chapter| chapter.start.as_secs_f64();
        // Each mark: the chapter's number and where it starts.
        let mut marks: Vec<(usize, f64)> = Vec::new();
        for (clip, at) in edit.clips().iter().zip(edit.starts()) {
            let current = chapters
                .iter()
                .rposition(|chapter| start_of(chapter) <= clip.from + 1e-6);
            if let Some(number) = current
                && marks.last().is_none_or(|(last, _)| *last != number)
            {
                marks.push((number, at));
            }
            for (number, chapter) in chapters.iter().enumerate() {
                let start = start_of(chapter);
                if start > clip.from + 1e-6 && start < clip.to {
                    marks.push((number, at + start - clip.from));
                }
            }
        }
        let length = edit.duration();
        let ends: Vec<f64> = marks
            .iter()
            .skip(1)
            .map(|(_, start)| *start)
            .chain([length])
            .collect();
        marks
            .iter()
            .zip(ends)
            .map(|((number, start), end)| Chapter {
                title: chapters[*number].title.clone(),
                start: Duration::from_secs_f64(*start),
                end: Duration::from_secs_f64(end),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edits_undo_and_redo() {
        let mut editor = Editor::default();
        assert!(
            !editor.commit(Edit::whole(60.0), 60.0),
            "the whole file is no edit"
        );
        let mut edit = editor.working(60.0);
        edit.trim(10.0, 20.0);
        assert!(editor.commit(edit, 60.0));
        assert!(editor.unexported());
        let mut turned = editor.working(60.0);
        turned.rotate(true);
        assert!(editor.commit(turned, 60.0));
        assert_eq!(editor.transform().turns, 1);
        assert!(editor.undo());
        assert_eq!(editor.transform().turns, 0);
        assert!(editor.is_edited());
        assert!(editor.undo());
        assert!(!editor.is_edited());
        assert!(!editor.undo());
        assert!(editor.redo() && editor.redo());
        assert_eq!(editor.transform().turns, 1);
        // A new change drops what could be redone.
        assert!(editor.undo());
        assert!(editor.revert());
        assert!(!editor.can_redo());
        assert!(!editor.is_edited());
    }

    #[test]
    fn turning_alone_leaves_the_player_the_whole_file() {
        let mut editor = Editor::default();
        let mut edit = editor.working(30.0);
        edit.flip(true);
        editor.commit(edit, 30.0);
        assert!(editor.is_edited());
        assert!(editor.for_player(30.0).is_none());
        let mut edit = editor.working(30.0);
        edit.split(10.0);
        // Split, the clips still play the whole file through.
        editor.commit(edit.clone(), 30.0);
        assert!(editor.for_player(30.0).is_none());
        assert_eq!(editor.cuts(), [10.0]);
        edit.move_clip(1, 0);
        editor.commit(edit, 30.0);
        assert!(editor.for_player(30.0).is_some());
        assert_eq!(editor.source_of(25.0), 5.0);
    }

    #[test]
    fn chapters_follow_the_clips() {
        let chapter = |title: &str, start: u64, end: u64| Chapter {
            title: Some(title.to_owned()),
            start: Duration::from_secs(start),
            end: Duration::from_secs(end),
        };
        let chapters = [
            chapter("A", 0, 10),
            chapter("B", 10, 20),
            chapter("C", 20, 30),
        ];
        let mut editor = Editor::default();
        let mut edit = editor.working(30.0);
        edit.split(15.0);
        edit.move_clip(1, 0);
        edit.trim(0.0, 25.0);
        editor.commit(edit, 30.0);
        // 15 to 30, then 0 to 10: the rest of B, C, then A.
        let kept = editor.chapters(&chapters);
        let titles: Vec<_> = kept
            .iter()
            .filter_map(|chapter| chapter.title.as_deref())
            .collect();
        assert_eq!(titles, ["B", "C", "A"]);
        let starts: Vec<_> = kept.iter().map(|chapter| chapter.start.as_secs()).collect();
        assert_eq!(starts, [0, 5, 15]);
        assert_eq!(kept[2].end, Duration::from_secs(25));
        // Split only, a chapter going on over the cut is marked once.
        let mut editor = Editor::default();
        let mut edit = editor.working(30.0);
        edit.split(15.0);
        edit.flip(false);
        editor.commit(edit, 30.0);
        assert_eq!(editor.chapters(&chapters).len(), 3);
    }
}
