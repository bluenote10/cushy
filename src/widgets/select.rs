//! A selectable, labeled widget representing a value.
use std::fmt::Debug;
use std::panic::{RefUnwindSafe, UnwindSafe};

use kludgine::Color;

use crate::styles::components::OutlineColor;
use crate::styles::{Component, DynamicComponent};
use crate::value::{Dynamic, IntoDynamic, IntoValue, MapEach, Value};
use crate::widget::{MakeWidget, MakeWidgetWithId, WidgetInstance};
use crate::widgets::button::{ButtonBackground, ButtonHoverBackground, ButtonKind};

/// A selectable, labeled widget representing a value.
#[derive(Debug)]
pub struct Select<T> {
    /// The value this button represents.
    pub value: T,
    /// The state (value) of the select.
    pub state: Dynamic<T>,
    /// The button kind to use as the basis for this select. Selects default to
    /// [`ButtonKind::Transparent`].
    pub kind: Value<ButtonKind>,
    label: WidgetInstance,
}

impl<T> Select<T> {
    /// Returns a new select that sets `state` to `value` when pressed. `label`
    /// is drawn inside of the button.
    pub fn new(value: T, state: impl IntoDynamic<T>, label: impl MakeWidget) -> Self {
        Self {
            value,
            state: state.into_dynamic(),
            kind: Value::Constant(ButtonKind::Transparent),
            label: label.make_widget(),
        }
    }

    /// Updates the button kind to use as the basis for this select, and
    /// returns self.
    ///
    /// Selects default to [`ButtonKind::Transparent`].
    #[must_use]
    pub fn kind(mut self, kind: impl IntoValue<ButtonKind>) -> Self {
        self.kind = kind.into_value();
        self
    }
}

impl<T> MakeWidgetWithId for Select<T>
where
    T: Clone + Debug + Eq + RefUnwindSafe + UnwindSafe + Send + Sync + 'static,
{
    fn make_with_id(self, id: crate::widget::WidgetTag) -> WidgetInstance {
        let selected = self.state.map_each({
            let value = self.value.clone();
            move |state| state == &value
        });
        let selected_color = DynamicComponent::new({
            let selected = selected.clone();
            move |context| {
                if selected.get_tracking_refresh(context) {
                    Some(Component::Color(context.get(&SelectedColor)))
                } else {
                    None
                }
            }
        });
        let kind = (&selected, &self.kind.into_dynamic()).map_each(|(selected, default_kind)| {
            if *selected {
                ButtonKind::Solid
            } else {
                *default_kind
            }
        });
        self.label
            .into_button()
            .on_click(move |()| {
                self.state.set(self.value.clone());
            })
            .kind(kind)
            .with_dynamic(&ButtonBackground, selected_color.clone())
            .with_dynamic(&ButtonHoverBackground, selected_color)
            .make_with_id(id)
    }
}

define_components! {
    Select {
        /// The color of the selected [`Select`] widget.
        SelectedColor(Color, "color", @OutlineColor)
    }
}