mod display {
    use crate::model::display::{App, Display};
    use druid::Lens;

    #[derive(Clone, Default, Debug)]
    pub struct FocusedDisplay;

    impl Lens<App, Option<Display>> for FocusedDisplay {
        fn with<V, F: FnOnce(&Option<Display>) -> V>(&self, data: &App, f: F) -> V {
            for (_, v) in data.display_geo.iter() {
                if v.focused {
                    return f(&Some(v.clone()));
                }
            }

            f(&None)
        }

        fn with_mut<V, F: FnOnce(&mut Option<Display>) -> V>(&self, data: &mut App, f: F) -> V {
            for (_, v) in data.display_geo.iter_mut() {
                if v.focused {
                    let mut tmp = Some(v.clone());
                    let ret = f(&mut tmp);
                    *v = tmp.unwrap();
                    return ret;
                }
            }

            f(&mut None)
        }
    }
}

pub use display::FocusedDisplay;