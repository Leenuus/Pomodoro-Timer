use ratatui::widgets::ListState;


#[derive(Debug, Default)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        if !items.is_empty() {
            StatefulList {
                state: ListState::default().with_selected(Some(0)),
                items,
            }
        } else {
            StatefulList {
                state: ListState::default(),
                items,
            }
        }
    }

    pub fn next_entry(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            // NOTE select first display item but not the first item
            None => self.state.offset(),
        };
        self.state.select(Some(i));
    }

    pub fn previous_entry(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.state.offset(),
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
