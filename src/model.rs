use druid::im::HashMap;
use druid::{Data, Lens};

use swayipc::Output;


#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub enum Transform {
	None,
	R90,
	R180,
	R270
}

impl Default for Transform {
	fn default() -> Transform {
		Transform::None
	}
}

impl std::fmt::Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
        	Transform::R90 => "90",
        	Transform::R180 => "180",
        	Transform::R270 => "270",
        	_ => "normal",
        })
    }
}

#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub struct Scale(pub f64);

impl From<f64> for Scale {
    fn from(i: f64) -> Scale {
        Scale(i)
    }
}

impl std::str::FromStr for Scale {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Scale(s.parse::<f64>()?))
    }
}

impl std::fmt::Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub struct Pos(pub i32, pub i32);

impl From<(i32, i32)> for Pos {
    fn from(i: (i32, i32)) -> Pos {
        Pos(i.0, i.1)
    }
}

impl Into<(i32, i32)> for Pos {
    fn into(self) -> (i32, i32) {
        (self.0, self.1)
    }
}

impl std::str::FromStr for Pos {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')' || p == ' ')
            .split(',')
            .collect();

        if coords.len() < 2 {
            return Err(i32::from_str("a").err().unwrap());
        }

        Ok(Pos(
            coords[0].trim_matches(|p| p == ' ').parse::<i32>()?,
            coords[1].trim_matches(|p| p == ' ').parse::<i32>()?,
        ))
    }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Clone, Default, Data, Lens)]
pub struct DisplayInfo {
    pub name: String,
    pub make: String,
    pub model: String,
    pub serial: String,
    pub active: bool,

    pub position: Pos,
    pub size: (u32, u32),
    pub scale: Scale,
    pub transform: Transform,

    pub id: Option<i64>,

    pub focused: bool,
}

impl From<Output> for DisplayInfo {
    fn from(o: Output) -> Self {
        DisplayInfo {
            name: o.name,
            make: o.make,
            model: o.model,
            serial: o.serial,
            active: o.active,

            position: (o.rect.x, o.rect.y).into(),
            size: (o.rect.width as u32, o.rect.height as u32),
            scale: o.scale.unwrap_or(1.).into(),
            transform: match o.transform {
            	Some(t) => match t.as_str() {
            		"90" => Transform::R90,
	            	"180" => Transform::R180,
	            	"270" => Transform::R270,
	            	_ => Transform::None,
            	},
            	_ => Transform::None,
            },

            id: o.id,
            focused: false,
        }
    }
}

#[derive(Clone, Default, Data, Lens)]
pub struct AppData {
    pub display_geo: HashMap<String, DisplayInfo>,
}

impl From<Vec<Output>> for AppData {
    fn from(outputs: Vec<Output>) -> Self {
        let mut display_geo = HashMap::new();
        for o in outputs {
            display_geo.insert(o.name.clone(), o.into());
        }

        AppData { display_geo }
    }
}

impl AppData {
	pub fn apply_displays(&mut self) {
		let mut conn = swayipc::Connection::new().unwrap();
		let mut outputs = conn.get_outputs().unwrap();
		for o in outputs {
			if let Some(our) = self.display_geo.get(&o.name) {
				let live = o.into();
				if !our.same(&live) {
					let mut cmd = String::with_capacity(200);
					cmd.push_str("output ");
					cmd.push_str(&live.name);

					if !our.scale.same(&live.scale) {
						cmd.push_str(" scale ");
						cmd.push_str(format!("{:.2}", our.scale.0).as_str());
					}

					if !our.position.same(&live.position) {
						cmd.push_str(" pos ");
						cmd.push_str(format!("{} {}", our.position.0, our.position.1).as_str());
					}

					if !our.transform.same(&live.transform) {
						cmd.push_str(" transform ");
						cmd.push_str(format!("{}", our.transform).as_str());
					}

					println!("command = {:?}", cmd);
					conn.run_command(cmd);
				}
			}
		}


		// Update ourselves based on the new reality of things
		outputs = conn.get_outputs().unwrap();
		*self = outputs.into();
	}
}


#[derive(Clone, Default, Debug)]
pub struct FocusedDisplay;

impl Lens<AppData, Option<DisplayInfo>> for FocusedDisplay {
    fn with<V, F: FnOnce(&Option<DisplayInfo>) -> V>(&self, data: &AppData, f: F) -> V {
        for (_, v) in data.display_geo.iter() {
            if v.focused {
                return f(&Some(v.clone()));
            }
        }

        f(&None)
    }

    fn with_mut<V, F: FnOnce(&mut Option<DisplayInfo>) -> V>(&self, data: &mut AppData, f: F) -> V {
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
