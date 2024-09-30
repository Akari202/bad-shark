use std::clone;

#[derive(Debug, Clone, Copy)]
pub struct Distance {
    pub value: f64,
    pub unit: DistanceUnit
}

#[derive(Debug, Clone, Copy)]
pub enum DistanceUnit {
    Meters,
    Decimeters,
    Centimeters,
    Millimeters,
    Inches,
    Feet,
    Yards
}

impl Distance {
    pub fn new(value: f64, unit: DistanceUnit) -> Distance {
        Distance { value, unit }
    }

    pub fn from_meters(value: f64) -> Distance {
        Distance::new(value, DistanceUnit::Meters)
    }

    pub fn from_decimeters(value: f64) -> Distance {
        Distance::new(value, DistanceUnit::Decimeters)
    }

    pub fn from_centimeters(value: f64) -> Distance {
        Distance::new(value, DistanceUnit::Centimeters)
    }

    pub fn from_millimeters(value: f64) -> Distance {
        Distance::new(value, DistanceUnit::Millimeters)
    }

    pub fn from_inches(value: f64) -> Distance {
        Distance::new(value, DistanceUnit::Inches)
    }

    pub fn from_feet(value: f64) -> Distance {
        Distance::new(value, DistanceUnit::Feet)
    }

    pub fn from_yards(value: f64) -> Distance {
        Distance::new(value, DistanceUnit::Yards)
    }

    pub fn to_meters(&self) -> Distance {
        match self.unit {
            DistanceUnit::Meters => self.clone(),
            DistanceUnit::Decimeters => Distance::from_meters(self.value / 10.0),
            DistanceUnit::Centimeters => Distance::from_meters(self.value / 100.0),
            DistanceUnit::Millimeters => Distance::from_meters(self.value / 1000.0),
            DistanceUnit::Inches => Distance::from_meters(self.value * 0.0254),
            DistanceUnit::Feet => Distance::from_meters(self.value * 0.3048),
            DistanceUnit::Yards => Distance::from_meters(self.value * 0.9144)
        }
    }

    pub fn to_decimeters(&self) -> Distance {
        match self.unit {
            DistanceUnit::Meters => Distance::from_decimeters(self.value * 10.0),
            DistanceUnit::Decimeters => self.clone(),
            DistanceUnit::Centimeters => Distance::from_decimeters(self.value / 10.0),
            DistanceUnit::Millimeters => Distance::from_decimeters(self.value / 100.0),
            DistanceUnit::Inches => Distance::from_decimeters(self.value * 0.254),
            DistanceUnit::Feet => Distance::from_decimeters(self.value * 3.048),
            DistanceUnit::Yards => Distance::from_decimeters(self.value * 9.144)
        }
    }

    pub fn to_centimeters(&self) -> Distance {
        match self.unit {
            DistanceUnit::Meters => Distance::from_centimeters(self.value * 100.0),
            DistanceUnit::Decimeters => Distance::from_centimeters(self.value * 10.0),
            DistanceUnit::Centimeters => self.clone(),
            DistanceUnit::Millimeters => Distance::from_centimeters(self.value / 10.0),
            DistanceUnit::Inches => Distance::from_centimeters(self.value * 2.54),
            DistanceUnit::Feet => Distance::from_centimeters(self.value * 30.48),
            DistanceUnit::Yards => Distance::from_centimeters(self.value * 91.44)
        }
    }

    pub fn to_millimeters(&self) -> Distance {
        match self.unit {
            DistanceUnit::Meters => Distance::from_millimeters(self.value * 1000.0),
            DistanceUnit::Decimeters => Distance::from_millimeters(self.value * 100.0),
            DistanceUnit::Centimeters => Distance::from_millimeters(self.value * 10.0),
            DistanceUnit::Millimeters => self.clone(),
            DistanceUnit::Inches => Distance::from_millimeters(self.value * 25.4),
            DistanceUnit::Feet => Distance::from_millimeters(self.value * 304.8),
            DistanceUnit::Yards => Distance::from_millimeters(self.value * 914.4)
        }
    }

    pub fn to_inches(&self) -> Distance {
        match self.unit {
            DistanceUnit::Meters => Distance::from_inches(self.value / 0.0254),
            DistanceUnit::Decimeters => Distance::from_inches(self.value / 0.254),
            DistanceUnit::Centimeters => Distance::from_inches(self.value / 2.54),
            DistanceUnit::Millimeters => Distance::from_inches(self.value / 25.4),
            DistanceUnit::Inches => self.clone(),
            DistanceUnit::Feet => Distance::from_inches(self.value * 12.0),
            DistanceUnit::Yards => Distance::from_inches(self.value * 36.0)
        }
    }

    pub fn to_feet(&self) -> Distance {
        match self.unit {
            DistanceUnit::Meters => Distance::from_feet(self.value / 0.3048),
            DistanceUnit::Decimeters => Distance::from_feet(self.value / 3.048),
            DistanceUnit::Centimeters => Distance::from_feet(self.value / 30.48),
            DistanceUnit::Millimeters => Distance::from_feet(self.value / 304.8),
            DistanceUnit::Inches => Distance::from_feet(self.value / 12.0),
            DistanceUnit::Feet => self.clone(),
            DistanceUnit::Yards => Distance::from_feet(self.value * 3.0)
        }
    }

    pub fn to_yards(&self) -> Distance {
        match self.unit {
            DistanceUnit::Meters => Distance::from_yards(self.value / 0.9144),
            DistanceUnit::Decimeters => Distance::from_yards(self.value / 9.144),
            DistanceUnit::Centimeters => Distance::from_yards(self.value / 91.44),
            DistanceUnit::Millimeters => Distance::from_yards(self.value / 914.4),
            DistanceUnit::Inches => Distance::from_yards(self.value / 36.0),
            DistanceUnit::Feet => Distance::from_yards(self.value / 3.0),
            DistanceUnit::Yards => self.clone()
        }
    }
}

impl std::fmt::Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(precision) = f.precision() {
            write!(f, "{:.precision$} in", self.to_inches().value, precision = precision)
        } else {
            write!(f, "{} in", self.to_inches().value)
        }
        // write!(f, "{:.3} in", self.to_inches().value)
        // match self.unit {
        //     DistanceUnit::Meters => write!(f, "{} m", self.value),
        //     DistanceUnit::Millimeters => write!(f, "{} mm", self.value),
        //     DistanceUnit::Inches => write!(f, "{} in", self.value)
        // }
    }
}

